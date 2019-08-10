use indextree::{Arena, NodeEdge, NodeId};
use std::io::{Error, Write};

use crate::config::ParseConfig;
use crate::elements::Element;
use crate::export::*;
use crate::node::HeadlineNode;
use crate::parsers::{parse_container, Container};

pub struct Org<'a> {
    pub(crate) arena: Arena<Element<'a>>,
    root: NodeId,
}

#[derive(Debug)]
pub enum Event<'a> {
    Start(&'a Element<'a>),
    End(&'a Element<'a>),
}

impl Org<'_> {
    pub fn parse(text: &str) -> Org<'_> {
        Org::parse_with_config(text, &ParseConfig::default())
    }

    pub fn parse_with_config<'a>(content: &'a str, config: &ParseConfig) -> Org<'a> {
        let mut arena = Arena::new();
        let node = arena.new_node(Element::Document);

        parse_container(&mut arena, Container::Document { content, node }, config);

        Org { arena, root: node }
    }

    pub fn iter(&self) -> impl Iterator<Item = Event<'_>> + '_ {
        self.root.traverse(&self.arena).map(move |edge| match edge {
            NodeEdge::Start(e) => Event::Start(self.arena[e].get()),
            NodeEdge::End(e) => Event::End(self.arena[e].get()),
        })
    }

    pub fn headlines(&self) -> impl Iterator<Item = HeadlineNode> + '_ {
        self.root
            .descendants(&self.arena)
            .skip(1)
            .filter_map(move |node| match self.arena[node].get() {
                Element::Headline => Some(HeadlineNode(node)),
                _ => None,
            })
    }

    pub fn html<W: Write>(&self, wrtier: W) -> Result<(), Error> {
        self.html_with_handler(wrtier, DefaultHtmlHandler)
    }

    pub fn html_with_handler<W, H, E>(&self, mut writer: W, mut handler: H) -> Result<(), E>
    where
        W: Write,
        E: From<Error>,
        H: HtmlHandler<E>,
    {
        for event in self.iter() {
            match event {
                Event::Start(element) => handler.start(&mut writer, element)?,
                Event::End(element) => handler.end(&mut writer, element)?,
            }
        }

        Ok(())
    }

    pub fn org<W: Write>(&self, wrtier: W) -> Result<(), Error> {
        self.org_with_handler(wrtier, DefaultOrgHandler)
    }

    pub fn org_with_handler<W, H, E>(&self, mut writer: W, mut handler: H) -> Result<(), E>
    where
        W: Write,
        E: From<Error>,
        H: OrgHandler<E>,
    {
        for event in self.iter() {
            match event {
                Event::Start(element) => handler.start(&mut writer, element)?,
                Event::End(element) => handler.end(&mut writer, element)?,
            }
        }

        Ok(())
    }
}

#[cfg(feature = "ser")]
use serde::{ser::Serializer, Serialize};

#[cfg(feature = "ser")]
impl Serialize for Org<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde_indextree::Node;

        serializer.serialize_newtype_struct("Org", &Node::new(self.root, &self.arena))
    }
}
