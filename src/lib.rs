//! This crate provides a simple API for searching for a pattern in an array of bytes as either single-threaded or multi-threaded. It supports matching on either a single pattern or all possible patterns.

use core::num;
use thiserror::Error;

/// Multithreaded pattern scanning
pub mod mt;

/// Singlethreaded pattern scanning
pub mod st;

#[derive(Error, Debug)]
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
fn create_bytes_from_string(pattern: &str) -> Result<Vec<Option<u8>>, PatternScannerError> {
    let split_pattern = pattern.split_whitespace();

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
    fn test_create_bytes_from_string_wildcard() {
        assert_eq!(
            create_bytes_from_string("AA BB ? ? CC").unwrap(),
            vec![Some(0xAA), Some(0xBB), None, None, Some(0xCC)]
        );
    }

    #[test]
    // Test the create_bytes_from_string function with an invalid byte "GG"
    fn test_create_bytes_from_string_error_invalid_byte() {
        assert!(create_bytes_from_string("AA GG").is_err());
    }

    #[test]
    // Test the create_bytes_from_string function with a string that contains a space between the bytes
    fn test_create_bytes_from_string_error_space() {
        assert!(create_bytes_from_string("A A BB").is_err());
    }
}
