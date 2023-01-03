use rayon::{prelude::IndexedParallelIterator, slice::ParallelSlice};

// Function that takes a series of bytes and a string pattern of bytes such as "BA AB AB FF CC DD ? ? BB"
// and scans the series of bytes for the pattern. If the pattern is found, return the index of the first byte of the pattern.
// If the pattern is not found, return None. The ? character in the pattern matches any byte.
fn scan_bytes(bytes: &[u8], pattern: &str, multithreaded: bool) -> Option<usize> {
    // Create a Vec of Option<u8> where None represents a ? character in the pattern string
    let pattern_bytes = pattern
        .split_whitespace()
        .map(|pattern_byte| {
            if pattern_byte == "?" {
                None
            } else {
                Some(u8::from_str_radix(pattern_byte, 16).unwrap())
            }
        })
        .collect::<Vec<_>>();

    // Scan the bytes for the pattern using the rayon crate if multithreaded is true and single threaded if false
    match multithreaded {
        // Use the .par_windows() method from the rayon crate to scan the bytes in parallel
        true => bytes
            .par_windows(pattern_bytes.len())
            .position_any(|window| {
                window
                    .iter()
                    .zip(pattern_bytes.iter())
                    .all(|(byte, pattern_byte)| {
                        pattern_byte.is_none() || Some(*byte) == *pattern_byte
                    })
            }),
        // Use the .windows() method to scan the bytes sequentially
        false => bytes.windows(pattern_bytes.len()).position(|window| {
            window
                .iter()
                .zip(pattern_bytes.iter())
                .all(|(byte, pattern_byte)| pattern_byte.is_none() || Some(*byte) == *pattern_byte)
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = scan_bytes(
            &[
                0x55, 0x66, 0x99, 0xBA, 0xAB, 0xAB, 0xFF, 0xCC, 0xDD, 0xEE, 0xFF, 0xBB,
            ],
            "BA AB AB FF CC DD ? ? BB",
            true,
        )
        .unwrap();
        assert_eq!(result, 3);
    }
}
