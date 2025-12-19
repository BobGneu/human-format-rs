extern crate human_format;

#[cfg(test)]
mod parsing {
    use human_format::*;

    #[test]
    fn should_parse_1_0_g_as_1000000000() {
        let formatter = Formatter::new();
        assert_eq!(formatter.try_parse("1.0 G").unwrap(), 10.0_f64.powf(9.0));
    }

    #[test]
    fn should_parse_11248924551_k_as_1_1248924551e13() {
        let formatter = Formatter::new();
        assert_eq!(
            formatter.try_parse("11248924551 k").unwrap(),
            1.1248924551e13
        );
    }

    #[test]
    fn should_parse_55_86_q_as_5_586_e31() {
        let formatter = Formatter::new();
        assert_eq!(formatter.try_parse("55.86 Q").unwrap(), 5.586e31);
    }

    #[test]
    fn should_parse_558559_63_q_as_5_5855963e35() {
        let formatter = Formatter::new();
        assert_eq!(formatter.try_parse("558559.63 Q").unwrap(), 5.5855963e35);
    }

    #[test]
    fn should_parse_1494_k_as_1494222() {
        let mut formatter = Formatter::new();
        formatter.with_decimals(3);

        assert_eq!(formatter.try_parse("1494 k").unwrap(), 1494000.0);
    }

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
        // micro sign input should be accepted and treated as 'u'
        assert_eq!(f.try_parse("1.0 µ").unwrap(), 1e-6);
    }
}
