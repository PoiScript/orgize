use crate::SyntaxKind;

use super::{filter_token, Keyword, Token};

impl Keyword {
    ///
    /// ```rust
    /// use orgize::{Org, ast::Keyword};
    ///
    /// let keyword = Org::parse("#+KEY: VALUE\nabc").first_node::<Keyword>().unwrap();
    /// assert_eq!(keyword.key(), "KEY");
    /// ```
    pub fn key(&self) -> Token {
        self.syntax
            .children_with_tokens()
            .find_map(filter_token(SyntaxKind::TEXT))
            .unwrap_or_else(|| {
                debug_assert!(false, "keyword must contains TEXT");
                Token::default()
            })
    }

    ///
    /// ```rust
    /// use orgize::{Org, ast::Keyword};
    ///
    /// let keyword = Org::parse("#+KEY: VALUE\nabc").first_node::<Keyword>().unwrap();
    /// assert_eq!(keyword.value(), " VALUE");
    /// let keyword = Org::parse("#+KEY:").first_node::<Keyword>().unwrap();
    /// assert_eq!(keyword.value(), "");
    /// ```
    pub fn value(&self) -> Token {
        self.syntax
            .children_with_tokens()
            .filter_map(filter_token(SyntaxKind::TEXT))
            .nth(1)
            .unwrap_or_default()
    }
}
