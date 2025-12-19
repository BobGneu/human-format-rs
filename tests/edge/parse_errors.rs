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

    assert!(e.to_string().contains("Unknown suffix") || matches!(e, ParseError::UnknownSuffix(_)));
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

    assert!(res.is_ok());
    assert_eq!(res.unwrap(), -1000.0);
}
