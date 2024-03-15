use std::collections::HashMap;

use super::{filter_token, Drawer, SyntaxKind, Token};
use crate::ast::PropertyDrawer;

impl PropertyDrawer {
    /// ```rust
    /// use orgize::{Org, ast::PropertyDrawer};
    ///
    /// let org = Org::parse("* Heading\n:PROPERTIES:\n:CUSTOM_ID: someid\n:ID: id\n:END:");
    /// let drawer = org.first_node::<PropertyDrawer>().unwrap();
    /// assert_eq!(drawer.iter().count(), 2);
    /// ```
    pub fn iter(&self) -> impl Iterator<Item = (Token, Token)> {
        self.node_properties().filter_map(|property| {
            let mut texts = property
                .syntax
                .children_with_tokens()
                .filter_map(filter_token(SyntaxKind::TEXT));

            Some((texts.next()?, texts.next()?))
        })
    }

    /// ```rust
    /// use orgize::{Org, ast::PropertyDrawer};
    ///
    /// let org = Org::parse("* Heading\n:PROPERTIES:\n:CUSTOM_ID: someid\n:ID: id\n:END:");
    /// let drawer = org.first_node::<PropertyDrawer>().unwrap();
    /// assert_eq!(drawer.get("CUSTOM_ID").unwrap(), "someid");
    /// assert_eq!(drawer.get("ID").unwrap(), "id");
    /// ```
    pub fn get(&self, key: &str) -> Option<Token> {
        self.iter().find_map(|(k, v)| (k == key).then_some(v))
    }

    /// ```rust
    /// use orgize::{Org, ast::PropertyDrawer};
    ///
    /// let org = Org::parse("* Heading\n:PROPERTIES:\n:CUSTOM_ID: someid\n:CUSTOM_ID: id\n:END:");
    /// let drawer = org.first_node::<PropertyDrawer>().unwrap();
    /// let map = drawer.to_hash_map();
    /// assert_eq!(map.len(), 1);
    /// assert_eq!(map.get("CUSTOM_ID").unwrap(), "id");
    /// ```
    pub fn to_hash_map(&self) -> HashMap<Token, Token> {
        self.iter().collect()
    }

    #[cfg(feature = "indexmap")]
    /// ```rust
    /// use orgize::{Org, ast::PropertyDrawer};
    ///
    /// let org = Org::parse("* Heading\n:PROPERTIES:\n:CUSTOM_ID: someid\n:ID: id\n:END:");
    /// let drawer = org.first_node::<PropertyDrawer>().unwrap();
    /// let map = drawer.to_index_map();
    /// let item1 = map.get_index(1).unwrap();
    /// assert_eq!(item1.0, "ID");
    /// assert_eq!(item1.1, "id");
    /// ```
    pub fn to_index_map(&self) -> indexmap::IndexMap<Token, Token> {
        self.iter().collect()
    }
}

impl Drawer {
    /// ```rust
    /// use orgize::{Org, ast::Drawer};
    ///
    /// let org = Org::parse("* Heading\n:LOGBOOK:\n:END:");
    /// let drawer = org.first_node::<Drawer>().unwrap();
    /// assert_eq!(drawer.name(), "LOGBOOK");
    /// ```
    pub fn name(&self) -> Token {
        self.syntax
            .first_child()
            .and_then(|n| {
                n.children_with_tokens()
                    .filter_map(|e| e.into_token())
                    .find(|e| e.kind() == SyntaxKind::TEXT)
            })
            .map(|t| Token(Some(t)))
            .unwrap_or_default()
    }
}
