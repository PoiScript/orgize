use indextree::{Arena, NodeId};
use jetscii::bytes;
use memchr::{memchr, memchr_iter};
use std::io::{Error, Write};

use crate::config::ParseConfig;
use crate::elements::*;
use crate::export::{DefaultHtmlHandler, HtmlHandler};
use crate::iter::Iter;

pub struct Org<'a> {
    pub(crate) arena: Arena<Element<'a>>,
    pub(crate) document: NodeId,
}

impl<'a> Org<'a> {
    pub fn parse(text: &'a str) -> Self {
        Org::parse_with_config(text, ParseConfig::default())
    }

    pub fn parse_with_config(text: &'a str, config: ParseConfig<'_>) -> Self {
        let mut arena = Arena::new();
        let document = arena.new_node(Element::Document { contents: text });

        let mut org = Org { arena, document };
        org.parse_internal(config);

        org
    }

    pub fn iter(&'a self) -> Iter<'a> {
        Iter {
            arena: &self.arena,
            traverse: self.document.traverse(&self.arena),
        }
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
        use crate::iter::Event::*;

        for event in self.iter() {
            match event {
                Start(e) => handler.start(&mut writer, e)?,
                End(e) => handler.end(&mut writer, e)?,
            }
        }

        Ok(())
    }

    fn parse_internal(&mut self, config: ParseConfig<'_>) {
        let mut node = self.document;
        loop {
            match self.arena[node].data {
                Element::Document { mut contents }
                | Element::Headline(Headline { mut contents, .. }) => {
                    if !contents.is_empty() {
                        let off = Headline::find_level(contents, std::usize::MAX);
                        if off != 0 {
                            let section = Element::Section {
                                contents: &contents[0..off],
                            };
                            let new_node = self.arena.new_node(section);
                            node.append(new_node, &mut self.arena).unwrap();
                            contents = &contents[off..];
                        }
                    }
                    while !contents.is_empty() {
                        let (tail, headline) = Headline::parse(contents, &config);
                        let headline = Element::Headline(headline);
                        let new_node = self.arena.new_node(headline);
                        node.append(new_node, &mut self.arena).unwrap();
                        contents = tail;
                    }
                }
                Element::Section { contents } => {
                    // TODO
                    if let Some((tail, _planning)) = Planning::parse(contents) {
                        self.parse_elements_children(tail, node);
                    } else {
                        self.parse_elements_children(contents, node);
                    }
                }
                Element::Block(Block { contents, .. })
                | Element::ListItem(ListItem { contents, .. }) => {
                    self.parse_elements_children(contents, node);
                }
                Element::Paragraph { contents }
                | Element::Bold { contents }
                | Element::Underline { contents }
                | Element::Italic { contents }
                | Element::Strike { contents } => {
                    self.parse_objects_children(contents, node);
                }
                Element::List(List {
                    contents, indent, ..
                }) => {
                    self.parse_list_items(contents, indent, node);
                }
                _ => (),
            }

            if let Some(next_node) = self.next_node(node) {
                node = next_node;
            } else {
                break;
            }
        }
    }

    fn next_node(&self, mut node: NodeId) -> Option<NodeId> {
        if let Some(child) = self.arena[node].first_child() {
            return Some(child);
        }

        loop {
            if let Some(sibling) = self.arena[node].next_sibling() {
                return Some(sibling);
            } else if let Some(parent) = self.arena[node].parent() {
                node = parent;
            } else {
                return None;
            }
        }
    }

    fn parse_elements_children(&mut self, input: &'a str, node: NodeId) {
        let mut tail = skip_empty_lines(input);

        if let Some((new_tail, element)) = self.parse_element(input) {
            let new_node = self.arena.new_node(element);
            node.append(new_node, &mut self.arena).unwrap();
            tail = skip_empty_lines(new_tail);
        }

        let mut text = tail;
        let mut pos = 0;

        while !tail.is_empty() {
            let i = memchr(b'\n', tail.as_bytes())
                .map(|i| i + 1)
                .unwrap_or_else(|| tail.len());
            if tail.as_bytes()[0..i].iter().all(u8::is_ascii_whitespace) {
                tail = skip_empty_lines(&tail[i..]);
                let new_node = self.arena.new_node(Element::Paragraph {
                    contents: if text.as_bytes()[pos - 1] == b'\n' {
                        &text[0..pos - 1]
                    } else {
                        &text[0..pos]
                    },
                });
                node.append(new_node, &mut self.arena).unwrap();
                text = tail;
                pos = 0;
            } else if let Some((new_tail, element)) = self.parse_element(tail) {
                if pos != 0 {
                    let new_node = self.arena.new_node(Element::Paragraph {
                        contents: if text.as_bytes()[pos - 1] == b'\n' {
                            &text[0..pos - 1]
                        } else {
                            &text[0..pos]
                        },
                    });
                    node.append(new_node, &mut self.arena).unwrap();
                    pos = 0;
                }
                let new_node = self.arena.new_node(element);
                node.append(new_node, &mut self.arena).unwrap();
                tail = skip_empty_lines(new_tail);
                text = tail;
            } else {
                tail = &tail[i..];
                pos += i;
            }
        }

        if !text.is_empty() {
            let new_node = self.arena.new_node(Element::Paragraph {
                contents: if text.as_bytes()[pos - 1] == b'\n' {
                    &text[0..pos - 1]
                } else {
                    &text[0..pos]
                },
            });
            node.append(new_node, &mut self.arena).unwrap();
        }
    }

    fn parse_element(&self, contents: &'a str) -> Option<(&'a str, Element<'a>)> {
        if let Some((tail, fn_def)) = FnDef::parse(contents) {
            let fn_def = Element::FnDef(fn_def);
            return Some((tail, fn_def));
        } else if let Some((tail, list)) = List::parse(contents) {
            let list = Element::List(list);
            return Some((tail, list));
        }

        let tail = contents.trim_start();

        if let Some((tail, clock)) = Clock::parse(tail) {
            return Some((tail, clock));
        }

        // TODO: LaTeX environment
        if tail.starts_with("\\begin{") {}

        if tail.starts_with('-') {
            if let Ok((tail, rule)) = Rule::parse(tail) {
                return Some((tail, rule));
            }
        }

        if tail.starts_with(':') {
            if let Some((tail, drawer)) = Drawer::parse(tail) {
                return Some((tail, drawer));
            }
        }

        if tail == ":" || tail.starts_with(": ") || tail.starts_with(":\n") {
            let mut last_end = 1; // ":"
            for i in memchr_iter(b'\n', contents.as_bytes()) {
                last_end = i + 1;
                let line = &contents[last_end..];
                if !(line == ":" || line.starts_with(": ") || line.starts_with(":\n")) {
                    let fixed_width = Element::FixedWidth {
                        value: &contents[0..i + 1],
                    };
                    return Some((&contents[i + 1..], fixed_width));
                }
            }
            let fixed_width = Element::FixedWidth {
                value: &contents[0..last_end],
            };
            return Some((&contents[last_end..], fixed_width));
        }

        if tail == "#" || tail.starts_with("# ") || tail.starts_with("#\n") {
            let mut last_end = 1; // "#"
            for i in memchr_iter(b'\n', contents.as_bytes()) {
                last_end = i + 1;
                let line = &contents[last_end..];
                if !(line == "#" || line.starts_with("# ") || line.starts_with("#\n")) {
                    let fixed_width = Element::Comment {
                        value: &contents[0..i + 1],
                    };
                    return Some((&contents[i + 1..], fixed_width));
                }
            }
            let fixed_width = Element::Comment {
                value: &contents[0..last_end],
            };
            return Some((&contents[last_end..], fixed_width));
        }

        if tail.starts_with("#+") {
            Block::parse(tail)
                .or_else(|| DynBlock::parse(tail))
                .or_else(|| Keyword::parse(tail).ok())
        } else {
            None
        }
    }

    fn parse_objects_children(&mut self, contents: &'a str, node: NodeId) {
        let mut tail = contents;

        if let Some((new_tail, obj)) = self.parse_object(tail) {
            let new_node = self.arena.new_node(obj);
            node.append(new_node, &mut self.arena).unwrap();
            tail = new_tail;
        }

        let mut text = tail;
        let mut pos = 0;

        let bs = bytes!(b'@', b'<', b'[', b' ', b'(', b'{', b'\'', b'"', b'\n');

        while let Some(off) = bs.find(tail.as_bytes()) {
            match tail.as_bytes()[off] {
                b'{' => {
                    if let Some((new_tail, obj)) = self.parse_object(&tail[off..]) {
                        if pos != 0 {
                            let new_node = self.arena.new_node(Element::Text {
                                value: &text[0..pos + off],
                            });
                            node.append(new_node, &mut self.arena).unwrap();
                            pos = 0;
                        }
                        let new_node = self.arena.new_node(obj);
                        node.append(new_node, &mut self.arena).unwrap();
                        tail = new_tail;
                        text = new_tail;
                    } else if let Some((new_tail, obj)) = self.parse_object(&tail[off + 1..]) {
                        let new_node = self.arena.new_node(Element::Text {
                            value: &text[0..pos + off + 1],
                        });
                        node.append(new_node, &mut self.arena).unwrap();
                        pos = 0;
                        let new_node = self.arena.new_node(obj);
                        node.append(new_node, &mut self.arena).unwrap();
                        tail = new_tail;
                        text = new_tail;
                    } else {
                        tail = &tail[off + 1..];
                        pos += off + 1;
                    }
                }
                b' ' | b'(' | b'\'' | b'"' | b'\n' => {
                    if let Some((new_tail, obj)) = self.parse_object(&tail[off + 1..]) {
                        let new_node = self.arena.new_node(Element::Text {
                            value: &text[0..pos + off + 1],
                        });
                        node.append(new_node, &mut self.arena).unwrap();
                        pos = 0;
                        let new_node = self.arena.new_node(obj);
                        node.append(new_node, &mut self.arena).unwrap();
                        tail = new_tail;
                        text = new_tail;
                    } else {
                        tail = &tail[off + 1..];
                        pos += off + 1;
                    }
                }
                _ => {
                    if let Some((new_tail, obj)) = self.parse_object(&tail[off..]) {
                        if pos != 0 {
                            let new_node = self.arena.new_node(Element::Text {
                                value: &text[0..pos + off],
                            });
                            node.append(new_node, &mut self.arena).unwrap();
                            pos = 0;
                        }
                        let new_node = self.arena.new_node(obj);
                        node.append(new_node, &mut self.arena).unwrap();
                        tail = new_tail;
                        text = new_tail;
                    } else {
                        tail = &tail[off + 1..];
                        pos += off + 1;
                    }
                }
            }
        }

        if !text.is_empty() {
            let new_node = self.arena.new_node(Element::Text { value: text });
            node.append(new_node, &mut self.arena).unwrap();
        }
    }

    fn parse_object(&self, contents: &'a str) -> Option<(&'a str, Element<'a>)> {
        if contents.len() < 3 {
            return None;
        }

        let bytes = contents.as_bytes();
        match bytes[0] {
            b'@' => Snippet::parse(contents).ok(),
            b'{' => Macros::parse(contents).ok(),
            b'<' => RadioTarget::parse(contents)
                .or_else(|_| Target::parse(contents))
                .or_else(|_| {
                    Timestamp::parse_active(contents)
                        .map(|(tail, timestamp)| (tail, timestamp.into()))
                })
                .or_else(|_| {
                    Timestamp::parse_diary(contents)
                        .map(|(tail, timestamp)| (tail, timestamp.into()))
                })
                .ok(),
            b'[' => {
                if contents[1..].starts_with("fn:") {
                    FnRef::parse(contents).map(|(tail, fn_ref)| (tail, fn_ref.into()))
                } else if bytes[1] == b'[' {
                    Link::parse(contents).ok()
                } else {
                    Cookie::parse(contents)
                        .map(|(tail, cookie)| (tail, cookie.into()))
                        .or_else(|| {
                            Timestamp::parse_inactive(contents)
                                .map(|(tail, timestamp)| (tail, timestamp.into()))
                                .ok()
                        })
                }
            }
            b'*' => parse_emphasis(contents, b'*')
                .map(|(tail, contents)| (tail, Element::Bold { contents })),
            b'+' => parse_emphasis(contents, b'+')
                .map(|(tail, contents)| (tail, Element::Strike { contents })),
            b'/' => parse_emphasis(contents, b'/')
                .map(|(tail, contents)| (tail, Element::Italic { contents })),
            b'_' => parse_emphasis(contents, b'_')
                .map(|(tail, contents)| (tail, Element::Underline { contents })),
            b'=' => parse_emphasis(contents, b'=')
                .map(|(tail, value)| (tail, Element::Verbatim { value })),
            b'~' => {
                parse_emphasis(contents, b'~').map(|(tail, value)| (tail, Element::Code { value }))
            }
            b's' if contents.starts_with("src_") => InlineSrc::parse(contents).ok(),
            b'c' if contents.starts_with("call_") => InlineCall::parse(contents).ok(),
            _ => None,
        }
    }

    fn parse_list_items(&mut self, mut contents: &'a str, indent: usize, node: NodeId) {
        while !contents.is_empty() {
            let (tail, list_item) = ListItem::parse(contents, indent);
            let list_item = Element::ListItem(list_item);
            let new_node = self.arena.new_node(list_item);
            node.append(new_node, &mut self.arena).unwrap();
            contents = tail;
        }
    }
}

fn skip_empty_lines(contents: &str) -> &str {
    let mut i = 0;
    for pos in memchr_iter(b'\n', contents.as_bytes()) {
        if contents.as_bytes()[i..pos]
            .iter()
            .all(u8::is_ascii_whitespace)
        {
            i = pos + 1;
        } else {
            break;
        }
    }
    &contents[i..]
}

#[test]
fn test_skip_empty_lines() {
    assert_eq!(skip_empty_lines("foo"), "foo");
    assert_eq!(skip_empty_lines(" foo"), " foo");
    assert_eq!(skip_empty_lines(" \nfoo\n"), "foo\n");
    assert_eq!(skip_empty_lines(" \n\n\nfoo\n"), "foo\n");
    assert_eq!(skip_empty_lines(" \n  \n\nfoo\n"), "foo\n");
    assert_eq!(skip_empty_lines(" \n  \n\n   foo\n"), "   foo\n");
}
