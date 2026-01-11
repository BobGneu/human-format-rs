use human_format::*;

#[test]
fn binary_and_units_examples() {
    assert_eq!(
        Formatter::new()
            .with_scales(Scales::Binary())
            .format(1024.0),
        "1.00 ki"
    );
    assert_eq!(
        Formatter::new()
            .with_scales(Scales::Binary())
            .with_units("B")
            .format(102400.0),
        "100.00 KiB"
    );
    assert_eq!(
        Formatter::new()
            .with_scales(Scales::Binary())
            .with_units("B")
            .format(1024.0 * 1024.0),
        "1.00 MiB"
    );
}
