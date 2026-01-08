#![doc(html_root_url = "https://docs.rs/human_format")]

//! `human_format` is a library for formatting numbers into human readable strings. It supports SI, Time and Binary scales out of the box, and allows for custom scales as well.
//!
//! ## Setup
//!
//! Add the library to your dependencies listing
//!
//! ```bash
//! $ cargo add human_format
//! ```
//!
//! ## Usage
//!
//! Print human readable strings from numbers using SI scales by default
//!
//! ```rust
//! // "1.00 k"
//! let tmpStr = human_format::Formatter::new()
//!     .format(1000.0);
//! # assert_eq!(tmpStr, "1.00 k");
//!
//! // "1.00 M"
//! let tmpStr2 = human_format::Formatter::new()
//!     .format(1000000.0);
//! # assert_eq!(tmpStr2, "1.00 M");
//!
//! // "1.00 G"
//! let tmpStr3 = human_format::Formatter::new()
//!     .format(1000000000.0);
//! # assert_eq!(tmpStr3, "1.00 G");
//! ```
//!
//! If you are so inspired you can even try playing with units and customizing your `Scales`
//!
//! For more examples you should review the examples on github: [tests/demo.rs](https://github.com/BobGneu/human-format-rs/blob/master/tests/demo.rs)
//!

#[derive(Debug)]
struct ScaledValue {
    value: f64,
    suffix: String,
}

/// Entry point to the library. Use this to handle your formatting needs.
#[derive(Debug)]
pub struct Formatter {
    decimals: usize,
    separator: String,
    scales: Scales,
    forced_units: String,
    forced_suffix: String,
    use_micro_sign: bool,
}

impl Default for Formatter {
    fn default() -> Self {
        Formatter {
            decimals: 2,
            separator: " ".to_owned(),
            scales: Scales::new(),
            forced_units: "".to_owned(),
            forced_suffix: "".to_owned(),
            use_micro_sign: false,
        }
    }
}

/// Provide a customized scaling scheme for your own modeling.
#[derive(Debug)]
pub struct Scales {
    base: u32,
    suffixes: Vec<String>,
    suffixes_neg: Vec<String>,
    explicit_map: Option<std::collections::HashMap<String, f64>>,
}

impl Formatter {
    /// Initializes a new `Formatter` with default values.
    pub fn new() -> Self {
        Default::default()
    }

    /// Sets the decimals value for formatting the string.
    pub fn with_decimals(&mut self, decimals: usize) -> &mut Self {
        self.decimals = decimals;

        self
    }

    /// Sets the separator value for formatting the string.
    pub fn with_separator(&mut self, separator: &str) -> &mut Self {
        self.separator = separator.to_owned();

        self
    }

    /// Sets the scales value.
    pub fn with_scales(&mut self, scales: Scales) -> &mut Self {
        self.scales = scales;

        self
    }

    /// Sets the units value.
    pub fn with_units(&mut self, units: &str) -> &mut Self {
        self.forced_units = units.to_owned();

        self
    }

    /// Sets the expected suffix value.
    pub fn with_suffix(&mut self, suffix: &str) -> &mut Self {
        self.forced_suffix = suffix.to_owned();

        self
    }

    /// Enable using the micro sign `µ` in formatted output when fractional suffix is `u`.
    pub fn with_micro_sign(&mut self, enable: bool) -> &mut Self {
        self.use_micro_sign = enable;

        self
    }

    /// Formats the number into a string
    pub fn format(&self, value: f64) -> String {
        // Handle non-finite values explicitly to avoid loops in scaling logic
        if value.is_nan() {
            return "NaN".to_owned();
        }

        if value < 0.0 {
            return format!("-{}", self.format(-value));
        }

        if value.is_infinite() {
            return "inf".to_owned();
        }

        // If a forced suffix is provided, attempt to scale to that suffix
        let scaled_value = if !self.forced_suffix.is_empty() {
            // normalize micro sign in forced suffix when looking up
            let lookup = self.forced_suffix.replace('\u{00B5}', "u");
            match self.scales.try_get_magnitude_multiplier(&lookup) {
                Ok(mult) => ScaledValue {
                    value: value / mult,
                    suffix: self.forced_suffix.clone(),
                },
                Err(_) => self.scales.to_scaled_value(value),
            }
        } else {
            self.scales.to_scaled_value(value)
        };

        let out_suffix = if self.use_micro_sign && scaled_value.suffix == "u" {
            "µ".to_owned()
        } else {
            scaled_value.suffix.clone()
        };

        format!(
            "{:.width$}{}{}{}",
            scaled_value.value,
            self.separator,
            out_suffix,
            self.forced_units,
            width = self.decimals
        )
    }

