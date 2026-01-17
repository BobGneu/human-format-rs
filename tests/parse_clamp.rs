use human_format::{Formatter, Scales};

#[test]
fn explicit_map_clamps_to_largest_multiplier() {
    let mut f = Formatter::new();
    f.with_scales(Scales::Time());

    // Choose a suffix that is unknown; with clamp=true this should clamp
    // to the largest explicit multiplier (Gyr -> 1e9 * year_secs)
    let parsed = f.parse_or_clamp("1.0 unknown_suffix", true).unwrap();

    // The largest multiplier in Time() explicit map is Gyr (1e9 * year_secs)
    let year_secs = 365.2425 * 86400.0;
    let gyr = 1.0e9 * year_secs;

    assert_eq!(parsed, 1.0 * gyr);
}
