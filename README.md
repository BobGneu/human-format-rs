# human-format-rs

[![Crates.io](https://img.shields.io/crates/v/human_format.svg)](https://crates.io/crates/human_format) [![Documentation](https://img.shields.io/badge/docs-rs-red.svg)](https://docs.rs/human_format)


Rust Port of human-format from node, formatting numbers for us, while the machines are still at bay.

| Branch | Linux/MacOS | Windows |
| :-- | :--: | :--: |
| Master  | [![Build Status](https://travis-ci.org/BobGneu/human-format-rs.svg?branch=master)](https://travis-ci.org/BobGneu/human-format-rs)  |  [![Build status](https://ci.appveyor.com/api/projects/status/jarar20r4wucmmec/branch/master?svg=true)](https://ci.appveyor.com/project/BobGneu/human-format-rs/branch/master) |
| Develop | [![Build Status](https://travis-ci.org/BobGneu/human-format-rs.svg?branch=develop)](https://travis-ci.org/BobGneu/human-format-rs) | [![Build status](https://ci.appveyor.com/api/projects/status/jarar20r4wucmmec/branch/develop?svg=true)](https://ci.appveyor.com/project/BobGneu/human-format-rs/branch/develop) |

## What is human_format?

The primary purpose for this crate is to format numbers in a customizable fashion based around magnitudes. It is inspired by the [human-format](https://www.npmjs.com/package/human-format) package and the hope is to ultimately provide an idiomatic rust port. 

## Usage

1. Add this package as a dependency

```toml
[dependencies]
human_format = "1.0"
```

2. Add the crate reference at your crate root

```rust
extern crate human_format;
```

3. Print some human readable strings

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

For more examples please consult [tests/demo.rs](https://github.com/BobGneu/human-format-rs/blob/master/tests/demo.rs)

