use super::List;
use crate::syntax::SyntaxKind;

impl List {
    /// Returns `true` if this list is an ordered link
    ///
    /// ```rust
    /// use orgize::{Org, ast::List};
    ///
    /// let list = Org::parse("+ 1").first_node::<List>().unwrap();
    /// assert!(!list.is_ordered());
    ///
    /// let list = Org::parse("1. 1").first_node::<List>().unwrap();
    /// assert!(list.is_ordered());
    ///
    /// let list = Org::parse("1) 1\n- 2\n3. 3").first_node::<List>().unwrap();
    /// assert!(list.is_ordered());
    /// ```
    pub fn is_ordered(&self) -> bool {
        self.items()
            .next()
            .and_then(|item| item.bullet())
            .map(|bullet| bullet.text().starts_with(|c: char| c.is_ascii_digit()))
            .unwrap_or_default()
    }

    /// Returns `true` if this list contains a TAG
    ///
    /// ```rust
    /// use orgize::{Org, ast::List};
    ///
    /// let list = Org::parse("- some tag :: item 2.1").first_node::<List>().unwrap();
    /// assert!(list.is_descriptive());
    /// let list = Org::parse("2. [X] item 2").first_node::<List>().unwrap();
    /// assert!(!list.is_descriptive());
    /// ```
    pub fn is_descriptive(&self) -> bool {
        self.items()
            .next()
            .map(|item| {
                item.syntax
                    .children()
                    .any(|it| it.kind() == SyntaxKind::LIST_ITEM_TAG)
            })
            .unwrap_or_default()
    }
}
