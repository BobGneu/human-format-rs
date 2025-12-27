use human_format::*;

#[test]
fn parse_or_clamp_largest_suffix() {
    let f = Formatter::new();

    // unknown suffix errors when clamp == false
    assert!(f.parse_or_clamp("1.0 DN", false).is_err());

    // clamp to largest suffix (Q -> 10^30)
    assert_eq!(f.parse_or_clamp("1.0 DN", true).unwrap(), 1e30);
}
