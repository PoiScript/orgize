use indextree::NodeId;
use std::borrow::Cow;

use crate::config::ParseConfig;
use crate::elements::{Element, Title};
use crate::parsers::{parse_container, Container, OwnedArena};
use crate::{Org, OrgizeError};

#[derive(Copy, Clone, Debug)]
pub struct HeadlineNode {
    pub(crate) node: NodeId,
    pub(crate) level: usize,
    pub(crate) title_node: NodeId,
    pub(crate) section_node: Option<NodeId>,
}

impl HeadlineNode {
    pub(crate) fn new(node: NodeId, level: usize, org: &Org) -> HeadlineNode {
        let title_node = org.arena[node].first_child().unwrap();
        let section_node = if let Some(node) = org.arena[title_node].next_sibling() {
            if let Element::Section = org.arena[node].get() {
                Some(node)
            } else {
                None
            }
        } else {
            None
        };
        HeadlineNode {
            node,
            level,
            title_node,
            section_node,
        }
    }

    pub fn node(self) -> NodeId {
        self.node
    }

    pub fn title_node(self) -> NodeId {
        self.title_node
    }

    pub fn section_node(self) -> Option<NodeId> {
        self.section_node
    }

    pub fn level(self) -> usize {
        self.level
    }

    pub fn title<'a: 'b, 'b>(self, org: &'b Org<'a>) -> &'b Title<'a> {
        if let Element::Title(title) = org.arena[self.title_node].get() {
            title
        } else {
            unreachable!()
        }
    }

    pub fn title_mut<'a: 'b, 'b>(self, org: &'b mut Org<'a>) -> &'b mut Title<'a> {
        if let Element::Title(title) = org.arena[self.title_node].get_mut() {
            title
        } else {
            unreachable!()
        }
    }

    pub fn set_title_content<'a, S: Into<Cow<'a, str>>>(self, content: S, org: &mut Org<'a>) {
        let content = content.into();

        let children: Vec<_> = self.title_node.children(&org.arena).collect();
        for child in children {
            child.detach(&mut org.arena);
        }

        match &content {
            Cow::Borrowed(content) => parse_container(
                &mut org.arena,
                Container::Inline {
                    node: self.title_node,
                    content,
                },
                &ParseConfig::default(),
            ),
            Cow::Owned(ref content) => parse_container(
                &mut OwnedArena::new(&mut org.arena),
                Container::Inline {
                    node: self.title_node,
                    content,
                },
                &ParseConfig::default(),
            ),
        }

        self.title_mut(org).raw = content;

        org.debug_validate();
    }

    pub fn set_section_content<'a, S: Into<Cow<'a, str>>>(self, content: S, org: &mut Org<'a>) {
        let node = if let Some(node) = self.section_node {
            let children: Vec<_> = node.children(&org.arena).collect();
            for child in children {
                child.detach(&mut org.arena);
            }
            node
        } else {
            let node = org.arena.new_node(Element::Section);
            self.node.append(node, &mut org.arena);
            node
        };

        match content.into() {
            Cow::Borrowed(content) => parse_container(
                &mut org.arena,
                Container::Block { node, content },
                &ParseConfig::default(),
            ),
            Cow::Owned(ref content) => parse_container(
                &mut OwnedArena::new(&mut org.arena),
                Container::Block { node, content },
                &ParseConfig::default(),
            ),
        }

        org.debug_validate();
    }

    pub fn parent(self, org: &Org) -> Option<HeadlineNode> {
        org.arena[self.node].parent().map(|node| {
            if let Element::Headline { level } = *org.arena[node].get() {
                HeadlineNode::new(node, level, org)
            } else {
                unreachable!()
            }
        })
    }

    pub fn children<'a>(self, org: &'a Org) -> impl Iterator<Item = HeadlineNode> + 'a {
        self.node.children(&org.arena).filter_map(move |node| {
            if let Element::Headline { level } = *org.arena[node].get() {
                Some(HeadlineNode::new(node, level, org))
            } else {
                None
            }
        })
    }

    pub fn previous_headline(self, org: &Org) -> Option<HeadlineNode> {
        if let Some(node) = org.arena[self.node].previous_sibling() {
            if let Element::Headline { level } = *org.arena[node].get() {
                Some(HeadlineNode::new(node, level, org))
            } else {
                debug_assert_eq!(node, self.section_node.unwrap());
                None
            }
        } else {
            None
        }
    }

    pub fn next_headline(self, org: &Org) -> Option<HeadlineNode> {
        if let Some(node) = org.arena[self.node].next_sibling() {
            if let Element::Headline { level } = *org.arena[node].get() {
                Some(HeadlineNode::new(node, level, org))
            } else {
                unreachable!()
            }
        } else {
            None
        }
    }

    pub fn detach(self, org: &mut Org) {
        self.node.detach(&mut org.arena);

        org.debug_validate();
    }

    pub fn is_detached(self, org: &Org) -> bool {
        self.parent(&org).is_none()
    }

    fn check_level(self, min: usize, max: Option<usize>) -> Result<(), OrgizeError> {
        match max {
            Some(max) if self.level > max || self.level < min => Err(OrgizeError::HeadlineLevel {
                min: Some(min),
                max: Some(max),
                at: self.node,
            }),
            None if self.level < min => Err(OrgizeError::HeadlineLevel {
                min: Some(min),
                max: None,
                at: self.node,
            }),
            _ => Ok(()),
        }
    }

    pub fn append(self, headline: HeadlineNode, org: &mut Org) -> Result<(), OrgizeError> {
        if !headline.is_detached(org) {
            return Err(OrgizeError::Detached { at: headline.node });
        }

        if let Some(last_headline) = org.headlines().last() {
            headline.check_level(self.level + 1, Some(last_headline.level))?;
        } else {
            headline.check_level(self.level + 1, None)?;
        }

        self.node.append(headline.node, &mut org.arena);

        org.debug_validate();

        Ok(())
    }

    pub fn prepend(self, headline: HeadlineNode, org: &mut Org) -> Result<(), OrgizeError> {
        if !headline.is_detached(org) {
            return Err(OrgizeError::Detached { at: headline.node });
        }

        if let Some(first_headline) = self.children(org).next() {
            headline.check_level(first_headline.level, None)?;
        } else {
            headline.check_level(self.level + 1, None)?;
        }

        if let Some(node) = self.section_node {
            node.insert_after(headline.node, &mut org.arena);
        } else {
            self.title_node.insert_after(headline.node, &mut org.arena);
        }

        org.debug_validate();

        Ok(())
    }

    pub fn insert_before(self, headline: HeadlineNode, org: &mut Org) -> Result<(), OrgizeError> {
        if !headline.is_detached(org) {
            return Err(OrgizeError::Detached { at: headline.node });
        }

        if let Some(previous) = self.previous_headline(org) {
            headline.check_level(self.level, Some(previous.level))?;
        } else {
            headline.check_level(self.level, None)?;
        }

        self.node.insert_before(headline.node, &mut org.arena);

        org.debug_validate();

        Ok(())
    }

    pub fn insert_after(self, headline: HeadlineNode, org: &mut Org) -> Result<(), OrgizeError> {
        if !headline.is_detached(org) {
            return Err(OrgizeError::Detached { at: headline.node });
        }

        if let Some(next) = self.next_headline(org) {
            headline.check_level(next.level, Some(self.level))?;
        } else if let Some(parent) = self.parent(org) {
            headline.check_level(parent.level + 1, Some(self.level))?;
        } else {
            headline.check_level(1, Some(self.level))?;
        }

        self.node.insert_after(headline.node, &mut org.arena);

        org.debug_validate();

        Ok(())
    }
}