    /// Parse a string back into a float value.
    ///
    /// This convenience wrapper unwraps the result of `try_parse` and will panic
    /// on malformed input. It is feature-gated behind `panic_parse` so that
    /// consumers must opt-in to the panicking API via the panic_parse feature.
    #[cfg(feature = "panic_parse")]
    #[deprecated(
        note = "Use `try_parse`, which returns `Result<f64, ParseError>` and does not panic on malformed input"
    )]
    pub fn parse(&self, value: &str) -> f64 {
        self.try_parse(value).unwrap()
    }

    /// Attempt to parse a string back into a float value.
    ///
    /// Examples:
    ///
    /// ```rust
    /// use human_format::{Formatter, Scales};
    /// // SI example
    /// let f = Formatter::new();
    /// assert_eq!(f.try_parse("1.00 k").unwrap(), 1000.0);
    /// // Binary scales (Ki = 1024)
    /// let mut fbin = Formatter::new();
    /// fbin.with_scales(Scales::Binary());
    /// assert_eq!(fbin.try_parse("1.00 Ki").unwrap(), 1024.0);
    /// // Units specified via with_units() are automatically stripped from input
    /// let mut funit = Formatter::new();
    /// funit.with_units("B");
    /// assert_eq!(funit.try_parse("1.00 kB").unwrap(), 1000.0);
    /// // Negative numbers
    /// assert_eq!(Formatter::new().try_parse("-1.0 k").unwrap(), -1000.0);
    /// // Invalid input
    /// assert!(Formatter::new().try_parse("bad input").is_err());
    /// ```
    pub fn try_parse(&self, value: &str) -> Result<f64, ParseError> {
        let (number_str, suffix) = self.parse_components(value)?;
        let number = number_str
            .parse::<f64>()
            .map_err(ParseError::InvalidNumber)?;
        let magnitude_multiplier = self.scales.try_get_magnitude_multiplier(&suffix)?;

        Ok(number * magnitude_multiplier)
    }

    fn parse_components(&self, value: &str) -> Result<(String, String), ParseError> {
        // Remove forced units if present
        let value = value
            .trim()
            .trim_end_matches(&self.forced_units)
            .to_string();

        // Extract leading number (allow sign and decimal)
        let mut number = String::new();
        for (i, c) in value.chars().enumerate() {
            if c.is_ascii_digit() || c == '.' || (c == '-' && i == 0) {
                number.push(c);
            } else {
                break;
            }
        }

        if number.is_empty() {
            return Err(ParseError::EmptyInput);
        }

        let suffix = value
            .trim_start_matches(&number)
            .trim_start_matches(&self.separator)
            .to_string();

        // Normalize common variants: acceptance of micro sign 'µ'
        let suffix = suffix.replace('\u{00B5}', "u");

        Ok((number, suffix))
    }

    /// Parse a string and optionally clamp unknown suffixes to the largest suffix multiplier.
    ///
    /// If `clamp` is `false`, this behaves like `try_parse` and returns an error on unknown suffixes.
    /// If `clamp` is `true`, unknown suffixes will be interpreted as the largest available suffix.
    ///
    /// Examples:
    ///
    /// ```rust
    /// use human_format::{Formatter, Scales};
    /// let f = Formatter::new();
    /// // Unknown suffix errors when clamp == false
    /// assert!(f.parse_or_clamp("1.0 DN", false).is_err());
    /// // Unknown suffix clamps to largest suffix multiplier when clamp == true
    /// assert!(f.parse_or_clamp("1.0 DN", true).is_ok());
    /// // Binary example with units
    /// let mut fb = Formatter::new();
    /// fb.with_scales(Scales::Binary()).with_units("B");
    /// assert_eq!(fb.parse_or_clamp("1.0 KiB", false).unwrap(), 1024.0);
    /// // Negative number with clamp
    /// assert_eq!(Formatter::new().parse_or_clamp("-1.0 k", true).unwrap(), -1000.0);
    /// ```
    pub fn parse_or_clamp(&self, value: &str, clamp: bool) -> Result<f64, ParseError> {
        let (number_str, suffix) = self.parse_components(value)?;
        let number = number_str
            .parse::<f64>()
            .map_err(ParseError::InvalidNumber)?;

        match self.scales.try_get_magnitude_multiplier(&suffix) {
            Ok(mult) => Ok(number * mult),
            Err(ParseError::UnknownSuffix(_)) if clamp => {
                // If scales has an explicit_map (e.g., Time), clamp to the
                // largest explicit multiplier rather than assuming a power of
                // `base` matching the last suffix index.
                if let Some(map) = &self.scales.explicit_map
                    && !map.is_empty()
                {
                    let max_mult = map.values().copied().fold(f64::NEG_INFINITY, f64::max);
                    return Ok(number * max_mult);
                }

                let last_index = self.scales.suffixes.len().saturating_sub(1);
                let mult = (self.scales.base as f64).powi(last_index as i32);

                Ok(number * mult)
            }
            Err(e) => Err(e),
        }
    }
}

