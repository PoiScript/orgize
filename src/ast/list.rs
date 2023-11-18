use super::{filter_token, List, ListItem};
use crate::{syntax::SyntaxKind, SyntaxElement, SyntaxToken};

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

impl ListItem {
    /// ```rust
    /// use orgize::{Org, ast::ListItem};
    ///
    /// let item = Org::parse("- 1").first_node::<ListItem>().unwrap();
    /// assert_eq!(item.indent(), 0);
    /// let item = Org::parse(" \t * 2").first_node::<ListItem>().unwrap();
    /// assert_eq!(item.indent(), 3);
    /// ```
    pub fn indent(&self) -> usize {
        self.syntax
            .children_with_tokens()
            .find_map(filter_token(SyntaxKind::LIST_ITEM_INDENT))
            .map(|t| t.text().len())
            .unwrap_or_else(|| {
                debug_assert!(false, "list must contains indent token");
                0
            })
    }

    /// ```rust
    /// use orgize::{Org, ast::ListItem};
    ///
    /// let item = Org::parse("- some tag").first_node::<ListItem>().unwrap();
    /// assert_eq!(item.bullet().unwrap().text(), "- ");
    /// let item = Org::parse("2. [X] item 2").first_node::<ListItem>().unwrap();
    /// assert_eq!(item.bullet().unwrap().text(), "2. ");
    /// ```
    pub fn bullet(&self) -> Option<SyntaxToken> {
        self.syntax
            .children_with_tokens()
            .find_map(filter_token(SyntaxKind::LIST_ITEM_BULLET))
    }

    /// ```rust
    /// use orgize::{Org, ast::ListItem};
    ///
    /// let item = Org::parse("- [-] item 1").first_node::<ListItem>().unwrap();
    /// assert_eq!(item.checkbox().unwrap().text(), "-");
    /// let item = Org::parse("2. [X] item 2").first_node::<ListItem>().unwrap();
    /// assert_eq!(item.checkbox().unwrap().text(), "X");
    /// let item = Org::parse("3) [ ] item 3").first_node::<ListItem>().unwrap();
    /// assert_eq!(item.checkbox().unwrap().text(), " ");
    /// ```
    pub fn checkbox(&self) -> Option<SyntaxToken> {
        self.syntax
            .children()
            .find(|n| n.kind() == SyntaxKind::LIST_ITEM_CHECK_BOX)
            .and_then(|n| {
                n.children_with_tokens()
                    .find_map(filter_token(SyntaxKind::TEXT))
            })
    }

    pub fn counter(&self) -> Option<SyntaxToken> {
        self.syntax
            .children()
            .find(|n| n.kind() == SyntaxKind::LIST_ITEM_COUNTER)
            .and_then(|n| {
                n.children_with_tokens()
                    .find_map(filter_token(SyntaxKind::TEXT))
            })
    }

    /// ```rust
    /// use orgize::{Org, ast::ListItem};
    ///
    /// let item = Org::parse("+ this is *TAG* :: item1").first_node::<ListItem>().unwrap();
    /// let tag = item.tag().map(|n| n.to_string()).collect::<String>();
    /// assert_eq!(tag, "this is *TAG* ");
    /// ```
    pub fn tag(&self) -> impl Iterator<Item = SyntaxElement> {
        self.syntax
            .children()
            .find(|n| n.kind() == SyntaxKind::LIST_ITEM_TAG)
            .into_iter()
            .flat_map(|n| {
                n.children_with_tokens().filter(|n| {
                    n.kind() != SyntaxKind::WHITESPACE && n.kind() != SyntaxKind::COLON2
                })
            })
    }
}
