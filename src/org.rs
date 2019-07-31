use indextree::{Arena, NodeId};
use jetscii::bytes;
use memchr::{memchr, memchr2, memchr_iter};
use std::io::{Error, Write};

use crate::config::ParseConfig;
use crate::elements::*;
use crate::export::*;
use crate::iter::{Event, Iter};

pub struct Org<'a> {
    pub(crate) arena: Arena<Element<'a>>,
    pub(crate) document: NodeId,
}

enum Container<'a> {
    // List
    List {
        content: &'a str,
        node: NodeId,
        indent: usize,
    },
    // Block, List Item
    Block {
        content: &'a str,
        node: NodeId,
    },
    // Pargraph, Inline Markup
    Inline {
        content: &'a str,
        node: NodeId,
    },
    // Headline
    Headline {
        content: &'a str,
        node: NodeId,
    },
    // Document
    Document {
        content: &'a str,
        node: NodeId,
    },
}

impl<'a> Org<'a> {
    pub fn parse(text: &'a str) -> Self {
        Org::parse_with_config(text, &ParseConfig::default())
    }

    pub fn parse_with_config(content: &'a str, config: &ParseConfig) -> Self {
        let mut arena = Arena::new();
        let document = arena.new_node(Element::Document);

        let mut containers = vec![Container::Document {
            content,
            node: document,
        }];

        while let Some(container) = containers.pop() {
            match container {
                Container::Document {
                    content,
                    node: parent,
                } => {
                    let mut tail = skip_empty_lines(content);
                    if let Some((new_tail, content)) = parse_section(tail) {
                        let node = arena.new_node(Element::Section);
                        parent.append(node, &mut arena).unwrap();
                        containers.push(Container::Block { content, node });
                        tail = new_tail;
                    }
                    while !tail.is_empty() {
                        let (new_tail, content) = parse_headline(tail);
                        let node = arena.new_node(Element::Headline);
                        parent.append(node, &mut arena).unwrap();
                        containers.push(Container::Headline { content, node });
                        tail = new_tail;
                    }
                }
                Container::Headline {
                    content,
                    node: parent,
                } => {
                    let mut tail = content;
                    let (new_tail, title, content) = Title::parse(tail, config);
                    let node = arena.new_node(Element::Title(title));
                    parent.append(node, &mut arena).unwrap();
                    containers.push(Container::Inline { content, node });
                    tail = skip_empty_lines(new_tail);
                    if let Some((new_tail, content)) = parse_section(tail) {
                        let node = arena.new_node(Element::Section);
                        parent.append(node, &mut arena).unwrap();
                        containers.push(Container::Block { content, node });
                        tail = new_tail;
                    }
                    while !tail.is_empty() {
                        let (new_tail, content) = parse_headline(tail);
                        let node = arena.new_node(Element::Headline);
                        parent.append(node, &mut arena).unwrap();
                        containers.push(Container::Headline { content, node });
                        tail = new_tail;
                    }
                }
                Container::Block { content, node } => {
                    parse_blocks(&mut arena, content, node, &mut containers);
                }
                Container::Inline { content, node } => {
                    parse_inlines(&mut arena, content, node, &mut containers);
                }
                Container::List {
                    content,
                    node,
                    indent,
                } => {
                    parse_list_items(&mut arena, content, indent, node, &mut containers);
                }
            }
        }

        Org { arena, document }
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

fn is_headline(text: &str) -> Option<usize> {
    if let Some(off) = memchr2(b'\n', b' ', text.as_bytes()) {
        if off > 0 && text[0..off].as_bytes().iter().all(|&c| c == b'*') {
            Some(off)
        } else {
            None
        }
    } else if text.len() > 0 && text.as_bytes().iter().all(|&c| c == b'*') {
        Some(text.len())
    } else {
        None
    }
}

fn parse_section(text: &str) -> Option<(&str, &str)> {
    if text.is_empty() || is_headline(text).is_some() {
        return None;
    }

    for i in memchr_iter(b'\n', text.as_bytes()) {
        if is_headline(&text[i + 1..]).is_some() {
            return Some((&text[i + 1..], &text[0..i + 1]));
        }
    }

    Some(("", text))
}

fn parse_headline(text: &str) -> (&str, &str) {
    let level = is_headline(text).unwrap();

    for i in memchr_iter(b'\n', text.as_bytes()) {
        if let Some(l) = is_headline(&text[i + 1..]) {
            if l <= level {
                return (&text[i + 1..], &text[0..i + 1]);
            }
        }
    }

    ("", text)
}

fn parse_blocks<'a>(
    arena: &mut Arena<Element<'a>>,
    content: &'a str,
    parent: NodeId,
    containers: &mut Vec<Container<'a>>,
) {
    let mut tail = skip_empty_lines(content);

    if let Some((new_tail, element)) = parse_block(content, arena, containers) {
        parent.append(element, arena).unwrap();
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
            let node = arena.new_node(Element::Paragraph);
            parent.append(node, arena).unwrap();
            containers.push(Container::Inline {
                content: &text[0..pos].trim_end_matches('\n'),
                node,
            });
            text = tail;
            pos = 0;
        } else if let Some((new_tail, element)) = parse_block(tail, arena, containers) {
            if pos != 0 {
                let node = arena.new_node(Element::Paragraph);
                parent.append(node, arena).unwrap();
                containers.push(Container::Inline {
                    content: &text[0..pos].trim_end_matches('\n'),
                    node,
                });
                pos = 0;
            }
            parent.append(element, arena).unwrap();
            tail = skip_empty_lines(new_tail);
            text = tail;
        } else {
            tail = &tail[i..];
            pos += i;
        }
    }

