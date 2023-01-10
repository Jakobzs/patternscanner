# patternscanner

[![Build](https://github.com/Jakobzs/patternscanner/actions/workflows/rust.yml/badge.svg)](https://github.com/Jakobzs/patternscanner/actions/workflows/rust.yml)
[![API](https://docs.rs/patternscanner/badge.svg)](https://docs.rs/patternscanner)
[![Crate](https://img.shields.io/crates/v/patternscanner)](https://crates.io/crates/patternscanner)
[![dependency status](https://deps.rs/repo/github/jakobzs/patternscanner/status.svg)](https://deps.rs/repo/github/jakobzs/patternscanner)

A high performance pattern scanner for bytes.

This pattern scanner supports both single-threaded as well as multi-threaded scanning. Additionally, it is possible to include a wildcard `?` in the pattern.

## Installation

Add this crate as a dependency to your `Cargo.toml` file.

```toml
[dependencies]
patternscanner = "0.4.0"
```

## Example

```rust
// Use the multithreaded pattern scanners
use patternscanner::mt::{pattern_scan, pattern_scan_all};

// Scan for a single match of the pattern
let result = pattern_scan(
    &[0x00, 0x01, 0x02, 0x33, 0x35, 0x33, 0x42, 0x07, 0x08, 0x09],
    "33 35",
).unwrap();
assert_eq!(result, Some(3));

// Scan for a single match of the pattern with a wildcard
let result = pattern_scan(
    &[0x00, 0x01, 0x02, 0x33, 0x35, 0x42, 0x33, 0x35, 0x69, 0x09],
    "33 ? 42",
).unwrap();
assert_eq!(result, Some(3));

// Scan for all matches of the pattern with a wildcard
let result = pattern_scan_all(
    &[0x00, 0x01, 0x02, 0x33, 0x35, 0x42, 0x33, 0x35, 0x69, 0x09],
    "33 35 ?",
).unwrap();
assert_eq!(result, [3, 6]);
```

## License

[MIT](license-mit)

## Contributing

Contributions are welcome.
