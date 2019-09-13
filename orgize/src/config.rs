//! Parse configuration module

/// Parse configuration
#[derive(Clone, Debug)]
pub struct ParseConfig {
    /// Headline's TODO keywords, todo type
    pub todo_keywords: Vec<String>,
    /// Headline's TODO keywords, done type
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
