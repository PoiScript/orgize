#[rustfmt::skip]
mod generated;


mod affiliated_keyword;
mod block;
mod clock;
mod drawer;
mod entity;
mod headline;
mod inline_call;
mod inline_src;
mod link;
mod list;
mod macros;
mod planning;
mod snippet;
mod table;
mod timestamp;

use std::{
    borrow::{Borrow, Cow},
    fmt::Debug,
    hash::Hash,
    ops::Deref,
};

pub use generated::*;
pub use headline::*;
pub use rowan::ast::support::*;
pub use timestamp::*;

use crate::{
    syntax::{SyntaxKind, SyntaxNode},
    SyntaxToken,
};
use rowan::{ast::AstNode, NodeOrToken};

pub fn blank_lines(parent: &SyntaxNode) -> usize {
    parent
        .children_with_tokens()
        .filter(|n| n.kind() == SyntaxKind::BLANK_LINE)
        .count()
}

pub fn last_child<N: AstNode>(parent: &rowan::SyntaxNode<N::Language>) -> Option<N> {
    parent.children().filter_map(N::cast).last()
}

pub fn last_token(parent: &SyntaxNode, kind: SyntaxKind) -> Option<Token> {
    parent
        .children_with_tokens()
        .filter_map(filter_token(kind))
        .last()
}

pub fn token(parent: &SyntaxNode, kind: SyntaxKind) -> Option<Token> {
    rowan::ast::support::token(parent, kind).map(|t| Token(Some(t)))
}

pub fn filter_token(
    kind: SyntaxKind,
) -> impl Fn(NodeOrToken<SyntaxNode, SyntaxToken>) -> Option<Token> {
    move |elem| match elem {
        NodeOrToken::Token(tk) if tk.kind() == kind => Some(Token(Some(tk))),
        _ => None,
    }
}

/// A simple wrapper of `Option<SyntaxToken>`
///
/// It acts like a `token.text()` when inner is `Some(token)`, and an empty string when `None`.
#[derive(Default, Eq)]
pub struct Token(pub(crate) Option<SyntaxToken>);

impl AsRef<str> for Token {
    fn as_ref(&self) -> &str {
        match &self.0 {
            Some(t) => t.text(),
            None => "",
        }
    }
}

impl Borrow<str> for Token {
    fn borrow(&self) -> &str {
        self.as_ref()
    }
}

impl Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_ref().fmt(f)
    }
}

impl<'a> PartialEq<&'a str> for Token {
    fn eq(&self, other: &&'a str) -> bool {
        self.as_ref() == *other
    }
}

impl PartialEq<String> for Token {
    fn eq(&self, other: &String) -> bool {
        self.as_ref() == other
    }
}

impl PartialEq<Token> for Token {
    fn eq(&self, other: &Token) -> bool {
        self.as_ref() == other.as_ref()
    }
}

impl Hash for Token {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.as_ref().hash(state)
    }
}

impl<'a> PartialEq<Cow<'a, str>> for Token {
    fn eq(&self, other: &Cow<'a, str>) -> bool {
        self.as_ref() == other
    }
}

impl PartialEq<str> for Token {
    fn eq(&self, other: &str) -> bool {
        self.as_ref() == other
    }
}

impl Deref for Token {
    type Target = str;

    #[inline]
    fn deref(&self) -> &str {
        self.as_ref()
    }
}
