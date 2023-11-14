use crate::syntax::{SyntaxElement, SyntaxKind, SyntaxToken};

use super::AffiliatedKeyword;

impl AffiliatedKeyword {
    ///
    /// ```rust
    /// use orgize::{Org, ast::AffiliatedKeyword};
    ///
    /// let keyword = Org::parse("#+CAPTION: VALUE\nabc").first_node::<AffiliatedKeyword>().unwrap();
    /// assert_eq!(keyword.key().unwrap().text(), "CAPTION");
    /// ```
    pub fn key(&self) -> Option<SyntaxToken> {
        self.syntax.children_with_tokens().find_map(|it| match it {
            SyntaxElement::Token(t) if t.kind() == SyntaxKind::TEXT => Some(t),
            _ => None,
        })
    }

    ///
    /// ```rust
    /// use orgize::{Org, ast::AffiliatedKeyword};
    ///
    /// let keyword = Org::parse("#+CAPTION: VALUE\nabc").first_node::<AffiliatedKeyword>().unwrap();
    /// assert!(keyword.optional().is_none());
    /// let keyword = Org::parse("#+CAPTION[OPTIONAL]: VALUE\nabc").first_node::<AffiliatedKeyword>().unwrap();
    /// assert_eq!(keyword.optional().unwrap().text(), "OPTIONAL");
    /// ```
    pub fn optional(&self) -> Option<SyntaxToken> {
        self.syntax
            .children_with_tokens()
            .skip_while(|it| it.kind() != SyntaxKind::L_BRACKET)
            .nth(1)
            .and_then(|it| match it {
                SyntaxElement::Token(t) if t.kind() == SyntaxKind::TEXT => Some(t),
                _ => None,
            })
    }

    ///
    /// ```rust
    /// use orgize::{Org, ast::AffiliatedKeyword};
    ///
    /// let keyword = Org::parse("#+CAPTION: VALUE\nabc").first_node::<AffiliatedKeyword>().unwrap();
    /// assert_eq!(keyword.value().unwrap().text(), " VALUE");
    /// let keyword = Org::parse("#+CAPTION[OPTIONAL]:VALUE\nabc").first_node::<AffiliatedKeyword>().unwrap();
    /// assert_eq!(keyword.value().unwrap().text(), "VALUE");
    /// ```
    pub fn value(&self) -> Option<SyntaxToken> {
        self.syntax
            .children_with_tokens()
            .filter_map(|it| match it {
                SyntaxElement::Token(t) if t.kind() == SyntaxKind::TEXT => Some(t),
                _ => None,
            })
            .last()
    }
}
