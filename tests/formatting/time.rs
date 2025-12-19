use human_format::*;

#[test]
fn time_format_example() {
    let mut f = Formatter::new();

    f.with_scales(Scales::Time());

    assert_eq!(f.format(90.0), "1.50 m");
}
