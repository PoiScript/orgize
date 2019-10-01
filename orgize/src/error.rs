use indextree::NodeId;

use crate::elements::*;
use crate::Org;

/// Orgize Validation Error
#[derive(Debug)]
pub enum OrgizeError {
    /// Expect this node has children
    Children { at: NodeId },
    /// Expect this node has no children
    NoChildren { at: NodeId },
    /// Expect this node contains a headline or section element
    HeadlineOrSection { at: NodeId },
    /// Expect this node contains a title element
    Title { at: NodeId },
    /// Expect this node contains a headline element
    Headline { at: NodeId },
    /// Expect a detached headline
    Detached { at: NodeId },
    /// Expect a headline where its level >= max and <= min
    HeadlineLevel {
        max: Option<usize>,
        min: Option<usize>,
        at: NodeId,
    },
}

impl OrgizeError {
    pub fn element<'a, 'b>(&self, org: &'a Org<'b>) -> &'a Element<'b> {
        match &self {
            OrgizeError::Children { at }
            | OrgizeError::NoChildren { at }
            | OrgizeError::HeadlineOrSection { at }
            | OrgizeError::Title { at }
            | OrgizeError::Headline { at }
            | OrgizeError::Detached { at }
            | OrgizeError::HeadlineLevel { at, .. } => org.arena[*at].get(),
        }
    }
}

impl Org<'_> {
    /// Validate an `Org` struct.
    pub fn validate(&self) -> Result<(), OrgizeError> {
        for node_id in self.root.descendants(&self.arena) {
            let node = &self.arena[node_id];
            match node.get() {
                Element::Document => {
                    for child_id in node_id.children(&self.arena) {
                        match self.arena[child_id].get() {
                            Element::Headline { .. } | Element::Section => (),
                            _ => return Err(OrgizeError::HeadlineOrSection { at: child_id }),
                        }
                    }
                }
                Element::Headline { .. } => {
                    if node.first_child().is_none() {
                        return Err(OrgizeError::Children { at: node_id });
                    }
                    let title = node.first_child().unwrap();
                    match self.arena[title].get() {
                        Element::Title(Title { .. }) => (),
                        _ => return Err(OrgizeError::Title { at: title }),
                    }
                    if let Some(next) = self.arena[title].next_sibling() {
                        match self.arena[next].get() {
                            Element::Headline { .. } | Element::Section => (),
                            _ => return Err(OrgizeError::HeadlineOrSection { at: next }),
                        }

                        for sibling in next.following_siblings(&self.arena).skip(1) {
                            match self.arena[sibling].get() {
                                Element::Headline { .. } => (),
                                _ => return Err(OrgizeError::Headline { at: sibling }),
                            }
                        }
                    }
                }
                Element::Title(Title { raw, .. }) => {
                    if !raw.is_empty() && node.first_child().is_none() {
                        return Err(OrgizeError::Children { at: node_id });
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
                | Element::Rule
                | Element::Cookie(_)
                | Element::Table(Table::TableEl { .. })
                | Element::TableRow(TableRow::Rule) => {
                    if node.first_child().is_some() {
                        return Err(OrgizeError::NoChildren { at: node_id });
                    }
                }
                Element::SpecialBlock(_)
                | Element::QuoteBlock(_)
                | Element::CenterBlock(_)
                | Element::VerseBlock(_)
                | Element::Paragraph
                | Element::Section
                | Element::Table(Table::Org { .. })
                | Element::TableRow(TableRow::Standard)
                | Element::Bold
                | Element::Italic
                | Element::Underline
                | Element::Strike
                | Element::DynBlock(_)
                | Element::List(_)
                | Element::ListItem(_) => {
                    if node.first_child().is_none() {
                        return Err(OrgizeError::Children { at: node_id });
                    }
                }
                // TableCell is a container but it might
                // not contains anything, e.g. `||||||`
                Element::Drawer(_) | Element::TableCell => (),
            }
        }
        Ok(())
    }

    #[deprecated(since = "0.3.1", note = "rename to validate")]
    /// Validate an `Org` struct.
    pub fn check(&self) -> Result<(), OrgizeError> {
        self.validate()
    }

    pub(crate) fn debug_validate(&self) {
        if cfg!(debug_assertions) {
            if let Err(err) = self.validate() {
                panic!(
                    "Validation error: {:?} at element: {:?}",
                    err,
                    err.element(self)
                );
            }
        }
    }
}
