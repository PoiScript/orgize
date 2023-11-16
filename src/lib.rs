#![doc = include_str!("../README.md")]

pub mod ast;
mod config;
pub mod export;
mod org;
mod syntax;
#[cfg(test)]
mod tests;

// Re-export of the rowan crate.
pub use rowan;

pub use config::ParseConfig;
pub use org::Org;
pub use syntax::{
    SyntaxElement, SyntaxElementChildren, SyntaxKind, SyntaxNode, SyntaxNodeChildren, SyntaxToken,
};

pub(crate) use syntax::combinator::lossless_parser;
