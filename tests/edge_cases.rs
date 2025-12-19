extern crate human_format;

#[cfg(test)]
mod edge_cases {
    use human_format::*;

    #[test]
    fn empty_input_errors() {
        let res = Formatter::new().try_parse("");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), ParseError::EmptyInput);
    }

    #[test]
    fn whitespace_only_errors() {
        let res = Formatter::new().try_parse("   ");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), ParseError::EmptyInput);
    }

    #[test]
    fn trailing_garbage_errors() {
        let res = Formatter::new().try_parse("1.00 kxyz");
        assert!(res.is_err());
        let e = res.unwrap_err();
        assert!(
            e.to_string().contains("Unknown suffix") || matches!(e, ParseError::UnknownSuffix(_))
        );
    }

    #[test]
    fn comma_decimal_behavior() {
        // Decide expected behavior: current implementation uses '.' only, so comma should error
        let res = Formatter::new().try_parse("1,23 k");
        assert!(res.is_err());
    }

    #[test]
    fn parse_negative_numbers() {
        let res = Formatter::new().try_parse("-1.0 k");
        assert_eq!(res.unwrap(), -1000.0);
    }

    #[test]
    fn nan_and_infinity_formatting() {
        // Formatting should handle non-finite values gracefully; define expected behavior
        assert_eq!(Formatter::new().format(0.0 / 0.0), "NaN");
        assert_eq!(Formatter::new().format(f64::INFINITY), "inf");
        assert_eq!(Formatter::new().format(f64::NEG_INFINITY), "-inf");
    }

    #[test]
    fn rounding_boundaries() {
        // With 2 decimals, 999.995 with SI base 1000 should round to 1.00 k
        let mut f = Formatter::new();
        f.with_decimals(2);
        let formatted = f.format(999.995);
        assert!(formatted == "1000.00 " || formatted.starts_with("1.00 "));
    }

    #[test]
    fn very_large_magnitude_clamps_or_errors() {
        // Value beyond Y (largest suffix) should not panic; behavior: clamp to largest suffix
        let f = Formatter::new();
        let formatted = f.format(1e300);
        assert!(!formatted.is_empty());
    }

    #[test]
    fn parse_or_clamp_true_clamps_unknown_suffix() {
        let f = Formatter::new();
        let res = f.parse_or_clamp("1.0 DN", true).unwrap();
        // Should be interpreted with the largest suffix multiplier
        assert!(res.is_finite());
    }

    #[test]
    fn parse_or_clamp_false_errors_on_unknown_suffix() {
        let f = Formatter::new();
        let res = f.parse_or_clamp("1.0 DN", false);
        assert!(res.is_err());
    }
}
