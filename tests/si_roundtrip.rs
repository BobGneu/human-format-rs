use human_format::{Formatter, Scales};

#[test]
fn si_round_trip_new_prefixes() {
    let mut f = Formatter::new();
    f.with_scales(Scales::SI());

    // Test quetta (Q) and ronna (R)
    let q = f.format(1.0e30);
    assert!(q.contains("Q") || q.contains("quetta"));

    let r = f.format(1.0e27);
    assert!(r.contains("R") || r.contains("ronna"));

    // Round-trip via parse for these outputs should return approx original
    let parsed_q = f.try_parse(&q).unwrap();
    let parsed_r = f.try_parse(&r).unwrap();

    assert!((parsed_q / 1.0e30 - 1.0).abs() < 1e-12);
    assert!((parsed_r / 1.0e27 - 1.0).abs() < 1e-12);
}
