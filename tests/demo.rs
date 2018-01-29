#[macro_use]
extern crate galvanic_test;
extern crate human_format;

test_suite! {
    name demo_examples;
    use human_format::*;

    test should_allow_use_of_si_scale_implicitly() {
        assert_eq!(Formatter::new()
            .format(1000 as f64),
            "1.00 k");
    }

    test should_allow_explicit_decimals() {
        assert_eq!(Formatter::new()
            .with_decimals(1)
            .format(1000 as f64),
            "1.0 k");
    }

    test should_allow_explicit_separator() {
        assert_eq!(Formatter::new()
            .with_separator(" - ")
            .format(1000 as f64),
            "1.00 - k");
    }

    test should_allow_use_of_si_scale_explicitly() {
        assert_eq!(Formatter::new()
            .with_scales(Scales::SI())
            .format(1000 as f64),
            "1.00 k");
    }

    test should_allow_use_of_binary_scale_explicitly() {
        assert_eq!(Formatter::new()
            .with_scales(Scales::Binary())
            .format(1024 as f64),
            "1.00 ki");
    }

    test should_allow_use_of_binary_units_explicitly() {
        assert_eq!(Formatter::new()
            .with_scales(Scales::Binary())
            .with_units("B")
            .format(1024 as f64),
            "1.00 kiB");
    }

    test should_allow_explicit_suffix_and_unit() {
        assert_eq!(Formatter::new()
            .with_suffix("k")
            .with_units("m")
            .format(1024 as f64),
            "1.02 km");
    }

    test should_allow_use_of_explicit_scale() {
        let mut scales = Scales::new();

        scales
            .with_base(1024)
            .with_suffixes(["".to_owned(),"ki".to_owned(), "Mi".to_owned(), "Gi".to_owned(), "Ti".to_owned(), "Pi".to_owned(), "Ei".to_owned(), "Zi".to_owned(), "Yi".to_owned()].to_vec());

        assert_eq!(Formatter::new()
            .with_scales(scales)
            .with_units("B")
            .format(1024 as f64),
            "1.00 kiB");
    }

    test should_allow_parsing_to_f64() {
        assert_eq!(Formatter::new()
            .parse("1.00 k"), 1000.0);
    }

    test should_allow_parsing_binary_values_to_f64() {
        assert_eq!(Formatter::new()
            .with_scales(Scales::Binary())
            .parse("1.00 ki"), 1024.0);
    }

    test should_allow_parsing_binary_values_with_units_to_f64() {
        assert_eq!(Formatter::new()
            .with_scales(Scales::Binary())
            .with_units("B")
            .parse("1.00 kiB"), 1024.0);
    }
}
