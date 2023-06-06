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
patternscanner = "0.5.0"
```

## Example

```rust
use patternscanner::PatternScannerBuilder;

fn main() {
    let result = PatternScannerBuilder::builder()
        .with_bytes(&[0x00, 0x01, 0x02, 0x33, 0x35, 0x33, 0x35, 0x07, 0x08, 0x09])
        .build()
        .scan_all("33 35")
        .unwrap();

    assert_eq!(result, vec![3, 5]);
}
```

## License

[MIT](license-mit)

## Contributing

Contributions are welcome.
