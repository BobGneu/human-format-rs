# human_format

[![Crates.io](https://img.shields.io/crates/v/human_format.svg)](https://crates.io/crates/human_format) [![Documentation](https://img.shields.io/badge/docs-rs-red.svg)](https://docs.rs/human_format)

Rust Port of human-format from node, formatting numbers for us, while the machines are still at bay.

| Main                                                                                                                                                                          |                                                                                       Develop                                                                                       |
| :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------: |
| [![Main](https://github.com/BobGneu/human-format-rs/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/BobGneu/human-format-rs/actions/workflows/rust.yml) | [![Develop](https://github.com/BobGneu/human-format-rs/actions/workflows/rust.yml/badge.svg?branch=develop)](https://github.com/BobGneu/human-format-rs/actions/workflows/rust.yml) |

`human_format` is a small Rust library that makes numbers easier for people to read. It converts large or small numbers into short strings, like turning `1_000_000` into "1.00 M".

## What it does

-   Formats numbers with familiar suffixes: `k`, `M`, `G`, and so on.
-   Parses human-friendly strings back into numbers with `try_parse`.
-   Supports the newest SI prefixes (`R`, `Q`, `r`, `q`).
-   Accepts and (optionally) outputs the micro sign `µ`.
-   Lets you force a specific suffix for output (for example, always show values in `M`).
-   Includes a `Scales::Time()` option for common time units (seconds, minutes, hours, days, months, years, quarters, centuries, etc.).

## Quick start

Add the crate:

```bash
cargo add human_format
```

Use the formatter:

```rust
use human_format::Formatter;

// Default SI formatting
let s = Formatter::new().format(1000.0);
assert_eq!(s, "1.00 k");

// Control decimals
let s = Formatter::new().with_decimals(1).format(1337.0);
assert_eq!(s, "1.3 k");
```

Use binary scales (base 1024):

```rust
use human_format::{Formatter, Scales};
let s = Formatter::new().with_scales(Scales::Binary()).format(1024.0);
assert_eq!(s, "1.00 ki");
```

## Parsing strings

Convert a human-friendly string back to a `f64` with `try_parse`.

```rust
use human_format::{Formatter, Scales};

let f = Formatter::new();

assert_eq!(f.try_parse("1.00 k").unwrap(), 1000.0);

let mut fb = Formatter::new();
fb.with_scales(Scales::Binary());

assert_eq!(fb.try_parse("1.00 ki").unwrap(), 1024.0);
```

The parser accepts the micro sign `µ` as input.

## Force a suffix in output

To force output to a certain suffix, use `with_suffix`. The value is scaled to match that suffix if possible.

```rust
use human_format::Formatter;

let mut f = Formatter::new();
f.with_suffix("M");

assert_eq!(f.format(100_000.0), "0.10 M");
```

If the suffix is not valid for the current `Scales`, the formatter falls back to automatic selection.

## Micro sign output

Use `with_micro_sign(true)` to show `µ` for micro values in the output.

```rust
use human_format::Formatter;

let mut f = Formatter::new();
f.with_micro_sign(true);

assert_eq!(f.format(0.000001_f64), "1.00 µ");
```

Parsing accepts both `u` and `µ`.

## Time scales

`Scales::Time()` uses a set of explicit unit multipliers for time. It uses average values where needed (for example, the average year is 365.2425 days). Use `Scales::Time()` when you want time-aware formatting and parsing.

```rust
use human_format::{Formatter, Scales};

let mut ft = Formatter::new();
ft.with_scales(Scales::Time());

// 90 seconds -> 1.50 m (minutes)
assert_eq!(ft.format(90.0), "1.50 m");
// Quarters (qtr) parse as three-month periods
assert!(ft.try_parse("1 qtr").is_ok());
```

For more examples please consult [tests/demo.rs](https://github.com/BobGneu/human-format-rs/blob/develop/tests/demo.rs)

## Notes

-   Months and years are approximate here. For precise calendar math, use a date-time library.
-   Very large and very small numbers can lose precision when using `f64`.
-   The `with_suffix` method uses the same suffix strings that `try_parse` accepts.

### Suffix vs unit ambiguity

Some short tokens can be ambiguous. For example, the single letter `m` can mean **milli** (a magnitude suffix) or **meter** (a measurement unit) depending on how you configure the `Formatter`:

-   `Formatter::new().with_suffix("m")` treats `m` as milli (scale).
-   `Formatter::new().with_units("m")` appends `m` as the units string (meter).

To avoid ambiguity, prefer longer suffixes or specify units explicitly with `with_units`.

### Case sensitivity

Suffix matching is case-sensitive: `M` (mega) is not the same as `m` (milli). If you want different aliases or case-insensitive parsing, add those aliases to the `Scales` you use.

## parse_or_clamp behavior

The `parse_or_clamp` helper can be used when accepting user-provided suffixes that may be unknown or misspelled. Call it with `clamp = true` to interpret unknown suffixes as the largest available magnitude in the active `Scales`.

Important: when a `Scales` provides an `explicit_map` (for example `Scales::Time()`), `parse_or_clamp` will clamp to the largest multiplier defined in that explicit map rather than assuming a power-of-base value. This ensures consistent behavior for non-power-of-base scales such as time units (where the largest defined unit is `Gyr`).

## Testing and correctness notes

We added a number of focused tests to ensure robust handling of edge-cases:

-   Micro sign parsing and optional micro-sign output (tests/micro_sign.rs).
-   Forced-suffix behavior including unknown-suffix fallbacks (tests/forced_suffix.rs).
-   SI round-trip formatting for new prefixes `R`/`Q` and parsing validation (tests/si_roundtrip.rs).
-   Time scale edge cases: months, quarters, parsing case-sensitivity, and clamp behavior (tests/time_edgecases.rs).

If you rely on case-insensitive parsing or custom aliases, construct a `Scales` with the aliases you need and pass it to `Formatter::with_scales`.

## Contributing

Contributions are welcome.

When you add features, please include tests.

## License

See the repository `LICENSE` file for license details.
