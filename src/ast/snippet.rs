use crate::syntax::{SyntaxKind, SyntaxToken};

use super::{filter_token, Snippet};

impl Snippet {
    /// ```rust
    /// use orgize::{Org, ast::Snippet};
    ///
    /// let snippet = Org::parse("@@BACKEND:VALUE@@").first_node::<Snippet>().unwrap();
    /// assert_eq!(snippet.value().unwrap().text(), "VALUE");
    /// ```
    pub fn value(&self) -> Option<SyntaxToken> {
        self.syntax
            .children_with_tokens()
            .filter_map(filter_token(SyntaxKind::TEXT))
            .nth(1)
    }
}
