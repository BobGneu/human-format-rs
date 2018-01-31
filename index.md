# Human Format

`human_format` provides facilitates creating a formatted string, converting between numbers that are beyond typical needs for humans into a simpler string that conveys the gist of the meaning of the number.

## Setup

Add the library to your dependencies listing

```toml
[dependencies]
human_format = "0.2"
```

Add the crate reference at your crate root

```rust
extern crate human_format;
```

Print some human readable strings

```rust
// "1.00 k"
let tmpStr = human_format::Formatter::new()
    .format(1000.0);

// "1.00 M"
let tmpStr2 = human_format::Formatter::new()
    .format(1000000.0);

// "1.00 B"
let tmpStr3 = human_format::Formatter::new()
    .format(1000000000.0);
```

If you are so inspired you can even try playing with units and customizing your Scales

For more examples you should review the examples on github: `tests/demo.rs`
