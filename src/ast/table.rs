use rowan::ast::AstNode;

use super::{OrgTable, OrgTableRow};
use crate::syntax::SyntaxKind;

impl OrgTable {
    /// Returns `true` if this table has a header
    ///
    /// A table has a header when it contains at least two row groups.
    ///
    /// ```rust
    /// use orgize::{Org, ast::OrgTable};
    ///
    /// let org = Org::parse(r#"
    /// | a | b |
    /// |---+---|
    /// | c | d |"#);
    /// let table = org.first_node::<OrgTable>().unwrap();
    /// assert!(table.has_header());
    ///
    /// let org = Org::parse(r#"
    /// | a | b |
    /// | 0 | 1 |
    /// |---+---|
    /// | a | w |"#);
    /// let table = org.first_node::<OrgTable>().unwrap();
    /// assert!(table.has_header());
    ///
    /// let org = Org::parse(r#"
    /// | a | b |
    /// | c | d |"#);
    /// let table = org.first_node::<OrgTable>().unwrap();
    /// assert!(!table.has_header());
    ///
    /// let org = Org::parse(r#"
    /// |---+---|
    /// | a | b |
    /// | c | d |
    /// |---+---|"#);
    /// let table = org.first_node::<OrgTable>().unwrap();
    /// assert!(!table.has_header());
    /// ```
    pub fn has_header(&self) -> bool {
        self.syntax
            .children()
            .filter_map(OrgTableRow::cast)
            .skip_while(|row| row.is_rule())
            .skip_while(|row| row.is_standard())
            .any(|row| !row.is_rule())
    }
}

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
