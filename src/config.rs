//! Parse configuration module

/// Parse configuration
#[derive(Clone, Debug)]
pub struct ParseConfig {
    /// Headline's todo keywords, todo type
    pub todo_keywords: Vec<String>,
    /// Headline's todo keywords, done type
    pub done_keywords: Vec<String>,
}

impl Default for ParseConfig {
    fn default() -> Self {
        ParseConfig {
            todo_keywords: vec![String::from("TODO")],
            done_keywords: vec![String::from("DONE")],
        }
    }
}

lazy_static::lazy_static! {
    pub static ref DEFAULT_CONFIG: ParseConfig = ParseConfig::default();
}
