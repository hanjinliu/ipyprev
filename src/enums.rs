pub enum Language {
    Python,
    Julia,
    Unknown,
}

impl Language {
    pub fn from_str(s: &str) -> Language {
        match s {
            "python" => Language::Python,
            "julia" => Language::Julia,
            _ => Language::Unknown,
        }
    }

    pub fn to_str(&self) -> &str {
        match self {
            Language::Python => "python",
            Language::Julia => "julia",
            Language::Unknown => "unknown",
        }
    }

    pub fn to_extension(&self) -> &str {
        match self {
            Language::Python => "py",
            Language::Julia => "jl",
            Language::Unknown => "",
        }
    }
}