    if !text.is_empty() {
        let node = arena.new_node(Element::Paragraph);
        parent.append(node, arena).unwrap();
        containers.push(Container::Inline {
            content: &text[0..pos].trim_end_matches('\n'),
            node,
        });
    }
}

fn parse_block<'a>(
    contents: &'a str,
    arena: &mut Arena<Element<'a>>,
    containers: &mut Vec<Container<'a>>,
) -> Option<(&'a str, NodeId)> {
    if let Some((tail, fn_def, content)) = FnDef::parse(contents) {
        let node = arena.new_node(Element::FnDef(fn_def));
        containers.push(Container::Block { content, node });
        return Some((tail, node));
    } else if let Some((tail, list, content)) = List::parse(contents) {
        let indent = list.indent;
        let node = arena.new_node(Element::List(list));
        containers.push(Container::List {
            content,
            node,
            indent,
        });
        return Some((tail, node));
    }

    let tail = contents.trim_start();

    if let Some((tail, clock)) = Clock::parse(tail) {
        return Some((tail, arena.new_node(clock)));
    }

    // TODO: LaTeX environment
    if tail.starts_with("\\begin{") {}

    if tail.starts_with('-') {
        if let Ok((tail, rule)) = Rule::parse(tail) {
            return Some((tail, arena.new_node(rule)));
        }
    }

    if tail.starts_with(':') {
        if let Some((tail, drawer, _content)) = Drawer::parse(tail) {
            return Some((tail, arena.new_node(drawer)));
        }
    }

    // FixedWidth
    if tail == ":" || tail.starts_with(": ") || tail.starts_with(":\n") {
        let mut last_end = 1; // ":"
        for i in memchr_iter(b'\n', contents.as_bytes()) {
            last_end = i + 1;
            let line = &contents[last_end..];
            if !(line == ":" || line.starts_with(": ") || line.starts_with(":\n")) {
                let fixed_width = arena.new_node(Element::FixedWidth {
                    value: &contents[0..i + 1],
                });
                return Some((&contents[i + 1..], fixed_width));
            }
        }
        let fixed_width = arena.new_node(Element::FixedWidth {
            value: &contents[0..last_end],
        });
        return Some((&contents[last_end..], fixed_width));
    }

    // Comment
    if tail == "#" || tail.starts_with("# ") || tail.starts_with("#\n") {
        let mut last_end = 1; // "#"
        for i in memchr_iter(b'\n', contents.as_bytes()) {
            last_end = i + 1;
            let line = &contents[last_end..];
            if !(line == "#" || line.starts_with("# ") || line.starts_with("#\n")) {
                let comment = arena.new_node(Element::Comment {
                    value: &contents[0..i + 1],
                });
                return Some((&contents[i + 1..], comment));
            }
        }
        let comment = arena.new_node(Element::Comment {
            value: &contents[0..last_end],
        });
        return Some((&contents[last_end..], comment));
    }

    if tail.starts_with("#+") {
        if let Some((tail, block, content)) = Block::parse(tail) {
            let node = arena.new_node(block);
            containers.push(Container::Block { content, node });
            Some((tail, node))
        } else if let Some((tail, dyn_block, content)) = DynBlock::parse(tail) {
            let node = arena.new_node(dyn_block);
            containers.push(Container::Block { content, node });
            Some((tail, node))
        } else {
            Keyword::parse(tail)
                .ok()
                .map(|(tail, kw)| (tail, arena.new_node(kw)))
        }
    } else {
        None
    }
}

