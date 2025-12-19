use human_format::*;

#[test]
fn rounding_boundaries() {
    let mut f = Formatter::new();

    f.with_decimals(2);
    let formatted = f.format(999.995);

    assert!(formatted == "1000.00 " || formatted.starts_with("1.00 "));
}
