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
fn round_trip_ronna_and_quetta() {
    let f = Formatter::new();
    // format and parse back large SI values
    let s = f.format(1e27);
    assert_eq!(f.try_parse(&s).unwrap(), 1e27);

    let s2 = f.format(1e30);
    assert_eq!(f.try_parse(&s2).unwrap(), 1e30);

    // and small-side prefixes
    let s3 = f.format(1e-27);
    assert_eq!(f.try_parse(&s3).unwrap(), 1e-27);

    let s4 = f.format(1e-30);
    assert_eq!(f.try_parse(&s4).unwrap(), 1e-30);
}