fn parse_inlines<'a>(
    arena: &mut Arena<Element<'a>>,
    content: &'a str,
    parent: NodeId,
    containers: &mut Vec<Container<'a>>,
) {
    let mut tail = content;

    if let Some((new_tail, element)) = parse_inline(tail, arena, containers) {
        parent.append(element, arena).unwrap();
        tail = new_tail;
    }

    let mut text = tail;
    let mut pos = 0;

    let bs = bytes!(b'@', b'<', b'[', b' ', b'(', b'{', b'\'', b'"', b'\n');

    while let Some(off) = bs.find(tail.as_bytes()) {
        match tail.as_bytes()[off] {
            b'{' => {
                if let Some((new_tail, element)) = parse_inline(&tail[off..], arena, containers) {
                    if pos != 0 {
                        let node = arena.new_node(Element::Text {
                            value: &text[0..pos + off],
                        });
                        parent.append(node, arena).unwrap();
                        pos = 0;
                    }
                    parent.append(element, arena).unwrap();
                    tail = new_tail;
                    text = new_tail;
                    continue;
                } else if let Some((new_tail, element)) =
                    parse_inline(&tail[off + 1..], arena, containers)
                {
                    let node = arena.new_node(Element::Text {
                        value: &text[0..pos + off + 1],
                    });
                    parent.append(node, arena).unwrap();
                    pos = 0;
                    parent.append(element, arena).unwrap();
                    tail = new_tail;
                    text = new_tail;
                    continue;
                }
            }
            b' ' | b'(' | b'\'' | b'"' | b'\n' => {
                if let Some((new_tail, element)) = parse_inline(&tail[off + 1..], arena, containers)
                {
                    let node = arena.new_node(Element::Text {
                        value: &text[0..pos + off + 1],
                    });
                    parent.append(node, arena).unwrap();
                    pos = 0;
                    parent.append(element, arena).unwrap();
                    tail = new_tail;
                    text = new_tail;
                    continue;
                }
            }
            _ => {
                if let Some((new_tail, element)) = parse_inline(&tail[off..], arena, containers) {
                    if pos != 0 {
                        let node = arena.new_node(Element::Text {
                            value: &text[0..pos + off],
                        });
                        parent.append(node, arena).unwrap();
                        pos = 0;
                    }
                    parent.append(element, arena).unwrap();
                    tail = new_tail;
                    text = new_tail;
                    continue;
                }
            }
        }
        tail = &tail[off + 1..];
        pos += off + 1;
    }

    if !text.is_empty() {
        let node = arena.new_node(Element::Text { value: text });
        parent.append(node, arena).unwrap();
    }
}

