#![doc(html_root_url = "https://docs.rs/human_format")]

//! `human_format` provides facilitates creating a formatted string, converting between numbers that are beyond typical
//! needs for humans into a simpler string that conveys the gist of the meaning of the number.
//!
//! ## Setup
//!
//! 1. Add this library to your dependencies listing
//!
//! ```toml
//! [dependencies]
//! human_format = "0.2"
//! ```
//!
//! 2. Add the crate reference at your crate root
//!
//! ```rust
//! extern crate human_format;
//! ```
//!
//! 3. Print some human readable strings
//!
//! ```rust
//! // 1.00 k
//! let tmpStr = human_format::Formatter::new()
//!     .format(1000.0);
//! # assert_eq!(tmpStr, "1.00 k");
//! let tmpStr2 = human_format::Formatter::new()
//!     .format(1000000.0);
//! # assert_eq!(tmpStr2, "1.00 M");
//! let tmpStr3 = human_format::Formatter::new()
//!     .format(1000000000.0);
//! # assert_eq!(tmpStr3, "1.00 B");
//! ```
//!

#[derive(Debug)]
struct ScaledValue {
    value: f32,
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

/// Provide a customized scaling scheme for your own modeling.
#[derive(Debug)]
pub struct Scales {
    base: u32,
    suffixes: Vec<String>,
}

impl Formatter {
    /// Initializes a new `Formatter` with default values.
    pub fn new() -> Self {
        Formatter {
            decimals: 2,
            separator: " ".to_owned(),
            scales: Scales::SI(),
            forced_units: "".to_owned(),
            forced_suffix: "".to_owned(),
        }
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
        if value < 0.0 {
            return format!("-{}", self.format(value * -1.0));
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
    pub fn parse(&self, value: &str) -> f64 {
        let v: Vec<&str> = value.split(&self.separator).collect();

        let result = v.get(0).unwrap().parse::<f64>().unwrap();

        let mut suffix = v.get(1).unwrap().to_string();
        let new_len = suffix.len() - self.forced_units.len();

        suffix.truncate(new_len);

        let magnitude_multiplier = self.scales.get_magnitude_multipler(&suffix);

        (result * magnitude_multiplier)
    }
}

impl Scales {
    /// Instantiates a new `Scales` with SI keys
    pub fn new() -> Self {
        Scales::SI()
    }

    /// Instantiates a new `Scales` with SI keys
    pub fn SI() -> Self {
        Scales {
            base: 1000,
            suffixes: [
                "".to_owned(),
                "k".to_owned(),
                "M".to_owned(),
                "B".to_owned(),
                "T".to_owned(),
                "P".to_owned(),
                "E".to_owned(),
                "Z".to_owned(),
                "Y".to_owned(),
            ].to_vec(),
        }
    }

    /// Instantiates a new `Scales` with Binary keys
    pub fn Binary() -> Self {
        Scales {
            base: 1000,
            suffixes: [
                "".to_owned(),
                "ki".to_owned(),
                "Mi".to_owned(),
                "Gi".to_owned(),
                "Ti".to_owned(),
                "Pi".to_owned(),
                "Ei".to_owned(),
                "Zi".to_owned(),
                "Yi".to_owned(),
            ].to_vec(),
        }
    }

    /// Sets the base for the `Scales`
    pub fn with_base(&mut self, base: u32) -> &mut Self {
        self.base = base;

        self
    }

    /// Sets the suffixes listing appropriately
    pub fn with_suffixes(&mut self, suffixes: Vec<String>) -> &mut Self {
        self.suffixes = Vec::new();

        for suffix in suffixes {
            self.suffixes.push(suffix.to_owned());
        }

        self
    }

    fn get_magnitude_multipler(&self, value: &str) -> f64 {
        let ndx = 0;

        for ndx in 0..self.suffixes.len() {
            println!("{}", self.suffixes[ndx]);

            if value == self.suffixes[ndx] {
                return self.base.pow(ndx as u32) as f64;
            }
        }

        return 0.0;
    }

    fn to_scaled_value(&self, value: f64) -> ScaledValue {
        let mut index: usize = 0;
        let mut _value: f64 = value;

        loop {
            if _value < (self.base as f64) {
                break;
            }

            _value /= self.base as f64;
            index += 1;
        }

        println!(
            "\t\t{}: {} {} --- {}",
            value,
            index,
            self.base.pow(index as u32),
            value / (self.base.pow(index as u32) as f64)
        );

        ScaledValue {
            value: (value / self.base.pow((index) as u32) as f64) as f32,
            suffix: self.suffixes[index].to_owned(),
        }
    }
}
