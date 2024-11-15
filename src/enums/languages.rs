pub enum Language {
    Python,
}

impl Language {
    pub fn extension(&self) -> &'static str {
        match self {
            Language::Python => ".py",
        }
    }
}
