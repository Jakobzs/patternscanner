use crate::{create_bytes_from_string, PatternScannerError};

/// Scan the bytes for a single match of the given pattern
///
/// # Arguments
/// * `bytes` - The bytes to scan
/// * `pattern` - The pattern to scan for
///
/// # Returns
/// * The index of the match
///
/// # Example
/// ```
/// use patternscanner::st::pattern_scan;
///
/// let result = pattern_scan(
///     &[0x00, 0x01, 0x02, 0x33, 0x35, 0x33, 0x35, 0x07, 0x08, 0x09],
///     "33 35",
/// ).unwrap();
///
/// assert_eq!(result, Some(3));
/// ```
/// # Panics
/// This function will panic if the pattern is invalid
///
/// # Performance
/// This function is slower than the multi-threaded version
///
/// # See also
/// * [pattern_scan](fn.pattern_scan.html)
pub fn pattern_scan(bytes: &[u8], pattern: &str) -> Result<Option<usize>, PatternScannerError> {
    // Convert the pattern string into a vector of bytes
    let pattern_bytes = create_bytes_from_string(pattern)?;

    // Scan the bytes for the unique pattern, with the help of .windows() to scan the bytes sequentially
    Ok(bytes.windows(pattern_bytes.len()).position(|window| {
        window
            .iter()
            .zip(pattern_bytes.iter())
            .all(|(byte, pattern_byte)| pattern_byte.is_none() || Some(*byte) == *pattern_byte)
    }))
}

/// Scan the bytes for all matches of the given pattern
///
/// # Arguments
/// * `bytes` - The bytes to scan
/// * `pattern` - The pattern to scan for
///
/// # Returns
/// * A vector of indices of the matches
///
/// # Example
/// ```
/// use patternscanner::st::pattern_scan_all;
///
/// let result = pattern_scan_all(
///     &[0x00, 0x01, 0x02, 0x33, 0x35, 0x33, 0x35, 0x07, 0x08, 0x09],
///     "33 35",
/// ).unwrap();
///
/// assert_eq!(result, [3, 5]);
/// ```
///
/// # Panics
/// This function will panic if the pattern is invalid
///
/// # Performance
/// This function is slower than the multi-threaded version
///
/// # See also
/// * [pattern_scan_single](fn.pattern_scan_single.html)
pub fn pattern_scan_all(bytes: &[u8], pattern: &str) -> Result<Vec<usize>, PatternScannerError> {
    // Convert the pattern string into a vector of bytes
    let pattern_bytes = create_bytes_from_string(pattern)?;

    // Scan the bytes for the pattern, with the help of .windows() to scan the bytes sequentially
    Ok(bytes
        .windows(pattern_bytes.len())
        .enumerate()
        .filter_map(|(i, window)| {
            if window
                .iter()
                .zip(pattern_bytes.iter())
                .all(|(byte, pattern_byte)| pattern_byte.is_none() || Some(*byte) == *pattern_byte)
            {
                Some(i)
            } else {
                None
            }
        })
        .collect())
}
