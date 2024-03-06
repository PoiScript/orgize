use crate::{entities::ENTITIES, SyntaxKind};

use super::{filter_token, Entity};

impl Entity {
    fn entity(&self) -> Option<&(&str, &str, bool, &str, &str, &str, &str)> {
        let token = self
            .syntax
            .children_with_tokens()
            .find_map(filter_token(SyntaxKind::TEXT))?;

        ENTITIES.iter().find(|i| i.0 == token.as_ref())
    }

    /// Entity name
    ///
    /// ```rust
    /// use orgize::{ast::Entity, Org};
    ///
    /// let e = Org::parse("\\alpha{}").first_node::<Entity>().unwrap();
    /// assert_eq!(e.name(), "alpha");
    /// let e = Org::parse("\\_     ").first_node::<Entity>().unwrap();
    /// assert_eq!(e.name(), "     ");
    /// ```
    pub fn name(&self) -> &str {
        self.entity().map_or_else(
            || {
                debug_assert!(false);
                ""
            },
            |e| e.0,
        )
    }

    /// Entity LaTeX representation
    ///
    /// ```rust
    /// use orgize::{ast::Entity, Org};
    ///
    /// let e = Org::parse("\\middot").first_node::<Entity>().unwrap();
    /// assert_eq!(e.latex(), "\\textperiodcentered{}");
    /// ```
    pub fn latex(&self) -> &str {
        self.entity().map_or_else(
            || {
                debug_assert!(false);
                ""
            },
            |e| e.1,
        )
    }

    /// Whether entity needs to be in math mode
    ///
    /// ```rust
    /// use orgize::{ast::Entity, Org};
    ///
    /// let e = Org::parse("\\middot").first_node::<Entity>().unwrap();
    /// assert!(!e.is_latex_math());
    /// let e = Org::parse("\\alefsym").first_node::<Entity>().unwrap();
    /// assert!(e.is_latex_math());
    /// ```
    pub fn is_latex_math(&self) -> bool {
        self.entity().map_or_else(
            || {
                debug_assert!(false);
                false
            },
            |e| e.2,
        )
    }

    /// Entity HTML representation
    ///
    /// ```rust
    /// use orgize::{ast::Entity, Org};
    ///
    /// let e = Org::parse("\\S").first_node::<Entity>().unwrap();
    /// assert_eq!(e.html(), "&sect;");
    /// ```
    pub fn html(&self) -> &str {
        self.entity().map_or_else(
            || {
                debug_assert!(false);
                ""
            },
            |e| e.3,
        )
    }

    /// Entity ASCII representation
    ///
    /// ```rust
    /// use orgize::{ast::Entity, Org};
    ///
    /// let e = Org::parse("\\S").first_node::<Entity>().unwrap();
    /// assert_eq!(e.ascii(), "section");
    /// ```
    pub fn ascii(&self) -> &str {
        self.entity().map_or_else(
            || {
                debug_assert!(false);
                ""
            },
            |e| e.4,
        )
    }

    /// Entity Latin1 encoding representation
    ///
    /// ```rust
    /// use orgize::{ast::Entity, Org};
    ///
    /// let e = Org::parse("\\S").first_node::<Entity>().unwrap();
    /// assert_eq!(e.latin1(), "§");
    /// let e = Org::parse("\\rsaquo").first_node::<Entity>().unwrap();
    /// assert_eq!(e.latin1(), ">");
    /// ```
    pub fn latin1(&self) -> &str {
        self.entity().map_or_else(
            || {
                debug_assert!(false);
                ""
            },
            |e| e.5,
        )
    }

    /// Entity UTF-8 encoding representation
    ///
    /// ```rust
    /// use orgize::{ast::Entity, Org};
    ///
    /// let e = Org::parse("\\S").first_node::<Entity>().unwrap();
    /// assert_eq!(e.utf8(), "§");
    /// let e = Org::parse("\\rsaquo").first_node::<Entity>().unwrap();
    /// assert_eq!(e.utf8(), "›");
    /// ```
    pub fn utf8(&self) -> &str {
        self.entity().map_or_else(
            || {
                debug_assert!(false);
                ""
            },
            |e| e.6,
        )
    }

    /// Entity contains optional brackets
    ///
    /// ```rust
    /// use orgize::{ast::Entity, Org};
    ///
    /// let e = Org::parse("\\beta").first_node::<Entity>().unwrap();
    /// assert!(!e.is_use_brackets());
    /// let e = Org::parse("\\S{}").first_node::<Entity>().unwrap();
    /// assert!(e.is_use_brackets());
    /// let e = Org::parse("\\_     ").first_node::<Entity>().unwrap();
    /// assert!(!e.is_use_brackets());
    /// ```
    pub fn is_use_brackets(&self) -> bool {
        self.syntax
            .children_with_tokens()
            .filter(|n| n.kind() == SyntaxKind::TEXT)
            .nth(1)
            .is_some()
    }
}
