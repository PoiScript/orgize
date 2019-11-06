use indextree::NodeId;
use std::ops::RangeInclusive;

use crate::elements::{Element, Table, TableCell, TableRow};
use crate::Org;

/// Validation Error
#[derive(Debug)]
pub enum ValidationError {
    /// Expected at least one child
    ExpectedChildren {
        at: NodeId,
    },
    /// Expected no children
    UnexpectedChildren {
        at: NodeId,
    },
    UnexpectedElement {
        expected: &'static str,
        at: NodeId,
    },
    /// Expected a detached element
    ExpectedDetached {
        at: NodeId,
    },
    /// Expected headline level in sepcify range
    HeadlineLevelMismatch {
        range: RangeInclusive<usize>,
        at: NodeId,
    },
}

impl ValidationError {
    pub fn element<'a, 'b>(&self, org: &'a Org<'b>) -> &'a Element<'b> {
        match self {
            ValidationError::ExpectedChildren { at }
            | ValidationError::UnexpectedChildren { at }
            | ValidationError::UnexpectedElement { at, .. }
            | ValidationError::ExpectedDetached { at }
            | ValidationError::HeadlineLevelMismatch { at, .. } => &org[*at],
        }
    }
}

pub type ValidationResult<T> = Result<T, ValidationError>;

impl Org<'_> {
    /// Validates an `Org` struct.
    pub fn validate(&self) -> Vec<ValidationError> {
        let mut errors = Vec::new();

        macro_rules! expect_element {
            ($node:ident, $expect:expr, $($pattern:pat)|+) => {
                match self[$node] {
                    $($pattern)|+ => (),
                    _ => errors.push(ValidationError::UnexpectedElement {
                        expected: $expect,
                        at: $node
                    }),
                }
            };
        }

        macro_rules! expect_children {
            ($node:ident) => {
                if self.arena[$node].first_child().is_none() {
                    errors.push(ValidationError::ExpectedChildren { at: $node });
                }
            };
        }

        for node_id in self.root.descendants(&self.arena) {
            let node = &self.arena[node_id];
            match node.get() {
                Element::Document { .. } => {
                    let mut children = node_id.children(&self.arena);
                    if let Some(child) = children.next() {
                        expect_element!(
                            child,
                            "Headline|Section",
                            Element::Headline { .. } | Element::Section
                        );
                    }

                    for child in children {
                        expect_element!(
                            child,
                            "Headline",
                            Element::Headline { .. }
                        );
                    }
                }
                Element::Headline { .. } => {
                    expect_children!(node_id);

                    let mut children = node_id.children(&self.arena);
                    if let Some(child) = children.next() {
                        expect_element!(child, "Title", Element::Title(_));
                    }

                    if let Some(child) = children.next() {
                        expect_element!(
                            child,
                            "Headline|Section",
                            Element::Headline { .. } | Element::Section
                        );
                    }

                    for child in children {
                        expect_element!(
                            child,
                            "Headline",
                            Element::Headline { .. }
                        );
                    }
                }
                Element::Title(title) => {
                    if !title.raw.is_empty() && node.first_child().is_none() {
                        errors.push(ValidationError::ExpectedChildren { at: node_id });
                    }
                }
                Element::List(_) => {
                    expect_children!(node_id);
                    for child in node_id.children(&self.arena) {
                        expect_element!(child, "ListItem", Element::ListItem(_));
                    }
                }
                Element::Table(Table::Org { .. }) => {
                    for child in node_id.children(&self.arena) {
                        expect_element!(child, "TableRow", Element::TableRow(_));
                    }
                }
                Element::TableRow(TableRow::Header) => {
                    for child in node_id.children(&self.arena) {
                        expect_element!(
                            child,
                            "TableCell::Header",
                            Element::TableCell(TableCell::Header)
                        );
                    }
                }
                Element::TableRow(TableRow::Body) => {
                    for child in node_id.children(&self.arena) {
                        expect_element!(
                            child,
                            "TableCell::Body",
                            Element::TableCell(TableCell::Body)
                        );
                    }
                }
                Element::CommentBlock(_)
                | Element::ExampleBlock(_)
                | Element::ExportBlock(_)
                | Element::SourceBlock(_)
                | Element::BabelCall(_)
                | Element::InlineSrc(_)
                | Element::Code { .. }
                | Element::FnRef(_)
                | Element::InlineCall(_)
                | Element::Link(_)
                | Element::Macros(_)
                | Element::RadioTarget
                | Element::Snippet(_)
                | Element::Target(_)
                | Element::Text { .. }
                | Element::Timestamp(_)
                | Element::Verbatim { .. }
                | Element::FnDef(_)
                | Element::Clock(_)
                | Element::Comment { .. }
                | Element::FixedWidth { .. }
                | Element::Keyword(_)
                | Element::Rule(_)
                | Element::Cookie(_)
                | Element::TableRow(TableRow::BodyRule)
                | Element::TableRow(TableRow::HeaderRule) => {
                    if node.first_child().is_some() {
                        errors.push(ValidationError::UnexpectedChildren { at: node_id });
                    }
                }
                Element::SpecialBlock(_)
                | Element::QuoteBlock(_)
                | Element::CenterBlock(_)
                | Element::VerseBlock(_)
                | Element::Paragraph { .. }
                | Element::Section
                | Element::Bold
                | Element::Italic
                | Element::Underline
                | Element::Strike
                | Element::DynBlock(_)
                | Element::ListItem(_) => {
                    expect_children!(node_id);
                }
                Element::Drawer(_) | Element::TableCell(_) | Element::Table(_) => (),
            }
        }
        errors
    }

    pub(crate) fn debug_validate(&self) {
        if cfg!(debug_assertions) {
            let errors = self.validate();
            if !errors.is_empty() {
                eprintln!("Org validation failed. {} error(s) found:", errors.len());
                for err in errors {
                    eprintln!("{:?} at {:?}", err, err.element(self));
                }
                panic!(
                    "Looks like there's a bug in orgize! Please report it with your org-mode content at https://github.com/PoiScript/orgize/issues."
                );
            }
        }
    }
}
