use human_format::*;

#[test]
fn micro_sign_formatting_example() {
    let mut f = Formatter::new();

    f.with_micro_sign(true);

    assert_eq!(f.format(0.000001_f64), "1.00 µ");
}