fn parse_inline<'a>(
    contents: &'a str,
    arena: &mut Arena<Element<'a>>,
    containers: &mut Vec<Container<'a>>,
) -> Option<(&'a str, NodeId)> {
    if contents.len() < 3 {
        return None;
    }

    let bytes = contents.as_bytes();
    match bytes[0] {
        b'@' => Snippet::parse(contents)
            .ok()
            .map(|(tail, element)| (tail, arena.new_node(element))),
        b'{' => Macros::parse(contents)
            .ok()
            .map(|(tail, element)| (tail, arena.new_node(element))),
        b'<' => RadioTarget::parse(contents)
            .map(|(tail, (radio, _content))| (tail, radio))
            .or_else(|_| Target::parse(contents))
            .or_else(|_| {
                Timestamp::parse_active(contents).map(|(tail, timestamp)| (tail, timestamp.into()))
            })
            .or_else(|_| {
                Timestamp::parse_diary(contents).map(|(tail, timestamp)| (tail, timestamp.into()))
            })
            .ok()
            .map(|(tail, element)| (tail, arena.new_node(element))),
        b'[' => {
            if contents[1..].starts_with("fn:") {
                FnRef::parse(contents)
                    .map(|(tail, fn_ref)| (tail, fn_ref.into()))
                    .map(|(tail, element)| (tail, arena.new_node(element)))
            } else if bytes[1] == b'[' {
                Link::parse(contents)
                    .ok()
                    .map(|(tail, element)| (tail, arena.new_node(element)))
            } else {
                Cookie::parse(contents)
                    .map(|(tail, cookie)| (tail, cookie.into()))
                    .or_else(|| {
                        Timestamp::parse_inactive(contents)
                            .map(|(tail, timestamp)| (tail, timestamp.into()))
                            .ok()
                    })
                    .map(|(tail, element)| (tail, arena.new_node(element)))
            }
        }
        b'*' => {
            if let Some((tail, content)) = parse_emphasis(contents, b'*') {
                let node = arena.new_node(Element::Bold);
                containers.push(Container::Inline { content, node });
                Some((tail, node))
            } else {
                None
            }
        }
        b'+' => {
            if let Some((tail, content)) = parse_emphasis(contents, b'+') {
                let node = arena.new_node(Element::Strike);
                containers.push(Container::Inline { content, node });
                Some((tail, node))
            } else {
                None
            }
        }
        b'/' => {
            if let Some((tail, content)) = parse_emphasis(contents, b'/') {
                let node = arena.new_node(Element::Italic);
                containers.push(Container::Inline { content, node });
                Some((tail, node))
            } else {
                None
            }
        }
        b'_' => {
            if let Some((tail, content)) = parse_emphasis(contents, b'_') {
                let node = arena.new_node(Element::Underline);
                containers.push(Container::Inline { content, node });
                Some((tail, node))
            } else {
                None
            }
        }
        b'=' => parse_emphasis(contents, b'=')
            .map(|(tail, value)| (tail, arena.new_node(Element::Verbatim { value }))),
        b'~' => parse_emphasis(contents, b'~')
            .map(|(tail, value)| (tail, arena.new_node(Element::Code { value }))),
        b's' => InlineSrc::parse(contents)
            .ok()
            .map(|(tail, element)| (tail, arena.new_node(element))),
        b'c' => InlineCall::parse(contents)
            .ok()
            .map(|(tail, element)| (tail, arena.new_node(element))),
        _ => None,
    }
}

fn parse_list_items<'a>(
    arena: &mut Arena<Element<'a>>,
    mut contents: &'a str,
    indent: usize,
    parent: NodeId,
    containers: &mut Vec<Container<'a>>,
) {
    while !contents.is_empty() {
        let (tail, list_item, content) = ListItem::parse(contents, indent);
        let list_item = Element::ListItem(list_item);
        let node = arena.new_node(list_item);
        parent.append(node, arena).unwrap();
        containers.push(Container::Block { content, node });
        contents = tail;
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
