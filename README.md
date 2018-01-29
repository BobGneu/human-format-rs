# human-format-rs

Rust Port of human-format from node, formatting numbers for us, while the machines are still at bay.

| Branch | Linux/MacOS | Windows |
| :-- | :--: | :--: |
| Master  | [![Build Status](https://travis-ci.org/BobGneu/human-format-rs.svg?branch=master)](https://travis-ci.org/BobGneu/human-format-rs)  |  [![Build status](https://ci.appveyor.com/api/projects/status/jarar20r4wucmmec/branch/master?svg=true)](https://ci.appveyor.com/project/BobGneu/human-format-rs/branch/master) |
| Develop | [![Build Status](https://travis-ci.org/BobGneu/human-format-rs.svg?branch=develop)](https://travis-ci.org/BobGneu/human-format-rs) | [![Build status](https://ci.appveyor.com/api/projects/status/jarar20r4wucmmec/branch/develop?svg=true)](https://ci.appveyor.com/project/BobGneu/human-format-rs/branch/develop) |

## Usage

1. Add this package as a dependency

```toml
[dependencies]
human_format = "0.2"
```

2. Add the crate reference at your crate root

```rust
extern crate human_format;
```

3. Print some human readable strings

```rust 
println!("{}", Formatter::new().format(1000 as f64)); // outputs 1.00 k
println!("{}", Formatter::new().with_decimals(2).format(1337 as f64)); // outputs 1.34 k
println!("{}", Formatter::new().with_decimals(1).format(1337 as f64)); // outputs 1.3 k
```

For more examples please consult [tests/demo.rs](https://github.com/BobGneu/human-format-rs/blob/master/tests/demo.rs)

## What is human_format?

The primary purpose for this crate is to format numbers in a customizable fashion based around magnitudes. It is inspired by the [human-format](https://www.npmjs.com/package/human-format) package and the hope is to ultimately provide an idiomatic rust port. 
