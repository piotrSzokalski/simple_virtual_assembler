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
    pub fn string_code(&self) -> &str{
        match self {
            Language::Pl => "pl",
            Language::En => "en",
        }
    }
}