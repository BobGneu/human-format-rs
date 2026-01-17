use human_format::{Formatter, Scales};

#[test]
fn forced_suffix_unknown_falls_back_to_auto() {
    let mut f = Formatter::new();

    f.with_suffix("nonexist");

    // Forcing an unknown suffix should fall back to automatic scaling
    let s = f.format(1000.0);
    // automatic SI scaling for 1000 is "k"
    assert!((s.contains("k") || s.contains("K")) && !s.contains(" ki"));
}

#[test]
fn forced_suffix_applies_multiplier_when_known() {
    let mut f = Formatter::new();

    f.with_scales(Scales::SI());
    f.with_suffix("k");

    // forcing 'k' should divide value by 1000
    let s = f.format(2000.0);
    assert!(s.contains("2.00") && s.contains("k"));
}

#[test]
fn forced_suffix_respects_units_and_micro_sign_normalization() {
    let mut f = Formatter::new();

    f.with_units("B");
    // set forced suffix to the micro-sign variant; format should accept it
    f.with_suffix("µ");

    let s = f.format(1.0e-6);
    // since suffix was forced to micro, output should include micro sign when enabled
    assert!(s.contains("µ") || s.contains("u"));
}
