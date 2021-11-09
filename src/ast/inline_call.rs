use crate::syntax::{SyntaxElement, SyntaxKind, SyntaxToken};

use super::InlineCall;

impl InlineCall {
    ///
    /// ```rust
    /// use orgize::{Org, ast::InlineCall};
    ///
    /// let call = Org::parse("call_square(4)").first_node::<InlineCall>().unwrap();
    /// assert_eq!(call.call().unwrap().text(), "square");
    /// ```
    pub fn call(&self) -> Option<SyntaxToken> {
        self.syntax
            .children_with_tokens()
            .filter_map(|it| match it {
                SyntaxElement::Token(t) if t.kind() == SyntaxKind::TEXT => Some(t),
                _ => None,
            })
            .nth(1)
    }
}
