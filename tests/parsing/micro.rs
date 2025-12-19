use human_format::*;

#[test]
fn micro_sign_input_is_accepted() {
    let f = Formatter::new();

    // micro sign input should be accepted and treated as 'u'
    assert_eq!(f.try_parse("1.0 µ").unwrap(), 1e-6);
}
