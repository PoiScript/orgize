use crate::SyntaxKind;

use super::{filter_token, InlineSrc, Token};

impl InlineSrc {
    /// Language of the code
    ///
    /// ```rust
    /// use orgize::{Org, ast::InlineSrc};
    ///
    /// let s = Org::parse("src_C{int a = 0;}").first_node::<InlineSrc>().unwrap();
    /// assert_eq!(s.language(), "C");
    /// let s = Org::parse("src_xml[:exports code]{<tag>text</tag>}").first_node::<InlineSrc>().unwrap();
    /// assert_eq!(s.language(), "xml");
    /// ```
    pub fn language(&self) -> Token {
        self.syntax
            .children_with_tokens()
            .nth(1)
            .and_then(filter_token(SyntaxKind::TEXT))
            .unwrap_or_else(|| {
                debug_assert!(false, "inline src must contains TEXT");
                Token::default()
            })
    }

    /// Optional header arguments
    ///
    /// ```rust
    /// use orgize::{Org, ast::InlineSrc};
    ///
    /// let s = Org::parse("src_C{int a = 0;}").first_node::<InlineSrc>().unwrap();
    /// assert!(s.parameters().is_none());
    /// let s = Org::parse("src_xml[:exports code]{<tag>text</tag>}").first_node::<InlineSrc>().unwrap();
    /// assert_eq!(s.parameters().unwrap(), ":exports code");
    /// ```
    pub fn parameters(&self) -> Option<Token> {
        self.syntax
            .children_with_tokens()
            .skip_while(|n| n.kind() != SyntaxKind::L_BRACKET)
            .nth(1)
            .map(|n| {
                debug_assert!(n.kind() == SyntaxKind::TEXT);
                Token(n.into_token())
            })
    }

    /// Source code
    ///
    /// ```rust
    /// use orgize::{Org, ast::InlineSrc};
    ///
    /// let s = Org::parse("src_C{int a = 0;}").first_node::<InlineSrc>().unwrap();
    /// assert_eq!(s.value(), "int a = 0;");
    /// let s = Org::parse("src_xml[:exports code]{<tag>text</tag>}").first_node::<InlineSrc>().unwrap();
    /// assert_eq!(s.value(), "<tag>text</tag>");
    /// ```
    pub fn value(&self) -> Token {
        self.syntax
            .children_with_tokens()
            .filter_map(filter_token(SyntaxKind::TEXT))
            .last()
            .unwrap_or_else(|| {
                debug_assert!(false, "inline src must contains TEXT");
                Token::default()
            })
    }
}