/// Errors returned by parsing operations.
#[derive(Debug)]
pub enum ParseError {
    EmptyInput,
    InvalidNumber(std::num::ParseFloatError),
    UnknownSuffix(String),
}

impl PartialEq for ParseError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (ParseError::EmptyInput, ParseError::EmptyInput) => true,
            // Note: `ParseFloatError` does not implement `PartialEq` on stable Rust.
            // We therefore treat all `InvalidNumber(_)` variants as equal to each other,
            // ignoring the inner `ParseFloatError`.
            (ParseError::InvalidNumber(_), ParseError::InvalidNumber(_)) => true,
            (ParseError::UnknownSuffix(a), ParseError::UnknownSuffix(b)) => a == b,
            _ => false,
        }
    }
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::EmptyInput => write!(f, "Empty input"),
            ParseError::InvalidNumber(e) => write!(f, "Invalid number: {}", e),
            ParseError::UnknownSuffix(s) => write!(f, "Unknown suffix: {}", s),
        }
    }
}

impl std::error::Error for ParseError {}

impl Default for Scales {
    fn default() -> Self {
        Scales::SI()
    }
}

impl Scales {
    /// Instantiates a new `Scales` with SI keys
    pub fn new() -> Self {
        Default::default()
    }

    /// Instantiates a new `Scales` with SI keys
    #[allow(non_snake_case)]
    pub fn SI() -> Self {
        Scales {
            base: 1000,
            suffixes: vec![
                "".to_owned(),
                "k".to_owned(),
                "M".to_owned(),
                "G".to_owned(),
                "T".to_owned(),
                "P".to_owned(),
                "E".to_owned(),
                "Z".to_owned(),
                "Y".to_owned(),
                "R".to_owned(),
                "Q".to_owned(),
            ],
            suffixes_neg: vec![
                "".to_owned(),
                "m".to_owned(), // milli
                "u".to_owned(), // micro (use 'u' ascii for micro)
                "n".to_owned(), // nano
                "p".to_owned(), // pico
                "f".to_owned(), // femto
                "a".to_owned(), // atto
                "z".to_owned(), // zepto
                "y".to_owned(), // yocto
                "r".to_owned(), // ronto (10^-27)
                "q".to_owned(), // quecto (10^-30)
            ],
            explicit_map: None,
        }
    }

    /// Instantiates a new `Scales` with Binary keys
    #[allow(non_snake_case)]
    pub fn Binary() -> Self {
        Scales {
            base: 1024,
            suffixes: vec![
                "".to_owned(),
                "Ki".to_owned(),
                "Mi".to_owned(),
                "Gi".to_owned(),
                "Ti".to_owned(),
                "Pi".to_owned(),
                "Ei".to_owned(),
                "Zi".to_owned(),
                "Yi".to_owned(),
                "Ri".to_owned(),
                "Qi".to_owned(),
            ],
            // binary scales usually don't define fractional SI-like prefixes; keep empty placeholder
            suffixes_neg: vec!["".to_owned()],
            explicit_map: None,
        }
    }

    /// Instantiates a new `Scales` for time units.
    ///
    /// This maps common time suffixes to multipliers in seconds.
    #[allow(non_snake_case)]
    pub fn Time() -> Self {
        use std::collections::HashMap;

        let mut map: HashMap<String, f64> = HashMap::new();
        map.insert("ns".to_owned(), 1e-9);
        map.insert("us".to_owned(), 1e-6);
        map.insert("ms".to_owned(), 1e-3);
        map.insert("s".to_owned(), 1.0);
        map.insert("m".to_owned(), 60.0);
        map.insert("h".to_owned(), 3600.0);
        map.insert("d".to_owned(), 86400.0);
        map.insert("w".to_owned(), 604800.0);

        // longer period units using average definitions
        let year_secs = 365.2425 * 86400.0; // average Gregorian year
        let month_secs = year_secs / 12.0; // average month

        map.insert("mo".to_owned(), month_secs);
        map.insert("month".to_owned(), month_secs);

        // quarters: three-month periods
        map.insert("qtr".to_owned(), 3.0 * month_secs);
        map.insert("y".to_owned(), year_secs);
        map.insert("yr".to_owned(), year_secs);
        map.insert("year".to_owned(), year_secs);

        map.insert("dec".to_owned(), 10.0 * year_secs);
        map.insert("decade".to_owned(), 10.0 * year_secs);

        map.insert("c".to_owned(), 100.0 * year_secs);
        map.insert("century".to_owned(), 100.0 * year_secs);

        map.insert("kyr".to_owned(), 1000.0 * year_secs); // millennium (kilo-year)
        map.insert("millennium".to_owned(), 1000.0 * year_secs);

        map.insert("Myr".to_owned(), 1.0e6 * year_secs);

        map.insert("Gyr".to_owned(), 1.0e9 * year_secs);

        Scales {
            base: 1,
            suffixes: vec![],
            suffixes_neg: vec![],
            explicit_map: Some(map),
        }
    }

