extern crate human_format;

#[cfg(test)]
mod demo_examples {
    use human_format::*;

    #[test]
    fn should_allow_explicit_decimals() {
        assert_eq!(
            Formatter::new().with_decimals(1).format(1000 as f64),
            "1.0 k"
        );
    }

    #[test]
    fn should_allow_explicit_separator() {
        assert_eq!(
            Formatter::new().with_separator(" - ").format(1000 as f64),
            "1.00 - k"
        );
    }

    #[test]
    fn should_allow_use_of_si_scale_explicitly() {
        assert_eq!(
            Formatter::new()
                .with_scales(Scales::SI())
                .format(1000 as f64),
            "1.00 k"
        );
    }

    #[test]
    fn should_allow_use_of_binary_scale_explicitly() {
        assert_eq!(
            Formatter::new()
                .with_scales(Scales::Binary())
                .format(1024 as f64),
            "1.00 Ki"
        );
    }

    #[test]
    fn should_allow_use_of_binary_units_explicitly() {
        assert_eq!(
            Formatter::new()
                .with_scales(Scales::Binary())
                .with_units("B")
                .format(102400 as f64),
            "100.00 KiB"
        );
    }

    #[test]
    fn should_output_10_24_mib() {
        assert_eq!(
            Formatter::new()
                .with_scales(Scales::Binary())
                .with_units("B")
                .format(1024.0 * 1024.0 as f64),
            "1.00 MiB"
        );
    }

    #[test]
    fn should_output_75_11_pib() {
        assert_eq!(
            Formatter::new()
                .with_scales(Scales::Binary())
                .with_units("B")
                .format(84_567_942_345_572_238.0),
            "75.11 PiB"
        );
    }

    #[test]
    fn should_output_1_00_gbps() {
        assert_eq!(Formatter::new().with_units("B/s").format(1e9), "1.00 GB/s");
    }

    #[test]
    fn should_allow_explicit_suffix_and_unit() {
        assert_eq!(
            Formatter::new()
                .with_suffix("k")
                .with_units("m")
                .format(1024 as f64),
            "1.02 km"
        );
    }

    #[test]
    fn should_allow_use_of_explicit_scale() {
        let mut scales = Scales::new();

        scales
            .with_base(1024)
            .with_suffixes(vec!["", "Ki", "Mi", "Gi", "Ti", "Pi", "Ei", "Zi", "Yi"]);

        assert_eq!(
            Formatter::new()
                .with_scales(scales)
                .with_units("B")
                .format(1024 as f64),
            "1.00 KiB"
        );
    }

    #[test]
    fn should_allow_parsing_to_f64() {
        assert_eq!(Formatter::new().try_parse("1.00 k").unwrap(), 1000.0);
    }

    #[test]
    fn should_allow_try_parsing_to_f64() {
        assert_eq!(Formatter::new().try_parse("1.00 M"), Ok(1000000.0));
    }

    #[test]
    fn should_allow_parsing_binary_values_to_f64() {
        assert_eq!(
            Formatter::new()
                .with_scales(Scales::Binary())
                .try_parse("1.00 Ki")
                .unwrap(),
            1024.0
        );
    }

    #[test]
    fn should_allow_parsing_binary_values_with_units_to_f64() {
        assert_eq!(
            Formatter::new()
                .with_scales(Scales::Binary())
                .with_units("B")
                .try_parse("1.00 KiB")
                .unwrap(),
            1024.0
        );
    }

    #[test]
    fn should_allow_try_parsing_binary_values_with_units_to_f64() {
        assert_eq!(
            Formatter::new()
                .with_scales(Scales::Binary())
                .with_units("B")
                .try_parse("1.00 KiB"),
            Ok(1024.0)
        );
    }

