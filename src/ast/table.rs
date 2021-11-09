use super::OrgTableRow;
use crate::syntax::SyntaxKind;

impl OrgTableRow {
    /// Returns `true` if this row is a rule
    ///
    /// ```rust
    /// use orgize::{Org, ast::OrgTableRow};
    ///
    /// let org = Org::parse("|----|----|\n|Foo |Bar |");
    /// let row = org.first_node::<OrgTableRow>().unwrap();
    /// assert!(row.is_rule());
    /// ```
    pub fn is_rule(&self) -> bool {
        self.syntax.kind() == SyntaxKind::ORG_TABLE_RULE_ROW
    }

    /// Returns `true` if this row is a standard row
    ///
    /// ```rust
    /// use orgize::{Org, ast::OrgTableRow};
    ///
    /// let org = Org::parse("|Foo |Bar |\n|----|----|");
    /// let row = org.first_node::<OrgTableRow>().unwrap();
    /// assert!(row.is_standard());
    /// ```
    pub fn is_standard(&self) -> bool {
        self.syntax.kind() == SyntaxKind::ORG_TABLE_STANDARD_ROW
    }
}
