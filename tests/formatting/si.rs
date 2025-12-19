use human_format::*;

#[test]
fn should_allow_use_of_si_scale_implicitly() {
    assert_eq!(
        Formatter::new().with_suffix("k").format(1000 as f64),
        "1.00 k"
    );
}

#[test]
fn should_handle_negative_values() {
    assert_eq!(
        Formatter::new().with_suffix("k").format(-1000 as f64),
        "-1.00 k"
    );
}

#[test]
fn should_handle_large_numbers_in_scientific_notation() {
    assert_eq!(Formatter::new().format(1.2123123422324232e16), "12.12 P");
    assert_eq!(Formatter::new().format(1.2123123422324232e18), "1.21 E");
    assert_eq!(Formatter::new().format(1.2123123422324232e26), "121.23 Y");
    assert_eq!(Formatter::new().format(5.58559632792669e27), "5.59 R");
    assert_eq!(Formatter::new().format(5.58559632792669e30), "5.59 Q");
    assert_eq!(Formatter::new().format(5.58559632792669e31), "55.86 Q");
    assert_eq!(Formatter::new().format(5.58559632792669e35), "558559.63 Q");
}
