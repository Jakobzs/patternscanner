//! This crate provides a simple API for searching for a pattern in an array of bytes as either single-threaded or multi-threaded. It supports matching on either a single pattern or all possible patterns.

use core::num;
use rayon::{
    prelude::{IndexedParallelIterator, ParallelIterator},
    slice::ParallelSlice,
    ThreadPool, ThreadPoolBuilder,
};
use thiserror::Error;

pub struct PatternScanner {
    bytes: Vec<u8>,
    threadpool: ThreadPool,
}

impl PatternScanner {
    /// Scan for a unique pattern in the stored bytes
    pub fn scan<T: AsRef<str>>(&self, pattern: T) -> Result<Option<usize>, PatternScannerError> {
        self.scan_with_bytes(&self.bytes, pattern)
    }

    /// Scan for a unique pattern in the specified bytes
    pub fn scan_with_bytes<T: AsRef<[u8]> + std::marker::Sync, U: AsRef<str>>(
        &self,
        bytes: T,
        pattern: U,
    ) -> Result<Option<usize>, PatternScannerError> {
        // Scan for all occurrences of the pattern in the bytes
        let results = self.scan_all_with_bytes(bytes, pattern)?;

        // Check if there are multiple occurrences of the pattern
        if results.len() > 1 {
            return Err(PatternScannerError::NonUniquePattern);
        }

        // Return the first (and only) result, if any
        Ok(results.first().copied())
    }

    /// Scan for all occurrences of a pattern in the stored bytes
    pub fn scan_all<T: AsRef<str>>(&self, pattern: T) -> Result<Vec<usize>, PatternScannerError> {
        self.scan_all_with_bytes(&self.bytes, pattern)
    }

    /// Scan for all occurrences of a pattern in the specified bytes
    pub fn scan_all_with_bytes<T: AsRef<[u8]> + std::marker::Sync, U: AsRef<str>>(
        &self,
        bytes: T,
        pattern: U,
    ) -> Result<Vec<usize>, PatternScannerError> {
        let pattern_bytes = create_bytes_from_string(pattern)?;

        // Scan the bytes for all matches of the pattern using the rayon crate
        Ok(self.threadpool.install(|| {
            bytes
                .as_ref()
                .par_windows(pattern_bytes.len())
                .enumerate()
                .filter(|(_, window)| {
                    window
                        .iter()
                        .zip(pattern_bytes.iter())
                        .all(|(byte, pattern_byte)| {
                            pattern_byte.is_none() || Some(*byte) == *pattern_byte
                        })
                })
                .map(|(i, _)| i)
                .collect()
        }))
    }
}

pub struct PatternScannerBuilder {
    bytes: Vec<u8>,
    threadpool_builder: ThreadPoolBuilder,
}

impl PatternScannerBuilder {
    ///  Create a new pattern scanner builder
    pub fn builder() -> Self {
        Self {
            bytes: Vec::new(),
            threadpool_builder: ThreadPoolBuilder::new(),
        }
    }

    /// Set the bytes to scan
    pub fn with_bytes<T: AsRef<[u8]>>(mut self, bytes: T) -> Self {
        self.bytes = bytes.as_ref().to_vec();
        self
    }

    /// Set the number of threads to use
    pub fn with_threads(mut self, threads: usize) -> Self {
        self.threadpool_builder = self.threadpool_builder.num_threads(threads);
        self
    }

    /// Build the pattern scanner
    pub fn build(self) -> PatternScanner {
        PatternScanner {
            bytes: self.bytes,
            threadpool: self
                .threadpool_builder
                .build()
                .expect("failed to build threadpool"),
        }
    }
}

