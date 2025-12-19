use human_format::*;

#[test]
fn very_large_magnitude_clamps_or_errors() {
    let f = Formatter::new();

    let formatted = f.format(1e300);

    assert!(!formatted.is_empty());
}
