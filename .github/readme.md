# patternscanner

[![Build](https://github.com/Jakobzs/patternscanner/actions/workflows/rust.yml/badge.svg)](https://github.com/Jakobzs/patternscanner/actions/workflows/rust.yml)
[![API](https://docs.rs/patternscanner/badge.svg)](https://docs.rs/patternscanner)
[![Crate](https://img.shields.io/crates/v/patternscanner)](https://crates.io/crates/patternscanner)
[![dependency status](https://deps.rs/repo/github/jakobzs/patternscanner/status.svg)](https://deps.rs/repo/github/jakobzs/patternscanner)

A pattern scanner for bytes.

## Installation

Add this crate as a dependency to your `Cargo.toml` file.

```toml
[dependencies]
patternscanner = "0.1.0"
```

## Example

```rust
use patternscanner::mt::pattern_scan;

let result = pattern_scan(
    &[0x00, 0x01, 0x02, 0x33, 0x35, 0x33, 0x35, 0x07, 0x08, 0x09],
    "33 35",
);
assert_eq!(result, [3, 5]);
```

## License

[MIT](license-mit)

## Contributing

Contributions are welcome.