#[derive(Error, Debug, PartialEq)]
// The error types for the pattern scanner
pub enum PatternScannerError {
    #[error("failed to parse the pattern byte {0} as a u8")]
    InvalidByte(#[from] num::ParseIntError),
    #[error("the pattern byte {0} is invalid (must be 2 characters long)")]
    ByteLength(String),
    //#[error("invalid header (expected {expected:?}, found {found:?})")]
    //InvalidHeader { expected: String, found: String },
    #[error("pattern is not unique")]
    NonUniquePattern,
    #[error("unknown pattern scanner error")]
    Unknown,
}

/// Create a vector of bytes from a pattern string
///
/// # Arguments
/// * `pattern` - The pattern string
///
/// # Returns
/// * A vector of bytes
fn create_bytes_from_string<T: AsRef<str>>(
    pattern: T,
) -> Result<Vec<Option<u8>>, PatternScannerError> {
    pattern
        .as_ref()
        .split_whitespace()
        .map(|x| {
            if x == "?" || x == "??" {
                Ok(None)
            } else {
                if x.len() != 2 {
                    return Err(PatternScannerError::ByteLength(x.to_owned()));
                }
                match u8::from_str_radix(x, 16) {
                    Ok(b) => Ok(Some(b)),
                    Err(e) => Err(PatternScannerError::InvalidByte(e)),
                }
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // Test the create_bytes_from_string function with a valid string
    fn test_create_bytes_from_string_1() {
        assert_eq!(
            create_bytes_from_string("AA BB CC").unwrap(),
            vec![Some(0xAA), Some(0xBB), Some(0xCC)]
        );
    }

    #[test]
    // Test the create_bytes_from_string function with a valid string
    fn test_create_bytes_from_string_2() {
        assert_eq!(
            create_bytes_from_string("AA BB CC AA BB FF").unwrap(),
            vec![
                Some(0xAA),
                Some(0xBB),
                Some(0xCC),
                Some(0xAA),
                Some(0xBB),
                Some(0xFF)
            ]
        );
    }

    #[test]
    // Test the create_bytes_from_string function with a wildcard "?"
    fn test_create_bytes_from_string_wildcard_1() {
        assert_eq!(
            create_bytes_from_string("AA BB ? ? CC").unwrap(),
            vec![Some(0xAA), Some(0xBB), None, None, Some(0xCC)]
        );
    }

    #[test]
    // Test the create_bytes_from_string function with a wildcard "?"
    fn test_create_bytes_from_string_wildcard_2() {
        assert_eq!(
            create_bytes_from_string("? AA BB ? ? CC ? ? ? FF").unwrap(),
            vec![
                None,
                Some(0xAA),
                Some(0xBB),
                None,
                None,
                Some(0xCC),
                None,
                None,
                None,
                Some(0xFF)
            ]
        );
    }

    #[test]
    // Test the create_bytes_from_string function with an invalid byte "GG"
    fn test_create_bytes_from_string_error_invalid_byte() {
        // There is currently no way to construct a ParseIntError so we can't test this yet, reference: https://stackoverflow.com/questions/55572098/how-to-construct-a-parseinterror-in-my-own-code
        assert!(create_bytes_from_string("AA GG").is_err());
    }

    #[test]
    // Test the create_bytes_from_string function with a string that contains a space between the bytes
    fn test_create_bytes_from_string_error_space() {
        assert_eq!(
            create_bytes_from_string("A A BB"),
            Err(PatternScannerError::ByteLength("A".to_owned()))
        );
    }

    #[test]
    fn test_pattern_scan() {
        let result = PatternScannerBuilder::builder()
            .with_bytes(&[0x00, 0x01, 0x02, 0x33, 0x35, 0x33, 0x36, 0x07, 0x08, 0x09])
            .build()
            .scan("33 35")
            .unwrap();

        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_pattern_scan_nonunique() {
        let result = PatternScannerBuilder::builder()
            .with_bytes(&[0x00, 0x01, 0x02, 0x33, 0x35, 0x33, 0x35, 0x07, 0x08, 0x09])
            .build()
            .scan("33 35");

        assert_eq!(result, Err(PatternScannerError::NonUniquePattern));
    }

    #[test]
    fn test_pattern_scan_all() {
        let result = PatternScannerBuilder::builder()
            .with_bytes(&[0x00, 0x01, 0x02, 0x33, 0x35, 0x33, 0x35, 0x07, 0x08, 0x09])
            .build()
            .scan_all("33 35")
            .unwrap();

        assert_eq!(result, vec![3, 5]);
    }

    // This test measures the execution time of the scan_all function with 1 million bytes and 1 thread
    #[test]
    fn test_pattern_scan_all_1_million_bytes() {
        // Create an array of 1 million bytes
        let mut bytes = [0u8; 1_000_000];
        bytes[600_000] = 0x33;
        bytes[600_001] = 0x35;

        // Create the pattern scanner
        let scanner = PatternScannerBuilder::builder()
            .with_bytes(bytes)
            .with_threads(1)
            .build();

        // Start measuring the execution time
        let start = std::time::Instant::now();

        // Scan the bytes
        let result = scanner.scan_all("33 35").unwrap();

        // Stop measuring the execution time
        let duration = start.elapsed();

        // Print the execution time
        println!("Execution time: {:?}", duration);

        assert_eq!(result, vec![600_000]);
    }
}
