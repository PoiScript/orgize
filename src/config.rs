use crate::syntax::document::document_node;
use crate::Org;

#[derive(Clone, Debug)]
pub enum UseSubSuperscript {
    Nil,
    Brace,
    True,
}

impl UseSubSuperscript {
    pub fn is_nil(&self) -> bool {
        matches!(self, UseSubSuperscript::Nil)
    }

    pub fn is_true(&self) -> bool {
        matches!(self, UseSubSuperscript::True)
    }

    pub fn is_brace(&self) -> bool {
        matches!(self, UseSubSuperscript::Brace)
    }
}

/// Parse configuration
#[derive(Clone, Debug)]
pub struct ParseConfig {
    /// Headline's todo keywords
    pub todo_keywords: (Vec<String>, Vec<String>),

    pub dual_keywords: Vec<String>,

    pub parsed_keywords: Vec<String>,

    /// Control sub/superscript parsing
    ///
    /// Equivalent to `org-use-sub-superscripts`
    ///
    /// - `UseSubSuperscript::Nil`: disable parsing
    /// - `UseSubSuperscript::True`: enable parsing
    /// - `UseSubSuperscript::Brace`: enable parsing, but braces are required
    pub use_sub_superscript: UseSubSuperscript,

    /// Affiliated keywords
    ///
    /// Equivalent to [`org-element-affiliated-keywords`](https://git.sr.ht/~bzg/org-mode/tree/6f960f3c6a4dfe137fbd33fef9f7dadfd229600c/item/lisp/org-element.el#L331)
    pub affiliated_keywords: Vec<String>,

    /// Control tag parsing
    ///
    /// Defaults to org-mode's permitted characters: alphanumeric and `_@#%`.
    pub is_tag_char: fn(char) -> bool,
}

impl ParseConfig {
    /// Parses input with current config
    pub fn parse(self, input: impl AsRef<str>) -> Org {
        let input = (input.as_ref(), &self).into();
        let node = document_node(input).unwrap().1;

        Org {
            config: self,
            green: node.into_node().unwrap(),
        }
    }
}

impl Default for ParseConfig {
    fn default() -> Self {
        ParseConfig {
            todo_keywords: (vec!["TODO".into()], vec!["DONE".into()]),
            dual_keywords: vec!["CAPTION".into(), "RESULTS".into()],
            parsed_keywords: vec!["CAPTION".into()],
            use_sub_superscript: UseSubSuperscript::True,
            affiliated_keywords: vec![
                "CAPTION".into(),
                "DATA".into(),
                "HEADER".into(),
                "HEADERS".into(),
                "LABEL".into(),
                "NAME".into(),
                "PLOT".into(),
                "RESNAME".into(),
                "RESULT".into(),
                "RESULTS".into(),
                "SOURCE".into(),
                "SRCNAME".into(),
                "TBLNAME".into(),
            ],
            is_tag_char: |c| c.is_alphanumeric() || c == '_' || c == '@' || c == '#' || c == '%',
        }
    }
}
