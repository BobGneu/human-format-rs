use human_format::*;

#[test]
fn parse_or_clamp_true_clamps_unknown_suffix() {
    let f = Formatter::new();

    let res = f.parse_or_clamp("1.0 DN", true).unwrap();

    assert!(res.is_finite());
}

#[test]
fn parse_or_clamp_clamps_to_largest_explicit_when_present() {
    let mut f = Formatter::new();

    f.with_scales(Scales::Time());

    // unknown suffix with clamp=true should clamp to the largest explicit multiplier (Gyr)
    let parsed = f.parse_or_clamp("1.0 unknown_suffix", true).unwrap();
    let year_secs = 365.2425 * 86400.0;
    let gyr = 1.0e9 * year_secs;

    assert_eq!(parsed, 1.0 * gyr);
}

#[test]
fn parse_or_clamp_false_errors_on_unknown_suffix() {
    let f = Formatter::new();

    let res = f.parse_or_clamp("1.0 DN", false);

    assert!(res.is_err());
}
