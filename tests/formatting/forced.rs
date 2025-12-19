use human_format::*;

#[test]
fn forced_suffix_scaling_examples() {
    let mut f = Formatter::new();

    f.with_suffix("M");

    assert_eq!(f.format(100000.0), "0.10 M");
}

#[test]
fn forced_suffix_unknown_and_extremes_examples() {
    let mut f = Formatter::new();
    let mut f2 = Formatter::new();

    f.with_suffix("DN");
    f2.with_suffix("Q");

    let s = f.format(1000.0);

    assert!(!s.is_empty());
    assert!(f2.format(1e3).starts_with("0."));
}
