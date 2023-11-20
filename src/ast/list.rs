use super::{filter_token, List, ListItem, Token};
use crate::{syntax::SyntaxKind, SyntaxElement};

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
        self.items().next().map_or_else(
            || {
                debug_assert!(false, "list muts contains LIST_ITEM");
                false
            },
            |item| item.bullet().starts_with(|c: char| c.is_ascii_digit()),
        )
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
        self.items().next().map_or_else(
            || {
                debug_assert!(false, "list must contains LIST_ITEM");
                false
            },
            |item| {
                item.syntax
                    .children()
                    .any(|it| it.kind() == SyntaxKind::LIST_ITEM_TAG)
            },
        )
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
            .map_or_else(
                || {
                    debug_assert!(false, "list item must contains LIST_ITEM_INDENT");
                    0
                },
                |t| t.len(),
            )
    }

    /// ```rust
    /// use orgize::{Org, ast::ListItem};
    ///
    /// let item = Org::parse("- some tag").first_node::<ListItem>().unwrap();
    /// assert_eq!(item.bullet(), "- ");
    /// let item = Org::parse("2. [X] item 2").first_node::<ListItem>().unwrap();
    /// assert_eq!(item.bullet(), "2. ");
    /// ```
    pub fn bullet(&self) -> Token {
        self.syntax
            .children_with_tokens()
            .find_map(filter_token(SyntaxKind::LIST_ITEM_BULLET))
            .unwrap_or_else(|| {
                debug_assert!(false, "list item must contains LIST_ITEM_BULLET");
                Token::default()
            })
    }

    /// ```rust
    /// use orgize::{Org, ast::ListItem};
    ///
    /// let item = Org::parse("- [-] item 1").first_node::<ListItem>().unwrap();
    /// assert_eq!(item.checkbox().unwrap(), "-");
    /// let item = Org::parse("2. [X] item 2").first_node::<ListItem>().unwrap();
    /// assert_eq!(item.checkbox().unwrap(), "X");
    /// let item = Org::parse("3) [ ] item 3").first_node::<ListItem>().unwrap();
    /// assert_eq!(item.checkbox().unwrap(), " ");
    /// ```
    pub fn checkbox(&self) -> Option<Token> {
        self.syntax
            .children()
            .find(|n| n.kind() == SyntaxKind::LIST_ITEM_CHECK_BOX)
            .and_then(|n| {
                n.children_with_tokens()
                    .find_map(filter_token(SyntaxKind::TEXT))
            })
    }

    pub fn counter(&self) -> Option<Token> {
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