#[derive(Copy, Clone, Debug)]
pub struct DocumentNode {
    section_node: Option<NodeId>,
}

impl DocumentNode {
    pub(crate) fn new(org: &Org) -> DocumentNode {
        if let Some(node) = org.arena[org.root].first_child() {
            if let Element::Section = org.arena[node].get() {
                DocumentNode {
                    section_node: Some(node),
                }
            } else {
                DocumentNode { section_node: None }
            }
        } else {
            DocumentNode { section_node: None }
        }
    }

    pub fn children<'a>(self, org: &'a Org) -> impl Iterator<Item = HeadlineNode> + 'a {
        org.root.children(&org.arena).filter_map(move |node| {
            if let Element::Headline { level } = *org.arena[node].get() {
                Some(HeadlineNode::new(node, level, org))
            } else {
                None
            }
        })
    }

    pub fn set_section_content<'a, S: Into<Cow<'a, str>>>(self, content: S, org: &mut Org<'a>) {
        let node = if let Some(node) = self.section_node {
            let children: Vec<_> = node.children(&org.arena).collect();
            for child in children {
                child.detach(&mut org.arena);
            }
            node
        } else {
            let node = org.arena.new_node(Element::Section);
            org.root.append(node, &mut org.arena);
            node
        };

        match content.into() {
            Cow::Borrowed(content) => parse_container(
                &mut org.arena,
                Container::Block { node, content },
                &ParseConfig::default(),
            ),
            Cow::Owned(ref content) => parse_container(
                &mut OwnedArena::new(&mut org.arena),
                Container::Block { node, content },
                &ParseConfig::default(),
            ),
        }

        org.debug_validate();
    }

    pub fn append(self, headline: HeadlineNode, org: &mut Org) -> Result<(), OrgizeError> {
        if !headline.is_detached(org) {
            return Err(OrgizeError::Detached { at: headline.node });
        }

        if let Some(last_headline) = org.headlines().last() {
            headline.check_level(1, Some(last_headline.level))?;
        }

        org.root.append(headline.node, &mut org.arena);

        org.debug_validate();

        Ok(())
    }

    pub fn prepend(self, headline: HeadlineNode, org: &mut Org) -> Result<(), OrgizeError> {
        if !headline.is_detached(org) {
            return Err(OrgizeError::Detached { at: headline.node });
        }

        if let Some(first_headline) = self.children(org).next() {
            headline.check_level(first_headline.level, None)?;
        }

        if let Some(node) = self.section_node {
            node.insert_after(headline.node, &mut org.arena);
        } else {
            org.root.prepend(headline.node, &mut org.arena);
        }

        org.debug_validate();

        Ok(())
    }
}
