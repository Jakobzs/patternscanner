//! This crate provides a simple API for searching for a pattern in an array of bytes as either single-threaded or multi-threaded. It supports matching on either a single pattern or all possible patterns.

use core::num;
use rayon::{
    prelude::{IndexedParallelIterator, ParallelIterator},
    slice::ParallelSlice,
    ThreadPool, ThreadPoolBuilder,
};
use thiserror::Error;

/// Multithreaded pattern scanning
pub mod mt;

/// Singlethreaded pattern scanning
pub mod st;

pub struct PatternScanner {
    bytes: Vec<u8>,
    pattern: Vec<Option<u8>>,
    threadpool: ThreadPool,
}

impl PatternScanner {
    pub fn scan(&self) -> Result<Option<usize>, PatternScannerError> {
        // Scan the bytes for the unique pattern using the rayon crate
        Ok(self.threadpool.install(|| {
            self.bytes
                .par_windows(self.pattern.len())
                .position_any(|window| {
                    window
                        .iter()
                        .zip(self.pattern.iter())
                        .all(|(byte, pattern_byte)| {
                            pattern_byte.is_none() || Some(*byte) == *pattern_byte
                        })
                })
        }))
    }

    pub fn scan_all(&self) -> Result<Vec<usize>, PatternScannerError> {
        // Scan the bytes for all matches of the pattern using the rayon crate
        Ok(self
            .bytes
            .par_windows(self.pattern.len())
            .enumerate()
            .filter(|(_, window)| {
                window
                    .iter()
                    .zip(self.pattern.iter())
                    .all(|(byte, pattern_byte)| {
                        pattern_byte.is_none() || Some(*byte) == *pattern_byte
                    })
            })
            .map(|(i, _)| i)
            .collect())
    }
}

pub struct PatternScannerBuilder {
    bytes: Vec<u8>,
    pattern: Vec<Option<u8>>,
    threadpool_builder: ThreadPoolBuilder,
}

impl PatternScannerBuilder {
    pub fn builder() -> Self {
        Self {
            bytes: Vec::new(),
            pattern: Vec::new(),
            threadpool_builder: ThreadPoolBuilder::new(),
        }
    }

    pub fn with_bytes<T: AsRef<[u8]>>(mut self, bytes: T) -> Self {
        self.bytes = bytes.as_ref().to_vec();
        self
    }

    pub fn with_pattern<T: AsRef<str>>(mut self, pattern: T) -> Self {
        self.pattern = create_bytes_from_string(pattern).unwrap();
        self
    }

    pub fn with_threads(mut self, threads: usize) -> Self {
        self.threadpool_builder = self.threadpool_builder.num_threads(threads);
        self
    }

    pub fn build(self) -> PatternScanner {
        PatternScanner {
            bytes: self.bytes,
            pattern: self.pattern,
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
    let split_pattern = pattern.as_ref().split_whitespace();

    // Create a Vec of Option<u8> where None represents a ? character in the pattern string
    let mut v = Vec::new();
    for x in split_pattern {
        if x == "?" {
            v.push(None);
        } else {
            // Check that the pattern byte string is 2 characters long
            if x.len() != 2 {
                return Err(PatternScannerError::ByteLength(x.to_owned()));
            }

            v.push(Some(match u8::from_str_radix(x, 16) {
                Ok(b) => b,
                Err(e) => return Err(PatternScannerError::InvalidByte(e)),
            }));
        }
    }

    Ok(v)
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
            .with_bytes(&[0x00, 0x01, 0x02, 0x33, 0x35, 0x33, 0x35, 0x07, 0x08, 0x09])
            .with_pattern("33 35")
            .build()
            .scan()
            .unwrap();

        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_pattern_scan_all() {
        let result = PatternScannerBuilder::builder()
            .with_bytes(&[0x00, 0x01, 0x02, 0x33, 0x35, 0x33, 0x35, 0x07, 0x08, 0x09])
            .with_pattern("33 35")
            .build()
            .scan_all()
            .unwrap();

        assert_eq!(result, vec![3, 5]);
    }

    // Test scan_all with an array of 1 million bytes but in a random spot at say 600000 there is the pattern "33 35". The execution time is measured here
    #[test]
    fn test_pattern_scan_all_1_million_bytes() {
        let mut bytes = [0u8; 1_000_000];
        bytes[600_000] = 0x33;
        bytes[600_001] = 0x35;

        let scanner = PatternScannerBuilder::builder()
            .with_bytes(&bytes)
            .with_pattern("33 35")
            .with_threads(1)
            .build();
        // Start measuring the execution time
        let start = std::time::Instant::now();

        let result = scanner.scan_all().unwrap();

        // Stop measuring the execution time
        let duration = start.elapsed();
        println!("Time elapsed in expensive_function() is: {:?}", duration);

        assert_eq!(result, vec![600_000]);
    }
}
