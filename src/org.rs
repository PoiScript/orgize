use indextree::{Arena, NodeEdge, NodeId};
use std::io::{Error, Write};
use std::ops::{Index, IndexMut};

use crate::{
    config::{ParseConfig, DEFAULT_CONFIG},
    elements::{Element, Keyword},
    export::{DefaultHtmlHandler, DefaultOrgHandler, HtmlHandler, OrgHandler},
    parsers::{blank_lines, parse_container, Container},
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
    /// Creates a new empty `Org` struct.
    pub fn new() -> Org<'static> {
        let mut arena = Arena::new();
        let root = arena.new_node(Element::Document { pre_blank: 0 });
        Org { arena, root }
    }

    /// Parses string `text` into `Org` struct.
    pub fn parse(text: &'a str) -> Org<'a> {
        Org::parse_custom(text, &DEFAULT_CONFIG)
    }

    /// Parses string `text` into `Org` struct with custom `ParseConfig`.
    pub fn parse_custom(text: &'a str, config: &ParseConfig) -> Org<'a> {
        let mut arena = Arena::new();
        let (text, pre_blank) = blank_lines(text);
        let root = arena.new_node(Element::Document { pre_blank });
        let mut org = Org { arena, root };

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

    /// Parses string `text` into `Org` struct with custom `ParseConfig`.
    #[deprecated(since = "0.6.0", note = "rename to parse_custom")]
    pub fn parse_with_config(text: &'a str, config: &ParseConfig) -> Org<'a> {
        Org::parse_custom(text, config)
    }

    /// Returns a refrence to the underlay arena.
    pub fn arena(&self) -> &Arena<Element<'a>> {
        &self.arena
    }

    /// Returns a mutual reference to the underlay arena.
    pub fn arena_mut(&mut self) -> &mut Arena<Element<'a>> {
        &mut self.arena
    }

    /// Returns an iterator of `Event`s.
    pub fn iter<'b>(&'b self) -> impl Iterator<Item = Event<'a, 'b>> + 'b {
        self.root.traverse(&self.arena).map(move |edge| match edge {
            NodeEdge::Start(node) => Event::Start(&self[node]),
            NodeEdge::End(node) => Event::End(&self[node]),
        })
    }

    /// Returns an iterator of `Keyword`s.
    pub fn keywords(&self) -> impl Iterator<Item = &Keyword<'_>> {
        self.root
            .descendants(&self.arena)
            .skip(1)
            .filter_map(move |node| match &self[node] {
                Element::Keyword(kw) => Some(kw),
                _ => None,
            })
    }

    /// Writes an `Org` struct as html format.
    pub fn write_html<W>(&self, writer: W) -> Result<(), Error>
    where
        W: Write,
    {
        self.write_html_custom(writer, &mut DefaultHtmlHandler)
    }

    /// Writes an `Org` struct as html format with custom `HtmlHandler`.
    pub fn write_html_custom<W, H, E>(&self, mut writer: W, handler: &mut H) -> Result<(), E>
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

    /// Writes an `Org` struct as html format.
    #[deprecated(since = "0.6.0", note = "rename to write_html")]
    pub fn html<W>(&self, writer: W) -> Result<(), Error>
    where
        W: Write,
    {
        self.write_html_custom(writer, &mut DefaultHtmlHandler)
    }

    /// Writes an `Org` struct as html format with custom `HtmlHandler`.
    #[deprecated(since = "0.6.0", note = "rename to write_html_custom")]
    pub fn html_with_handler<W, H, E>(&self, writer: W, handler: &mut H) -> Result<(), E>
    where
        W: Write,
        E: From<Error>,
        H: HtmlHandler<E>,
    {
        self.write_html_custom(writer, handler)
    }

    /// Writes an `Org` struct as org format.
    pub fn write_org<W>(&self, writer: W) -> Result<(), Error>
    where
        W: Write,
    {
        self.write_org_custom(writer, &mut DefaultOrgHandler)
    }

    /// Writes an `Org` struct as org format with custom `OrgHandler`.
    pub fn write_org_custom<W, H, E>(&self, mut writer: W, handler: &mut H) -> Result<(), E>
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

    /// Writes an `Org` struct as org format.
    #[deprecated(since = "0.6.0", note = "rename to write_org")]
    pub fn org<W>(&self, writer: W) -> Result<(), Error>
    where
        W: Write,
    {
        self.write_org_custom(writer, &mut DefaultOrgHandler)
    }

    /// Writes an `Org` struct as org format with custom `OrgHandler`.
    #[deprecated(since = "0.6.0", note = "rename to write_org_custom")]
    pub fn org_with_handler<W, H, E>(&self, writer: W, handler: &mut H) -> Result<(), E>
    where
        W: Write,
        E: From<Error>,
        H: OrgHandler<E>,
    {
        self.write_org_custom(writer, handler)
    }
}

impl Default for Org<'static> {
    fn default() -> Self {
        Org::new()
    }
}

impl<'a> Index<NodeId> for Org<'a> {
    type Output = Element<'a>;

    fn index(&self, node_id: NodeId) -> &Self::Output {
        self.arena[node_id].get()
    }
}

impl<'a> IndexMut<NodeId> for Org<'a> {
    fn index_mut(&mut self, node_id: NodeId) -> &mut Self::Output {
        self.arena[node_id].get_mut()
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
