use human_format::*;

#[test]
fn nan_and_infinity_formatting() {
    assert_eq!(Formatter::new().format(0.0 / 0.0), "NaN");
    assert_eq!(Formatter::new().format(f64::INFINITY), "inf");
    assert_eq!(Formatter::new().format(f64::NEG_INFINITY), "-inf");
}
