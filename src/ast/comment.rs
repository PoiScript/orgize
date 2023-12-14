use crate::SyntaxKind;

use super::{filter_token, Comment};

impl Comment {
    /// Contents without pound signs
    ///
    /// ```rust
    /// use orgize::{ast::Comment, Org};
    ///
    /// let fixed = Org::parse("# A\n#\n# B\n# C").first_node::<Comment>().unwrap();
    /// assert_eq!(fixed.value(), "A\n\nB\nC");
    /// ```
    pub fn value(&self) -> String {
        self.syntax
            .children_with_tokens()
            .filter_map(filter_token(SyntaxKind::TEXT))
            .fold(String::new(), |acc, text| acc + &text)
    }
}
