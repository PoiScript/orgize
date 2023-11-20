use rowan::ast::support;

use crate::{ast::Token, SyntaxKind};

use super::{Clock, Timestamp};

impl Clock {
    pub fn value(&self) -> Option<Timestamp> {
        support::child(&self.syntax)
    }

    /// ```rust
    /// use orgize::{Org, ast::Clock};
    ///
    /// let clock = Org::parse("CLOCK: [2003-09-16 Tue 09:39]").first_node::<Clock>().unwrap();
    /// assert!(clock.duration().is_none());
    /// let clock = Org::parse("CLOCK: [2003-09-16 Tue 09:39] =>12:00").first_node::<Clock>().unwrap();
    /// assert_eq!(clock.duration().unwrap(), "12:00");
    ///
    /// ```
    pub fn duration(&self) -> Option<Token> {
        self.syntax
            .children_with_tokens()
            .skip_while(|t| t.kind() != SyntaxKind::DOUBLE_ARROW)
            .skip(1)
            .find(|t| t.kind() != SyntaxKind::WHITESPACE)
            .map(|e| {
                debug_assert!(e.kind() == SyntaxKind::TEXT);
                Token(e.into_token())
            })
    }

    /// ```rust
    /// use orgize::{Org, ast::Clock};
    ///
    /// let clock = Org::parse("CLOCK: [2003-09-16 Tue 09:39]").first_node::<Clock>().unwrap();
    /// assert!(!clock.is_closed());
    /// let clock = Org::parse("CLOCK: [2003-09-16 Tue 09:39] =>12:00").first_node::<Clock>().unwrap();
    /// assert!(clock.is_closed());
    /// ```
    pub fn is_closed(&self) -> bool {
        self.syntax
            .children_with_tokens()
            .any(|t| t.kind() == SyntaxKind::DOUBLE_ARROW)
    }

    /// ```rust
    /// use orgize::{Org, ast::Clock};
    ///
    /// let clock = Org::parse("CLOCK: [2003-09-16 Tue 09:39]").first_node::<Clock>().unwrap();
    /// assert!(clock.is_running());
    /// let clock = Org::parse("CLOCK: [2003-09-16 Tue 09:39] =>12:00").first_node::<Clock>().unwrap();
    /// assert!(!clock.is_running());
    /// ```
    pub fn is_running(&self) -> bool {
        !self.is_closed()
    }
}
