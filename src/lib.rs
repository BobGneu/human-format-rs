
#[derive(Debug, Clone)]
pub struct Formatter  {
    decimals: u32,
    separator: String,
    scales: Scales,
    forced_units: String,
    forced_suffix: String
}

#[derive(Debug, Clone)]
pub struct Scales {
    base: u32,
    initialMagnitude: u32,
    suffixes: Vec<String>
}

impl Formatter {
    pub fn new() -> Self {
        Formatter {
            decimals: 2,
            separator: " ".to_owned(),
            scales: Scales::SI(),
            forced_units: "".to_owned(),
            forced_suffix: "".to_owned()
        }
    }

    pub fn with_decimals(&self, decimals: u32) -> &Self {
        self
    }

    pub fn with_separator(&self, separator: &str) -> &Self {
        self
    }

    pub fn with_scales(&self, scales: &Scales) -> &Self {
        self
    }

    pub fn with_units(&self, units: &str) -> &Self {
        self
    }

    pub fn with_suffix(&self, suffix: &str) -> &Self {
        self
    }

    pub fn format(&self, value: f64) -> String {
        "".to_owned()
    }

    pub fn parse(&self, value: &str) -> f64 {
        0.0
    }
}

impl Scales {
    pub fn new() -> Self {
        Scales::SI()
    }

    pub fn SI() -> Self {
        Scales {
            base: 1000,
            initialMagnitude: 1,
            suffixes: ["".to_owned(),"k".to_owned(), "M".to_owned(), "B".to_owned(), "T".to_owned(), "P".to_owned(), "E".to_owned(), "Z".to_owned(), "Y".to_owned()].to_vec()
        }
    }

    pub fn Binary() -> Self {
        Scales {
            base: 0x1000,
            initialMagnitude: 1,
            suffixes: ["".to_owned(),"ki".to_owned(), "Mi".to_owned(), "Gi".to_owned(), "Ti".to_owned(), "Pi".to_owned(), "Ei".to_owned(), "Zi".to_owned(), "Yi".to_owned()].to_vec()
        }
    }

    pub fn with_base(&self, base: u32) -> &Self {
        self
    }

    pub fn with_suffixes(&self, suffixes: Vec<String>, initialMagnitude: i32) -> &Self {
        self
    } 
 }