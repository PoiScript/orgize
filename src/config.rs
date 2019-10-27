/// Parse configuration
#[derive(Clone, Debug)]
pub struct ParseConfig {
    /// Headline's todo keywords
    pub todo_keywords: (Vec<String>, Vec<String>),
}

impl Default for ParseConfig {
    fn default() -> Self {
        ParseConfig {
            todo_keywords: (vec![String::from("TODO")], vec![String::from("DONE")]),
        }
    }
}

lazy_static::lazy_static! {
    pub static ref DEFAULT_CONFIG: ParseConfig = ParseConfig::default();
}
