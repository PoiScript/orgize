use std::collections::HashMap;

use super::{filter_token, SyntaxKind::*};
use crate::{ast::PropertyDrawer, syntax::SyntaxToken};

impl PropertyDrawer {
    /// ```rust
    /// use orgize::{Org, ast::PropertyDrawer};
    ///
    /// let org = Org::parse("* Heading\n:PROPERTIES:\n:CUSTOM_ID: someid\n:ID: id\n:END:");
    /// let drawer = org.first_node::<PropertyDrawer>().unwrap();
    /// assert_eq!(drawer.iter().count(), 2);
    /// ```
    pub fn iter(&self) -> impl Iterator<Item = (SyntaxToken, SyntaxToken)> {
        self.node_properties().filter_map(|property| {
            let mut texts = property
                .syntax
                .children_with_tokens()
                .filter_map(filter_token(TEXT));

            Some((texts.next()?, texts.next()?))
        })
    }

    /// ```rust
    /// use orgize::{Org, ast::PropertyDrawer};
    ///
    /// let org = Org::parse("* Heading\n:PROPERTIES:\n:CUSTOM_ID: someid\n:ID: id\n:END:");
    /// let drawer = org.first_node::<PropertyDrawer>().unwrap();
    /// assert_eq!(drawer.get("CUSTOM_ID").unwrap().text(), "someid");
    /// assert_eq!(drawer.get("ID").unwrap().text(), "id");
    /// ```
    pub fn get(&self, key: &str) -> Option<SyntaxToken> {
        self.iter()
            .find_map(|(k, v)| (k.text() == key).then_some(v))
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
    pub fn to_hash_map(&self) -> HashMap<String, String> {
        self.iter()
            .map(|(k, v)| (k.text().into(), v.text().into()))
            .collect()
    }

    #[cfg(feature = "indexmap")]
    /// ```rust
    /// use orgize::{Org, ast::PropertyDrawer};
    ///
    /// let org = Org::parse("* Heading\n:PROPERTIES:\n:CUSTOM_ID: someid\n:ID: id\n:END:");
    /// let drawer = org.first_node::<PropertyDrawer>().unwrap();
    /// let map = drawer.to_index_map();
    /// assert_eq!(map.get_index(1).unwrap(), (&"ID".to_string(), &"id".to_string()));
    /// ```
    pub fn to_index_map(&self) -> indexmap::IndexMap<String, String> {
        self.iter()
            .map(|(k, v)| (k.text().into(), v.text().into()))
            .collect()
    }
}
