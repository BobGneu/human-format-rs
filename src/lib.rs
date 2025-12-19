#![doc(html_root_url = "https://docs.rs/human_format")]

//! `human_format` provides facilitates creating a formatted string, converting between numbers that are beyond typical
//! needs for humans into a simpler string that conveys the gist of the meaning of the number.
//!
//! ## Setup
//!
//! Add the library to your dependencies listing
//!
//! ```bash
//! $ cargo add human_format
//! ```
//!
//! Print some human readable strings
//!
//! ```rust
//! // "1.00 K"
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

/// Entry point to the lib. Use this to handle your formatting needs.
#[derive(Debug)]
pub struct Formatter {
    decimals: usize,
    separator: String,
    scales: Scales,
    forced_units: String,
    forced_suffix: String,
}

impl Default for Formatter {
    fn default() -> Self {
        Formatter {
            decimals: 2,
            separator: " ".to_owned(),
            scales: Scales::new(),
            forced_units: "".to_owned(),
            forced_suffix: "".to_owned(),
        }
    }
}

/// Provide a customized scaling scheme for your own modeling.
#[derive(Debug)]
pub struct Scales {
    base: u32,
    suffixes: Vec<String>,
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

    /// Formats the number into a string
    pub fn format(&self, value: f64) -> String {
        // Handle non-finite values explicitly to avoid loops in scaling logic
        if value.is_nan() {
            return "NaN".to_owned();
        }

        if value < 0.0 {
            return format!("-{}", self.format(value * -1.0));
        }

        if value.is_infinite() {
            return "inf".to_owned();
        }

        let scaled_value = self.scales.to_scaled_value(value);

        format!(
            "{:.width$}{}{}{}",
            scaled_value.value,
            self.separator,
            scaled_value.suffix,
            self.forced_units,
            width = self.decimals
        )
    }

    /// Parse a string back into a float value.
    #[deprecated(
        note = "Use `try_parse` which returns Result and does not panic on malformed input"
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
    /// // Binary scales (ki = 1024)
    /// let mut fbin = Formatter::new();
    /// fbin.with_scales(Scales::Binary());
    /// assert_eq!(fbin.try_parse("1.00 ki").unwrap(), 1024.0);
    /// // Units preserved in input are trimmed before parsing
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
        let value = value.trim_end_matches(&self.forced_units).to_string();

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

        Ok((number, suffix))
    }

    /// Parse a string and optionally clamp unknown suffixes to the largest suffix multiplier.
    ///
    /// If `clamp` is `false`, this behaves like `try_parse` and returns an error on unknown suffixes.
    /// If `clamp` is `true`, unknown suffixes will be interpreted as the largest available suffix.  ///
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
    /// assert_eq!(fb.parse_or_clamp("1.0 kiB", false).unwrap(), 1024.0);
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
                let last_index = self.scales.suffixes.len().saturating_sub(1);
                let mult = (self.scales.base as f64).powi(last_index as i32);
                Ok(number * mult)
            }
            Err(e) => Err(e),
        }
    }
}

/// Errors returned by parsing operations.
#[derive(Debug, PartialEq)]
pub enum ParseError {
    EmptyInput,
    InvalidNumber(std::num::ParseFloatError),
    UnknownSuffix(String),
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
        }
    }

    /// Instantiates a new `Scales` with Binary keys
    #[allow(non_snake_case)]
    pub fn Binary() -> Self {
        Scales {
            base: 1024,
            suffixes: vec![
                "".to_owned(),
                "ki".to_owned(),
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
        self.suffixes
            .iter()
            .enumerate()
            .find_map(|(idx, x)| {
                if value == x {
                    Some((self.base as f64).powi(idx as i32))
                } else {
                    None
                }
            })
            .ok_or_else(|| {
                ParseError::UnknownSuffix(format!(
                    "{}; valid suffixes are: {}",
                    value,
                    self.suffixes
                        .iter()
                        .filter(|x| !x.trim().is_empty())
                        .map(String::to_string)
                        .collect::<Vec<_>>()
                        .join(", ")
                ))
            })
    }

    fn to_scaled_value(&self, value: f64) -> ScaledValue {
        let mut index: usize = 0;
        let base: f64 = self.base as f64;
        let mut value = value;

        // Prevent infinite loops for non-finite values and cap index to available suffixes
        let last_index = self.suffixes.len().saturating_sub(1);
        while value >= base && index < last_index {
            value /= base;
            index += 1;
        }

        ScaledValue {
            value,
            suffix: self.suffixes[index].to_owned(),
        }
    }
}