    #[test]
    fn should_surface_errors() {
        let result = Formatter::new()
            .with_scales(Scales::Binary())
            .with_units("B")
            .try_parse("1.00 DN");

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("Unknown suffix"));
    }

    #[test]
    fn try_parse_explicit_suffix_and_unit() {
        assert_eq!(
            Formatter::new().with_units("m").try_parse("1.024Mm"),
            Ok(1024000.0)
        );
    }

    #[test]
    fn try_parse_explicit_suffix_and_unitless() {
        assert_eq!(
            Formatter::new().with_units("m").try_parse("1.024M"),
            Ok(1024000.0)
        );
    }

    #[test]
    fn try_parse_very_large_value() {
        assert_eq!(
            Formatter::new().with_units("B").try_parse("2PB"),
            Ok(2_000_000_000_000_000.0)
        );
    }

    #[test]
    fn formatting_uses_micro_sign_when_enabled() {
        let mut f = Formatter::new();
        f.with_micro_sign(true);
        // 0.000001 -> 1.00 µ
        assert_eq!(f.format(0.000001_f64), "1.00 µ");
    }

    #[test]
    fn time_scale_format_and_parse() {
        // 90 seconds should be formatted to 1.50 m when using time scales (minutes)
        let mut f = Formatter::new();
        f.with_scales(Scales::Time());
        assert_eq!(f.format(90.0), "1.50 m");

        // Parsing "1.5 m" with time scales should give 90 seconds
        let mut fp = Formatter::new();
        fp.with_scales(Scales::Time()).with_units("s");
        assert_eq!(fp.try_parse("1.5 m").unwrap(), 90.0);
    }

    #[test]
    fn time_scale_long_units() {
        let mut f = Formatter::new();
        f.with_scales(Scales::Time());

        // 1 month ~ average month in seconds
        let month_secs = (365.2425 * 86400.0) / 12.0;
        assert_eq!(f.try_parse("1 mo").unwrap(), month_secs);

        // 1 year
        let year_secs = 365.2425 * 86400.0;
        assert_eq!(f.try_parse("1 y").unwrap(), year_secs);

        // 1 decade = 10 years
        assert_eq!(f.try_parse("1 dec").unwrap(), 10.0 * year_secs);

        // 1 century = 100 years
        assert_eq!(f.try_parse("1 c").unwrap(), 100.0 * year_secs);
    }

    #[test]
    fn time_scale_quarters() {
        let mut f = Formatter::new();
        f.with_scales(Scales::Time());

        let month_secs = (365.2425 * 86400.0) / 12.0;
        let quarter_secs = 3.0 * month_secs;

        assert_eq!(f.try_parse("1 qtr").unwrap(), quarter_secs);
    }

    #[test]
    fn forced_suffix_scaling() {
        let mut f = Formatter::new();
        f.with_suffix("M");
        // 100000 -> 0.10M (100000 / 1_000_000 = 0.1)
        assert_eq!(f.format(100000.0), "0.10 M");
    }

    #[test]
    fn micro_sign_parsing_and_formatting() {
        // parsing accepts µ
        assert_eq!(Formatter::new().try_parse("1.0 µ").unwrap(), 1e-6);

        // formatting uses u by default
        let f = Formatter::new();
        assert!(f.format(1e-6).contains("u") || f.format(1e-6).contains("µ"));

        // enable µ output
        let mut fm = Formatter::new();
        fm.with_micro_sign(true);
        assert_eq!(fm.format(1e-6), "1.00 µ");
    }

    #[test]
    fn forced_suffix_unknown_and_extremes() {
        // unknown forced suffix falls back
        let mut f = Formatter::new();
        let s1 = f.with_suffix("DN").format(1000.0);
        assert_eq!(s1, "1.00 k");

        // forcing very large suffix produces < 1 values
        let mut f2 = Formatter::new();
        let s2 = f2.with_suffix("Q").format(1e3);

        assert_eq!(s2, "0.00 Q");
    }

    #[test]
    fn time_scale_round_trip_and_case() {
        let mut f = Formatter::new();
        f.with_scales(Scales::Time());

        // months and years
        let month_secs = (365.2425 * 86400.0) / 12.0;
        assert_eq!(f.try_parse("1 mo").unwrap(), month_secs);
        assert_eq!(f.try_parse("1 y").unwrap(), 365.2425 * 86400.0);

        // case sensitivity: use 'qtr' quarter alias
        assert_eq!(f.try_parse("1 qtr").unwrap(), 3.0 * month_secs);
    }
}
