use std::fmt::Display;

/// Available languages
#[derive(PartialEq, serde::Deserialize, serde::Serialize)]
pub enum Language {
    /// Polish
    Pl,
    /// English
    En,
}

impl Language {
    /// Converts to language code used by locales
    pub fn string_code(&self) -> &str {
        match self {
            Language::Pl => "pl",
            Language::En => "en",
        }
    }
}

impl Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            Language::Pl => "Polski",
            Language::En => "English",
        };

        write!(f, "{}", name)
    }
}
