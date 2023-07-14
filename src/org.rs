use indextree::{Arena, NodeEdge, NodeId};
use std::io::{Error, Write};
use std::ops::{Index, IndexMut};

use crate::{
    config::{ParseConfig, DEFAULT_CONFIG},
    elements::{Element, Keyword},
    export::{DefaultHtmlHandler, DefaultOrgHandler, ExportHandler},
    parsers::{blank_lines_count, parse_container, Container, OwnedArena},
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

    /// Likes `parse`, but accepts `String`.
    pub fn parse_string(text: String) -> Org<'static> {
        Org::parse_string_custom(text, &DEFAULT_CONFIG)
    }

    /// Parses string `text` into `Org` struct with custom `ParseConfig`.
    pub fn parse_custom(text: &'a str, config: &ParseConfig) -> Org<'a> {
        let mut arena = Arena::new();
        let (text, pre_blank) = blank_lines_count(text);
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

    /// Likes `parse_custom`, but accepts `String`.
    pub fn parse_string_custom(text: String, config: &ParseConfig) -> Org<'static> {
        let mut arena = Arena::new();
        let (text, pre_blank) = blank_lines_count(&text);
        let root = arena.new_node(Element::Document { pre_blank });
        let mut org = Org { arena, root };

        parse_container(
            &mut OwnedArena::new(&mut org.arena),
            Container::Document {
                content: text,
                node: org.root,
            },
            config,
        );

        org.debug_validate();

        org
    }

    /// Returns a reference to the underlay arena.
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

    /// Writes the document using the given {ExportHandler}
    pub fn write<W, X, E>(&self, mut writer: W, handler: &mut X) -> Result<(), E>
    where
        W: Write,
        E: From<Error>,
        X: ExportHandler<E>,
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
    pub fn write_html<W>(&self, writer: W) -> Result<(), Error>
    where
        W: Write,
    {
        self.write(writer, &mut DefaultHtmlHandler)
    }

    /// Writes an `Org` struct as org format.
    pub fn write_org<W>(&self, writer: W) -> Result<(), Error>
    where
        W: Write,
    {
        self.write(writer, &mut DefaultOrgHandler)
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
