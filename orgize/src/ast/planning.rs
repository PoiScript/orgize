use rowan::ast::AstNode;

use super::{Planning, Timestamp};
use crate::syntax::SyntaxKind;

impl Planning {
    /// Returns deadline timestamp
    ///
    ///
    /// ```rust
    /// use orgize::{ast::Planning, Org};
    ///
    /// let s = Org::parse("* a\nDEADLINE: <2019-04-08 Mon>")
    ///     .first_node::<Planning>()
    ///     .unwrap()
    ///     .deadline()
    ///     .unwrap();
    /// assert_eq!(s.day_start().unwrap(), "08");
    /// ```
    pub fn deadline(&self) -> Option<Timestamp> {
        self.syntax
            .children()
            .filter(|n| n.kind() == SyntaxKind::PLANNING_DEADLINE)
            .last()
            .and_then(|n| n.children().find_map(Timestamp::cast))
    }

    /// Returns scheduled timestamp
    ///
    /// ```rust
    /// use orgize::{ast::Planning, Org};
    ///
    /// let s = Org::parse("* a\nSCHEDULED: <2019-04-08 Mon>")
    ///     .first_node::<Planning>()
    ///     .unwrap()
    ///     .scheduled()
    ///     .unwrap();
    /// assert_eq!(s.year_start().unwrap(), "2019");
    /// ```
    pub fn scheduled(&self) -> Option<Timestamp> {
        self.syntax
            .children()
            .filter(|n| n.kind() == SyntaxKind::PLANNING_SCHEDULED)
            .last()
            .and_then(|n| n.children().find_map(Timestamp::cast))
    }

    /// Returns closed timestamp
    ///
    /// ```rust
    /// use orgize::{ast::Planning, Org};
    ///
    /// let s = Org::parse("* a\nCLOSED: <2019-04-08 Mon>")
    ///     .first_node::<Planning>()
    ///     .unwrap()
    ///     .closed()
    ///     .unwrap();
    /// assert_eq!(s.month_start().unwrap(), "04");
    /// ```
    pub fn closed(&self) -> Option<Timestamp> {
        self.syntax
            .children()
            .filter(|n| n.kind() == SyntaxKind::PLANNING_CLOSED)
            .last()
            .and_then(|n| n.children().find_map(Timestamp::cast))
    }
}
