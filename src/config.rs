//! Parse configuration module

/// Parse configuration
#[derive(Clone, Debug)]
pub struct ParseConfig<'a> {
    /// Default headline todo keywords, it shouldn't be changed.
    pub default_todo_keywords: &'a [&'a str],
    /// Custom headline todo keywords
    pub todo_keywords: &'a [&'a str],
}

impl Default for ParseConfig<'_> {
    fn default() -> Self {
        ParseConfig {
            default_todo_keywords: &["TODO", "DONE", "NEXT", "WAITING", "LATER", "CANCELLED"],
            todo_keywords: &[],
        }
    }
}
