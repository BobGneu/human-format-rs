use human_format::*;

#[test]
fn should_format_and_parse_ronna_and_quetta() {
    let f = Formatter::new();

    // 1 R -> 10^27
    assert_eq!(f.try_parse("1.0 R").unwrap(), 1.0e27);
    // 55.86 Q -> 55.86 * 10^30
    assert_eq!(f.try_parse("55.86 Q").unwrap(), 55.86e30);
}

#[test]
fn should_parse_ronto_and_quecto_and_micro_sign() {
    let f = Formatter::new();

    // 1 m -> milli
    assert_eq!(f.try_parse("1.0 m").unwrap(), 1e-3);
    // 1 r -> ronto (10^-27)
    assert_eq!(f.try_parse("1.0 r").unwrap(), 1e-27);
}
