use crate::{SyntaxKind, SyntaxNode};

use super::{filter_token, SourceBlock, Token};

fn argument(node: &SyntaxNode, name: &str) -> Option<Token> {
    node.children()
        .find(|e| e.kind() == SyntaxKind::BLOCK_BEGIN)
        .and_then(|n| {
            let mut iter = n
                .children_with_tokens()
                .filter_map(filter_token(SyntaxKind::TEXT))
                .skip_while(|n| n != name);

            iter.next()?;

            Some(iter.next().unwrap_or_default())
        })
}

impl SourceBlock {
    /// ```rust
    /// use orgize::{Org, ast::SourceBlock};
    ///
    /// let block = Org::parse("#+begin_src c\n#+end_src").first_node::<SourceBlock>().unwrap();
    /// assert_eq!(block.language(), "c");
    /// let block = Org::parse("#+begin_src javascript \n#+end_src").first_node::<SourceBlock>().unwrap();
    /// assert_eq!(block.language(), "javascript");
    /// let block = Org::parse("#+begin_src\n#+end_src").first_node::<SourceBlock>().unwrap();
    /// assert_eq!(block.language(), "");
    /// ````
    pub fn language(&self) -> Token {
        self.syntax
            .children()
            .find(|e| e.kind() == SyntaxKind::BLOCK_BEGIN)
            .and_then(|n| {
                n.children_with_tokens()
                    .filter_map(filter_token(SyntaxKind::TEXT))
                    .nth(2)
            })
            .unwrap_or_default()
    }

    /// ```rust
    /// use orgize::{Org, ast::SourceBlock};
    ///
    /// let block = Org::parse("#+begin_src c :tangle yes\n#+end_src").first_node::<SourceBlock>().unwrap();
    /// assert_eq!(block.tangle().unwrap(), "yes");
    /// let block = Org::parse("#+begin_src c :tangle\n#+end_src").first_node::<SourceBlock>().unwrap();
    /// assert_eq!(block.tangle().unwrap(), "");
    /// let block = Org::parse("#+begin_src c\n#+end_src").first_node::<SourceBlock>().unwrap();
    /// assert!(block.tangle().is_none());
    /// ````
    pub fn tangle(&self) -> Option<Token> {
        argument(&self.syntax, ":tangle")
    }

    /// ```rust
    /// use orgize::{Org, ast::SourceBlock};
    ///
    /// let block = Org::parse("#+begin_src c :mkdir yes\n#+end_src").first_node::<SourceBlock>().unwrap();
    /// assert_eq!(block.mkdir().unwrap(), "yes");
    /// let block = Org::parse("#+begin_src c :mkdir\n#+end_src").first_node::<SourceBlock>().unwrap();
    /// assert_eq!(block.mkdir().unwrap(), "");
    /// let block = Org::parse("#+begin_src c\n#+end_src").first_node::<SourceBlock>().unwrap();
    /// assert!(block.mkdir().is_none());
    /// ````
    pub fn mkdir(&self) -> Option<Token> {
        argument(&self.syntax, ":mkdir")
    }

    /// ```rust
    /// use orgize::{Org, ast::SourceBlock};
    ///
    /// let block = Org::parse("#+begin_src c :comments both\n#+end_src").first_node::<SourceBlock>().unwrap();
    /// assert_eq!(block.comments().unwrap(), "both");
    /// let block = Org::parse("#+begin_src c :comments\n#+end_src").first_node::<SourceBlock>().unwrap();
    /// assert_eq!(block.comments().unwrap(), "");
    /// let block = Org::parse("#+begin_src c\n#+end_src").first_node::<SourceBlock>().unwrap();
    /// assert!(block.comments().is_none());
    /// ````
    pub fn comments(&self) -> Option<Token> {
        argument(&self.syntax, ":comments")
    }

    /// ```rust
    /// use orgize::{Org, ast::SourceBlock};
    ///
    /// let block = Org::parse("#+begin_src c :padline yes\n#+end_src").first_node::<SourceBlock>().unwrap();
    /// assert_eq!(block.padline().unwrap(), "yes");
    /// let block = Org::parse("#+begin_src c :padline\n#+end_src").first_node::<SourceBlock>().unwrap();
    /// assert_eq!(block.padline().unwrap(), "");
    /// let block = Org::parse("#+begin_src c\n#+end_src").first_node::<SourceBlock>().unwrap();
    /// assert!(block.padline().is_none());
    /// ````
    pub fn padline(&self) -> Option<Token> {
        argument(&self.syntax, ":padline")
    }

    /// ```rust
    /// use orgize::{Org, ast::SourceBlock};
    ///
    /// let block = Org::parse("#+begin_src c :tangle-mode o444\n#+end_src").first_node::<SourceBlock>().unwrap();
    /// assert_eq!(block.tangle_mode().unwrap(), "o444");
    /// let block = Org::parse("#+begin_src c :tangle-mode\n#+end_src").first_node::<SourceBlock>().unwrap();
    /// assert_eq!(block.tangle_mode().unwrap(), "");
    /// let block = Org::parse("#+begin_src c\n#+end_src").first_node::<SourceBlock>().unwrap();
    /// assert!(block.tangle_mode().is_none());
    /// ````
    pub fn tangle_mode(&self) -> Option<Token> {
        argument(&self.syntax, ":tangle-mode")
    }
}
