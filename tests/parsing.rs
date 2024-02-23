#[macro_use]
extern crate galvanic_test;
extern crate human_format;

test_suite! {
    name parsing;
    use human_format::*;

    test should_parse_1_0_g_as_1000000000() {
        let formatter = Formatter::new();
        assert_eq!(formatter.parse("1.0 G"), 10.0_f64.powf(9.0));
    }

    test should_parse_11248924551_k_as_1_1248924551e13() {
        let formatter = Formatter::new();
        assert_eq!(formatter.parse("11248924551 K"), 1.1248924551e13);
    }

    test should_parse_1494_k_as_1494222() {
        let mut formatter = Formatter::new();
        formatter.with_decimals(3);

        assert_eq!(formatter.parse("1494 K"), 1494000.0);
    }
}
