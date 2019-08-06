use indextree::{Arena, NodeEdge, NodeId};
use std::io::{Error, Write};

use crate::config::ParseConfig;
use crate::elements::*;
use crate::export::*;
use crate::parsers::*;

pub struct Org<'a> {
    pub(crate) arena: Arena<Element<'a>>,
    pub(crate) document: NodeId,
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
        let document = arena.new_node(Element::Document);

        let containers = &mut vec![Container::Document {
            content,
            node: document,
        }];

        while let Some(container) = containers.pop() {
            match container {
                Container::Document { content, node } => {
                    parse_section_and_headlines(&mut arena, content, node, containers);
                }
                Container::Headline { content, node } => {
                    let content = parse_title(&mut arena, content, node, containers, config);
                    parse_section_and_headlines(&mut arena, content, node, containers);
                }
                Container::Block { content, node } => {
                    parse_blocks(&mut arena, content, node, containers);
                }
                Container::Inline { content, node } => {
                    parse_inlines(&mut arena, content, node, containers);
                }
                Container::List {
                    content,
                    node,
                    indent,
                } => {
                    parse_list_items(&mut arena, content, indent, node, containers);
                }
            }
        }

        Org { arena, document }
    }

    pub fn iter<'a>(&'a self) -> impl Iterator<Item = Event<'_>> + 'a {
        self.document
            .traverse(&self.arena)
            .map(move |edge| match edge {
                NodeEdge::Start(e) => Event::Start(&self.arena[e].data),
                NodeEdge::End(e) => Event::End(&self.arena[e].data),
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
