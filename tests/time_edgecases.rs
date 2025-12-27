use human_format::{Formatter, Scales};

#[test]
fn time_month_and_year_parsing_and_formatting() {
    let mut f = Formatter::new();
    f.with_scales(Scales::Time());

    // 1 month should be roughly year_secs/12 seconds
    let year_secs = 365.2425 * 86400.0;
    let month_secs = year_secs / 12.0;

    // Format 1 month in seconds should yield about "1.00 mo" when forced
    f.with_suffix("mo");
    let s = f.format(month_secs);
    assert!(s.contains("1.00") && s.contains("mo"));

    // Parsing month string should return month_secs
    let parsed = f.try_parse("1.0 mo").unwrap();
    assert!((parsed / month_secs - 1.0).abs() < 1e-12);
}

#[test]
fn quarters_are_three_months_and_parse() {
    let mut f = Formatter::new();
    f.with_scales(Scales::Time());

    // Quarter is defined as three months in the Time() mapping
    let year_secs = 365.2425 * 86400.0;
    let qtr_secs = 3.0 * (year_secs / 12.0);

    // Forced suffix qtr should yield 1.00 when formatting qtr_secs
    f.with_suffix("qtr");
    let s = f.format(qtr_secs);
    assert!(s.contains("1.00") && s.contains("qtr"));

    // Parsing "1 qtr" should return qtr_secs
    let parsed = f.try_parse("1 qtr").unwrap();
    assert!((parsed / qtr_secs - 1.0).abs() < 1e-12);
}

#[test]
fn time_parsing_is_case_sensitive_for_explicit_map() {
    // with Time scales 'mo' and 'month' are lower-case; 'Mo' should be unknown
    let mut ft = Formatter::new();
    ft.with_scales(Scales::Time());

    assert!(ft.try_parse("1 mo").is_ok());
    assert!(ft.try_parse("1 Mo").is_err());
}

#[test]
fn parse_or_clamp_uses_explicit_map_largest_for_time() {
    let mut f = Formatter::new();
    f.with_scales(Scales::Time());

    // Unknown suffix clamps to Gyr multiplier when clamp=true
    let parsed = f.parse_or_clamp("2.0 unknown", true).unwrap();
    let year_secs = 365.2425 * 86400.0;
    let gyr = 1.0e9 * year_secs;
    assert_eq!(parsed, 2.0 * gyr);
}
