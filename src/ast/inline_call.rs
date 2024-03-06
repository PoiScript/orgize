use crate::syntax::SyntaxKind;

use super::{filter_token, InlineCall, Token};

impl InlineCall {
    ///
    /// ```rust
    /// use orgize::{Org, ast::InlineCall};
    ///
    /// let call = Org::parse("call_square(4)").first_node::<InlineCall>().unwrap();
    /// assert_eq!(call.call(), "square");
    /// ```
    pub fn call(&self) -> Token {
        self.syntax
            .children_with_tokens()
            .filter_map(filter_token(SyntaxKind::TEXT))
            .nth(1)
            .unwrap_or_else(|| {
                debug_assert!(false, "inline call must contains two TEXT");
                Token::default()
            })
    }

    ///
    /// ```rust
    /// use orgize::{Org, ast::InlineCall};
    ///
    /// let call = Org::parse("call_square[:results output](4)").first_node::<InlineCall>().unwrap();
    /// assert_eq!(call.inside_header().unwrap(), ":results output");
    /// ```
    pub fn inside_header(&self) -> Option<Token> {
        self.syntax
            .children_with_tokens()
            .skip_while(|e| e.kind() != SyntaxKind::L_BRACKET)
            .nth(1)
            .map(|e| {
                debug_assert!(e.kind() == SyntaxKind::TEXT);
                Token(e.into_token())
            })
    }

    ///
    /// ```rust
    /// use orgize::{Org, ast::InlineCall};
    ///
    /// let call = Org::parse("call_square(4)").first_node::<InlineCall>().unwrap();
    /// assert_eq!(call.arguments(), "4");
    /// ```
    pub fn arguments(&self) -> Token {
        self.syntax
            .children_with_tokens()
            .skip_while(|e| e.kind() != SyntaxKind::L_PARENS)
            .nth(1)
            .map_or_else(
                || {
                    debug_assert!(false);
                    Token::default()
                },
                |e| {
                    debug_assert!(e.kind() == SyntaxKind::TEXT);
                    Token(e.into_token())
                },
            )
    }

    ///
    /// ```rust
    /// use orgize::{Org, ast::InlineCall};
    ///
    /// let call = Org::parse("call_square[:results output](4)[:results html]").first_node::<InlineCall>().unwrap();
    /// assert_eq!(call.end_header().unwrap(), ":results html");
    /// ```
    pub fn end_header(&self) -> Option<Token> {
        self.syntax
            .children_with_tokens()
            .skip_while(|e| e.kind() != SyntaxKind::L_BRACKET)
            .skip(1)
            .skip_while(|e| e.kind() != SyntaxKind::L_BRACKET)
            .nth(1)
            .map(|e| {
                debug_assert!(e.kind() == SyntaxKind::TEXT);
                Token(e.into_token())
            })
    }
}
