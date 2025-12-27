use human_format::Formatter;

#[test]
fn parse_accepts_micro_sign_and_ascii_u() {
    let f = Formatter::new();

    // ASCII 'u' should parse as micro
    assert_eq!(f.try_parse("1.0 u").unwrap(), 1.0e-6);

    // Unicode micro sign 'µ' should also parse the same
    assert_eq!(f.try_parse("1.0 µ").unwrap(), 1.0e-6);
}

#[test]
fn format_has_optional_micro_sign_output() {
    let mut f = Formatter::new();
    f.with_suffix("u");

    // Default formatting uses ascii 'u' for micro
    let s = f.format(1.0e-6);
    assert!(s.contains("u"));

    // Enable micro sign output
    let mut f2 = Formatter::new();
    f2.with_suffix("u");
    f2.with_micro_sign(true);

    let s2 = f2.format(1.0e-6);
    assert!(s2.contains("µ"));
}
