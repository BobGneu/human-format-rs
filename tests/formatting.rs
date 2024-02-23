#[macro_use]
extern crate galvanic_test;
extern crate human_format;

test_suite! {
    name formatting;
    use human_format::*;

    test should_allow_use_of_si_scale_implicitly() {
        assert_eq!(Formatter::new()
            .with_suffix("K")
            .format(1000 as f64),
            "1.00 K");
    }

    test should_handle_large_numbers_in_scientific_notation() {
        assert_eq!(Formatter::new().format(1.2123123422324232e16), "12.12 P");

        assert_eq!(Formatter::new().format(1.2123123422324232e18), "1.21 E");

        assert_eq!(Formatter::new().format(1.2123123422324232e26), "121.23 Y");
    }
}
