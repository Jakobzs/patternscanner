//! This crate provides a simple API for searching for a pattern in an array of bytes.

/// Multithreaded pattern scanning
pub mod mt;

/// Singlethreaded pattern scanning
pub mod st;

/// Create a vector of bytes from a pattern string
///
/// # Arguments
/// * `pattern` - The pattern string
///
/// # Returns
/// * A vector of bytes
fn create_bytes_from_string(pattern: &str) -> Vec<Option<u8>> {
    // Create a Vec of Option<u8> where None represents a ? character in the pattern string
    pattern
        .split_whitespace()
        .map(|pattern_byte| {
            if pattern_byte == "?" {
                None
            } else {
                if pattern_byte.len() != 2 {
                    panic!("Invalid pattern byte: {}", pattern_byte);
                }

                Some(u8::from_str_radix(pattern_byte, 16).unwrap())
            }
        })
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // Test the create_bytes_from_string function with a valid string
    fn test_create_bytes_from_string_1() {
        assert_eq!(
            create_bytes_from_string("AA BB CC"),
            vec![Some(0xAA), Some(0xBB), Some(0xCC)]
        );
    }

    #[test]
    // Test the create_bytes_from_string function with a wildcard "?"
    fn test_create_bytes_from_string_wildcard() {
        assert_eq!(
            create_bytes_from_string("AA BB ? ? CC"),
            vec![Some(0xAA), Some(0xBB), None, None, Some(0xCC)]
        );
    }

    /*
    #[test]
    // Test the create_bytes_from_string function with an invalid byte "GG"
    fn test_create_bytes_from_string_error_invalid_byte() {
        assert_eq!(
            create_bytes_from_string("AA GG"),
            vec![Some(0xAA), Some(0xBB), None, None, Some(0xCC)]
        );
    }

    #[test]
    // Test the create_bytes_from_string function with a string that contains a space between the bytes
    fn test_create_bytes_from_string_error_space() {
        assert_eq!(
            create_bytes_from_string("A A BB"),
            vec![Some(0xAA), Some(0xBB), None, None, Some(0xCC)]
        );
    }
        */
}
