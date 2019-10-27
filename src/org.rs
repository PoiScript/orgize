use indextree::{Arena, NodeEdge, NodeId};
use std::io::{Error, Write};

use crate::{
    config::{ParseConfig, DEFAULT_CONFIG},
    elements::Element,
    export::{DefaultHtmlHandler, DefaultOrgHandler, HtmlHandler, OrgHandler},
    parsers::{parse_container, Container},
};

pub struct Org<'a> {
    pub(crate) arena: Arena<Element<'a>>,
    pub(crate) root: NodeId,
}

#[derive(Debug)]
pub enum Event<'a, 'b> {
    Start(&'b Element<'a>),
    End(&'b Element<'a>),
}

impl<'a> Org<'a> {
    /// Create a new empty `Org` struct
    pub fn new() -> Org<'static> {
        let mut arena = Arena::new();
        let root = arena.new_node(Element::Document);

        Org { arena, root }
    }

    /// Create a new `Org` struct from parsing `text`, using the default ParseConfig
    pub fn parse(text: &'a str) -> Org<'a> {
        Org::parse_with_config(text, &DEFAULT_CONFIG)
    }

    /// Create a new Org struct from parsing `text`, using a custom ParseConfig
    pub fn parse_with_config(text: &'a str, config: &ParseConfig) -> Org<'a> {
        let mut org = Org::new();

        parse_container(
            &mut org.arena,
            Container::Document {
                content: text,
                node: org.root,
            },
            config,
        );

        org.debug_validate();

        org
    }

    /// Return a refrence to underlay arena
    pub fn arena(&self) -> &Arena<Element<'a>> {
        &self.arena
    }

    /// Return a mutual reference to underlay arena
    pub fn arena_mut(&mut self) -> &mut Arena<Element<'a>> {
        &mut self.arena
    }

    /// Return an iterator of Event
    pub fn iter<'b>(&'b self) -> impl Iterator<Item = Event<'a, 'b>> + 'b {
        self.root.traverse(&self.arena).map(move |edge| match edge {
            NodeEdge::Start(node) => Event::Start(self.arena[node].get()),
            NodeEdge::End(node) => Event::End(self.arena[node].get()),
        })
    }

    pub fn html<W>(&self, writer: W) -> Result<(), Error>
    where
        W: Write,
    {
        self.html_with_handler(writer, &mut DefaultHtmlHandler)
    }

    pub fn html_with_handler<W, H, E>(&self, mut writer: W, handler: &mut H) -> Result<(), E>
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

    pub fn org<W>(&self, writer: W) -> Result<(), Error>
    where
        W: Write,
    {
        self.org_with_handler(writer, &mut DefaultOrgHandler)
    }

    pub fn org_with_handler<W, H, E>(&self, mut writer: W, handler: &mut H) -> Result<(), E>
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

impl Default for Org<'static> {
    fn default() -> Self {
        Org::new()
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
