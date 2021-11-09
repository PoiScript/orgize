use rowan::ast::{support, AstNode};

use super::Link;
use crate::syntax::SyntaxKind;

impl Link {
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
    /// let link = Org::parse("[[file:/home/dominik/images/jupiter.jpg]]").first_node::<Link>().unwrap();
    /// assert!(link.is_image());
    /// ```
    pub fn is_image(&self) -> bool {
        const IMAGE_SUFFIX: &[&str] = &[
            // https://github.com/bzg/org-mode/blob/7de1e818d5fbe6a05c6b1a007eed07dc27e7246b/lisp/ox.el#L253
            ".png", ".jpeg", ".jpg", ".gif", ".tiff", ".tif", ".xbm", ".xpm", ".pbm", ".pgm",
            ".ppm", ".webp", ".avif", ".svg",
        ];

        self.path()
            .map(|path| IMAGE_SUFFIX.iter().any(|e| path.text().ends_with(e)))
            .unwrap_or_default()
            && !self.has_description()
    }
}
