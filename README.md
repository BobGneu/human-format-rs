# human_format

[![Crates.io](https://img.shields.io/crates/v/human_format.svg)](https://crates.io/crates/human_format) [![Documentation](https://img.shields.io/badge/docs-rs-red.svg)](https://docs.rs/human_format)

Rust Port of human-format from node, formatting numbers for us, while the machines are still at bay.

| Main                                                                                                                                                                          |                                                                                       Develop                                                                                       |
| :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------: |
| [![Main](https://github.com/BobGneu/human-format-rs/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/BobGneu/human-format-rs/actions/workflows/rust.yml) | [![Develop](https://github.com/BobGneu/human-format-rs/actions/workflows/rust.yml/badge.svg?branch=develop)](https://github.com/BobGneu/human-format-rs/actions/workflows/rust.yml) |

## What is human_format?

The primary purpose for this crate is to format numbers in a customizable fashion based around magnitudes. It is inspired by the [human-format](https://www.npmjs.com/package/human-format) package and the hope is to ultimately provide an idiomatic rust port.

## Usage

1. Add this package as a dependency - `cargo add human_format`
1. Print some human readable strings

## Examples

```rust
// 1.00 k
Formatter::new()
    .format(1000 as f64));

// 1.34 k
Formatter::new()
    .with_decimals(2)
    .format(1337 as f64);

// 1.3 k
Formatter::new()
    .with_decimals(1)
    .format(1337 as f64);

// 1.3B
Formatter::new()
    .with_decimals(1)
    .with_separator("")
    .format(1337000000 as f64);

// 1.00 - k
Formatter::new()
    .with_separator(" - ")
    .format(1000 as f64);


// Define your own scales as you see fit
let mut custom_binary_scales = Scales::new();

custom_binary_scales
    .with_base(1000)
    .with_suffixes(["".to_owned(),"k".to_owned(), "M".to_owned(), "G".to_owned(), "T".to_owned(), "P".to_owned(), "E".to_owned(), "Z".to_owned(), "Y".to_owned()].to_vec());

// 1.00 kB
Formatter::new()
    .with_scales(custom_binary_scales)
    .with_units("B")
    .format(1000 as f64);

// 1.00 kiB
Formatter::new()
    .with_scales(Scales::Binary())
    .with_units("B")
    .format(1000 as f64);
```

## Quick Parse Examples

Here are short examples demonstrating parsing and clamp behavior:

```rust
use human_format::{Formatter, Scales};

// SI parsing
let f = Formatter::new();
assert_eq!(f.try_parse("1.00 k").unwrap(), 1000.0);

// Binary parsing (ki = 1024)
let mut fbin = Formatter::new();
fbin.with_scales(Scales::Binary());
assert_eq!(fbin.try_parse("1.00 ki").unwrap(), 1024.0);

// Parsing with units trimmed
let mut funit = Formatter::new();
funit.with_units("B");
assert_eq!(funit.try_parse("1.00 kB").unwrap(), 1000.0);

// Negative numbers
assert_eq!(Formatter::new().try_parse("-1.0 k").unwrap(), -1000.0);

// parse_or_clamp: strict (errors)
assert!(Formatter::new().parse_or_clamp("1.0 DN", false).is_err());

// parse_or_clamp: clamp unknown suffix to largest multiplier
assert!(Formatter::new().parse_or_clamp("1.0 DN", true).is_ok());
```

For more examples please consult [tests/demo.rs](https://github.com/BobGneu/human-format-rs/blob/develop/tests/demo.rs)
