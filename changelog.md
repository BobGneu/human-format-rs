# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [1.2.1] - 2026-01-16

### Fixed

- Binary suffix ki should be Kis - [PR#34](https://github.com/BobGneu/human-format-rs/pull/34) by [ip1981](https://github.com/ip1981)

## [1.2.0] - 2025-12-27

### Added

- New tests for cases out in the wild
- `try_parse` which returns `Result<f64, ParseError>` instead of panicking
- `ParseError` enum with variants for `EmptyInput`, `InvalidNumber`, and `UnknownSuffix`
- `parse_or_clamp` convenience method to optionally clamp unknown suffixes to the largest multiplier (now respects `explicit_map` when present)
- Doctests and README snippets demonstrating `try_parse`, binary scales, units and negative numbers
- Support for the newest SI prefixes: `R`/`Q` (ronna/quetta) and `r`/`q` (ronto/quecto)
- Optional micro sign formatting and parsing: accept `µ` and output `µ` when enabled via `Formatter::with_micro_sign(true)`
- Forced suffix formatting: `Formatter::with_suffix("M")` will scale output to the requested suffix when possible (e.g., `100000 -> 0.10 M`)
- New `Scales::Time()` with explicit time unit multipliers (`ns`, `us`, `ms`, `s`, `m`, `h`, `d`, `w`, `mo`, `qtr`, `y`, `dec`, `c`, `kyr`, `Myr`, `Gyr`, and aliases)

### Changed

- A few tweaks to our build flow to run clippy, and make sure to gate building based on prior dependent actions
    - removed verbose flag as it does not appear to be necessary any longer
- Renamed `master` branch to `main`
- lower casing si suffixes - [PR#27](https://github.com/BobGneu/human-format-rs/pull/27) by [jdrouet](https://github.com/jdrouet)
- Replaced silent clamping/0.0 multiplier lookup with explicit `try_get_magnitude_multiplier` returning error on unknown suffix
- Refactored parsing internals to centralize numeric/suffix extraction and reduce duplication
- Added edge-case tests (empty input, trailing garbage, comma-decimal behavior, NaN/Infinity, rounding boundaries, and very large magnitudes)

### Removed

- Removed `galvanic-test` as a dependency

## [1.1.0] - 2024-02-16

### Changed

- Format check included in build
- Improve error handling in try_parse with better ergonomics - [PR#19](https://github.com/BobGneu/human-format-rs/pull/19) by [@jgrund](https://github.com/jgrund)

### Removed

- removed Travis & Appveyor

## [1.0.3] - 2019-11-23

### Fixed

- Removed unnecessary logging - [PR#9](https://github.com/BobGneu/human-format-rs/pull/9) by [@jaysonsantos](https://github.com/jaysonsantos)
- Corrected binary base to 1024

## [1.0.2] - 2018-02-01

### Fixed

- Corrected issue with API, expecting owned strings when the common occurrence will be references.

## [1.0.1] - 2018-01-28

### Added

- Updated Documentation to improve utility of [docs.rs](https://docs.rs/crate/human_format/)
- Added fmt to build scripts

## [1.0.0] - 2018-01-28

Initial Release

[unreleased]: https://github.com/BobGneu/human-format-rs/compare/1.2.1...develop
[1.2.1]: https://github.com/BobGneu/human-format-rs/compare/1.2.0...1.2.1
[1.2.0]: https://github.com/BobGneu/human-format-rs/compare/1.1.0...1.2.0
[1.1.0]: https://github.com/BobGneu/human-format-rs/compare/1.0.3...1.1.0
[1.0.3]: https://github.com/BobGneu/human-format-rs/compare/1.0.2...1.0.3
[1.0.2]: https://github.com/BobGneu/human-format-rs/compare/1.0.1...1.0.2
[1.0.1]: https://github.com/BobGneu/human-format-rs/compare/1.0.0...1.0.1
[1.0.0]: https://github.com/BobGneu/human-format-rs/tree/1.0.0
