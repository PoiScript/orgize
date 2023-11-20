use crate::SyntaxKind;

use super::{filter_token, Macros, Token};

impl Macros {
    /// ```rust
    /// use orgize::{Org, ast::Macros};
    ///
    /// let m = Org::parse("{{{title}}}").first_node::<Macros>().unwrap();
    /// assert_eq!(m.key(), "title");
    /// let m = Org::parse("{{{two_arg_macro(1, 2)}}}").first_node::<Macros>().unwrap();
    /// assert_eq!(m.key(), "two_arg_macro");
    /// ```
    pub fn key(&self) -> Token {
        self.syntax
            .children_with_tokens()
            .find_map(filter_token(SyntaxKind::TEXT))
            .unwrap_or_else(|| {
                debug_assert!(false, "macros must contains TEXT");
                Token::default()
            })
    }

    /// ```rust
    /// use orgize::{Org, ast::Macros};
    ///
    /// let m = Org::parse("{{{title}}}").first_node::<Macros>().unwrap();
    /// assert!(m.args().is_none());
    /// let m = Org::parse("{{{two_arg_macro(1, 2)}}}").first_node::<Macros>().unwrap();
    /// assert_eq!(m.args().unwrap(), "1, 2");
    /// ```
    pub fn args(&self) -> Option<Token> {
        self.syntax
            .children_with_tokens()
            .filter_map(filter_token(SyntaxKind::TEXT))
            .nth(1)
    }
}
