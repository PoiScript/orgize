use indextree::NodeId;
use std::borrow::Cow;

use crate::config::ParseConfig;
use crate::elements::{Element, Title};
use crate::parsers::{parse_container, Container, OwnedArena};
use crate::Org;

#[derive(Copy, Clone, Debug)]
pub struct HeadlineNode {
    pub(crate) node: NodeId,
    pub(crate) level: usize,
    pub(crate) title_node: NodeId,
    pub(crate) section_node: Option<NodeId>,
}

impl<'a: 'b, 'b> HeadlineNode {
    pub(crate) fn new(node: NodeId, level: usize, org: &Org<'_>) -> HeadlineNode {
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

    pub fn level(self) -> usize {
        self.level
    }

    pub fn title(self, org: &'b Org<'a>) -> &'b Title<'a> {
        if let Element::Title(title) = org.arena[self.title_node].get() {
            title
        } else {
            unreachable!()
        }
    }

    pub fn title_mut(self, org: &'b mut Org<'a>) -> &'b mut Title<'a> {
        if let Element::Title(title) = org.arena[self.title_node].get_mut() {
            title
        } else {
            unreachable!()
        }
    }

    pub fn set_title_content<S: Into<Cow<'a, str>>>(self, content: S, org: &mut Org<'a>) {
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
    }

    pub fn set_section_content<S: Into<Cow<'a, str>>>(self, content: S, org: &mut Org<'a>) {
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
    }

    pub fn parent(self, org: &Org<'_>) -> Option<HeadlineNode> {
        org.arena[self.node].parent().map(|node| {
            if let &Element::Headline { level } = org.arena[node].get() {
                HeadlineNode::new(node, level, org)
            } else {
                unreachable!()
            }
        })
    }

    pub fn detach(self, org: &mut Org<'_>) {
        self.node.detach(&mut org.arena);
    }

    pub fn is_detached(self, org: &Org<'_>) -> bool {
        self.parent(&org).is_none()
    }

    pub fn append(self, headline: &HeadlineNode, org: &mut Org<'_>) {
        if self.is_detached(org) || headline.level <= self.level {
            // TODO: return an error
            return;
        } else {
            self.node.append(headline.node, &mut org.arena);
        }
    }

    pub fn prepend(self, headline: &HeadlineNode, org: &mut Org<'_>) {
        if self.is_detached(org) || headline.level <= self.level {
            // TODO: return an error
            return;
        } else if let Some(node) = self.section_node {
            node.insert_after(headline.node, &mut org.arena);
        } else {
            self.title_node.insert_after(headline.node, &mut org.arena);
        }
    }

    pub fn insert_before(self, headline: &HeadlineNode, org: &mut Org<'_>) {
        if self.is_detached(org) || headline.level < self.level {
            // TODO: return an error
            return;
        } else {
            self.node.insert_after(headline.node, &mut org.arena);
        }
    }

    pub fn insert_after(self, headline: &HeadlineNode, org: &mut Org<'_>) {
        if self.is_detached(org) || headline.level < self.level {
            // TODO: return an error
            return;
        } else {
            self.node.insert_after(headline.node, &mut org.arena);
        }
    }
}
