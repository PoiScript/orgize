use rowan::ast::{support, AstNode};

use super::{AffiliatedKeyword, Link, Paragraph, Token};
use crate::syntax::SyntaxKind;

impl Link {
    /// Returns link destination
    ///
    /// ```rust
    /// use orgize::{Org, ast::Link};
    ///
    /// let link = Org::parse("[[#id]]").first_node::<Link>().unwrap();
    /// assert_eq!(link.path(), "#id");
    /// let link = Org::parse("[[https://google.com]]").first_node::<Link>().unwrap();
    /// assert_eq!(link.path(), "https://google.com");
    /// let link = Org::parse("[[https://google.com][Google]]").first_node::<Link>().unwrap();
    /// assert_eq!(link.path(), "https://google.com");
    /// ```
    pub fn path(&self) -> Token {
        support::token(&self.syntax, SyntaxKind::LINK_PATH).map_or_else(
            || {
                debug_assert!(false, "link must contains LINK_PATH");
                Token::default()
            },
            |e| Token(Some(e)),
        )
    }

    /// Returns `true` if link contains description
    ///
    /// ```rust
    /// use orgize::{Org, ast::Link};
    ///
    /// let link = Org::parse("[[https://google.com]]").first_node::<Link>().unwrap();
    /// assert!(!link.has_description());
    /// let link = Org::parse("[[https://google.com][Google]]").first_node::<Link>().unwrap();
    /// assert!(link.has_description());
    /// ```
    pub fn has_description(&self) -> bool {
        support::token(self.syntax(), SyntaxKind::TEXT).is_some()
    }

    /// Returns `true` if link is an image link
    ///
    /// ```rust
    /// use orgize::{Org, ast::Link};
    ///
    /// let link = Org::parse("[[https://google.com]]").first_node::<Link>().unwrap();
    /// assert!(!link.is_image());
    /// let link = Org::parse("[[file:/home/dominik/images/jupiter.jpg]]").first_node::<Link>().unwrap();
    /// assert!(link.is_image());
    /// ```
    pub fn is_image(&self) -> bool {
        const IMAGE_SUFFIX: &[&str] = &[
            // https://github.com/bzg/org-mode/blob/7de1e818d5fbe6a05c6b1a007eed07dc27e7246b/lisp/ox.el#L253
            ".png", ".jpeg", ".jpg", ".gif", ".tiff", ".tif", ".xbm", ".xpm", ".pbm", ".pgm",
            ".ppm", ".webp", ".avif", ".svg",
        ];

        let path = self.path();

        IMAGE_SUFFIX.iter().any(|e| path.ends_with(e)) && !self.has_description()
    }

    /// Returns caption keyword in this link
    ///
    /// ```rust
    /// use orgize::{Org, ast::Link};
    ///
    /// let link = Org::parse("#+CAPTION: image link\n[[file:/home/dominik/images/jupiter.jpg]]").first_node::<Link>().unwrap();
    /// assert_eq!(link.caption().unwrap().value().unwrap(), " image link");
    /// ```
    pub fn caption(&self) -> Option<AffiliatedKeyword> {
        // TODO: support other element type
        Paragraph::cast(self.syntax.parent()?.clone())?.caption()
    }
}