    /// Sets the base for the `Scales`
    pub fn with_base(&mut self, base: u32) -> &mut Self {
        self.base = base;

        self
    }

    /// Sets the suffixes listing appropriately
    pub fn with_suffixes(&mut self, suffixes: Vec<&str>) -> &mut Self {
        self.suffixes = Vec::new();

        for suffix in suffixes {
            // This should be to_owned to be clear about intent.
            // https://users.rust-lang.org/t/to-string-vs-to-owned-for-string-literals/1441/6
            self.suffixes.push(suffix.to_owned());
        }

        self
    }

    fn try_get_magnitude_multiplier(&self, value: &str) -> Result<f64, ParseError> {
        // If an explicit mapping exists (e.g., time units), prefer it
        if let Some(map) = &self.explicit_map
            && let Some(val) = map.get(value)
        {
            return Ok(*val);
        }

        // positive suffixes
        if let Some((idx, _)) = self.suffixes.iter().enumerate().find(|(_, x)| x == &value) {
            return Ok((self.base as f64).powi(idx as i32));
        }

        // negative suffixes (fractions)
        if let Some((idx, _)) = self
            .suffixes_neg
            .iter()
            .enumerate()
            .find(|(_, x)| x == &value)
        {
            // idx 0 corresponds to base^0 (no scaling); idx 1 => base^-1, idx 2 => base^-2
            let exp = -(idx as i32);
            return Ok((self.base as f64).powi(exp));
        }

        // build valid suffix list for error message
        let mut valid: Vec<String> = Vec::new();
        if let Some(map) = &self.explicit_map {
            valid.extend(map.keys().cloned());
        }

        valid.extend(
            self.suffixes
                .iter()
                .filter(|x| !x.trim().is_empty())
                .cloned(),
        );

        valid.extend(
            self.suffixes_neg
                .iter()
                .filter(|x| !x.trim().is_empty())
                .cloned(),
        );

        Err(ParseError::UnknownSuffix(format!(
            "{}; valid suffixes are: {}",
            value,
            valid.join(", ")
        )))
    }

    fn to_scaled_value(&self, value: f64) -> ScaledValue {
        let mut index: usize = 0;
        let base: f64 = self.base as f64;
        let mut value = value;

        // Prevent infinite loops for non-finite values and cap index to available suffixes
        let last_index = self.suffixes.len().saturating_sub(1);
        let last_neg = self.suffixes_neg.len().saturating_sub(1);

        // If explicit map provided (e.g., Time), prefer selecting suffix from it
        if let Some(map) = &self.explicit_map {
            // Build vector of (suffix, multiplier) and sort descending by multiplier
            let mut entries: Vec<(String, f64)> =
                map.iter().map(|(k, v)| (k.clone(), *v)).collect();
            entries.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

            if value > 0.0 {
                for (suf, mult) in entries.iter() {
                    if value >= *mult {
                        return ScaledValue {
                            value: value / *mult,
                            suffix: suf.clone(),
                        };
                    }
                }

                // If smaller than smallest multiplier, use smallest (e.g., ns)
                if let Some((suf, mult)) = entries.last() {
                    return ScaledValue {
                        value: value / *mult,
                        suffix: suf.clone(),
                    };
                }
            }
        }

        if value >= base {
            while value >= base && index < last_index {
                value /= base;
                index += 1;
            }

            ScaledValue {
                value,
                suffix: self.suffixes[index].to_owned(),
            }
        } else if value > 0.0 && value < 1.0 {
            // Use negative prefixes for fractional values
            let mut neg_idx: usize = 0;
            while value < 1.0 && neg_idx < last_neg {
                value *= base;
                neg_idx += 1;
            }

            ScaledValue {
                value,
                suffix: self.suffixes_neg[neg_idx].to_owned(),
            }
        } else {
            ScaledValue {
                value,
                suffix: self.suffixes[0].to_owned(),
            }
        }
    }
}
