Copilot Instructions for human-format-rs

## Purpose

This document gives guidance for automated copilots and contributors working on the human-format-rs repository. Follow these rules to make consistent, tested, and minimally-invasive changes.

## Repository layout

-   Cargo.toml: Rust crate manifest. Keep `edition = '2021'` and `resolver = "2"` unless instructed otherwise.
-   src/lib.rs: Primary library implementation. Be conservative when editing; prefer minimal, focused patches and run tests after edits.
-   tests/: Integration tests. Tests are organized into subfolders by concern:
    -   tests/parsing/: parsing-related tests (SI, suffix parsing, micro sign, clamp behavior)
    -   tests/formatting/: formatting-related tests (SI output, binary, forced suffix, time formatting, micro sign output)
    -   tests/edge/: edge-case tests (parse errors, NaN/Inf, rounding, large-value handling, clamp behavior)

## Key conventions

-   API changes should be minimal and backwards-compatible where possible. When a breaking change is required, document it in CHANGELOG and README.
-   Prefer adding new small helper functions over extensive refactors unless requested.
-   Use `try_parse` for parsing and return Result; `parse()` is behind `panic_parse` feature.
-   `Scales::Time()` uses an explicit `HashMap` for unit multipliers; parsing and clamping should respect explicit_map first.

## Testing

-   Always run `cargo test` locally after making code or test changes; tests must pass.
-   New features need tests added to the appropriate `tests/*` folder. Keep tests focused and deterministic.
-   Avoid duplicating tests. If similar behavior should be validated from different angles (parse vs format), keep tests in separate logical folders.

## Documentation

-   Update README.md and changelog.md when behavior or public API changes.
-   Keep README examples brief and runnable; prefer `Formatter::new()` usage examples.

## Style and formatting

-   Follow existing code style in `src/lib.rs`. Keep changes small and consistent.
-   Avoid bulk reformatting of unrelated files.

## Safety and scope

-   Do not change tests or behavior that would invalidate user-facing assumptions without explicit approval.
-   When making non-trivial behavioral changes (e.g., clamping strategy), include tests and update README and changelog.

## PR checklist

-   Code compiles and `cargo test` passes.
-   Added or updated tests for new behaviors.
-   Updated `README.md` and `changelog.md` for user-facing changes.
-   Avoid changing unrelated files.

## Contact

If unsure about changing behavior in `Scales::Time()` or the parsing/clamping semantics, ask the project maintainer before making breaking changes.
