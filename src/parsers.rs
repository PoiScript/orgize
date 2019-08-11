// parser related functions

use std::borrow::Cow;
use std::marker::PhantomData;

use indextree::{Arena, NodeId};
use jetscii::bytes;
use memchr::{memchr, memchr_iter};
use nom::{
    bytes::complete::take_while1,
    character::complete::{line_ending, not_line_ending},
    combinator::{opt, recognize, verify},
    error::ErrorKind,
    error_position,
    multi::{many0_count, many1_count},
    sequence::terminated,
    Err, IResult,
};

use crate::config::ParseConfig;
use crate::elements::*;

pub trait ElementArena<'a> {
    fn push_element<T: Into<Element<'a>>>(&mut self, element: T, parent: NodeId) -> NodeId;
    fn insert_before_last_child<T: Into<Element<'a>>>(
        &mut self,
        element: T,
        parent: NodeId,
    ) -> NodeId;
}

impl<'a> ElementArena<'a> for Arena<Element<'a>> {
    fn push_element<T: Into<Element<'a>>>(&mut self, element: T, parent: NodeId) -> NodeId {
        let node = self.new_node(element.into());
        parent.append(node, self);
        node
    }

    fn insert_before_last_child<T: Into<Element<'a>>>(
        &mut self,
        element: T,
        parent: NodeId,
    ) -> NodeId {
        if let Some(child) = self[parent].last_child() {
            let node = self.new_node(element.into());
            child.insert_before(node, self);
            node
        } else {
            self.push_element(element, parent)
        }
    }
}

pub struct OwnedArena<'a, 'b, 'c> {
    arena: &'b mut Arena<Element<'c>>,
    phantom: PhantomData<&'a ()>,
}

impl<'a, 'b, 'c> OwnedArena<'a, 'b, 'c> {
    pub fn new(arena: &'b mut Arena<Element<'c>>) -> OwnedArena<'a, 'b, 'c> {
        OwnedArena {
            arena,
            phantom: PhantomData,
        }
    }
}

impl<'a> ElementArena<'a> for OwnedArena<'a, '_, '_> {
    fn push_element<T: Into<Element<'a>>>(&mut self, element: T, parent: NodeId) -> NodeId {
        let node = self.arena.new_node(element.into().into_owned());
        parent.append(node, self.arena);
        node
    }

    fn insert_before_last_child<T: Into<Element<'a>>>(
        &mut self,
        element: T,
        parent: NodeId,
    ) -> NodeId {
        if let Some(child) = self.arena[parent].last_child() {
            let node = self.arena.new_node(element.into().into_owned());
            child.insert_before(node, self.arena);
            node
        } else {
            self.push_element(element, parent)
        }
    }
}

#[derive(Debug)]
pub enum Container<'a> {
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

pub fn parse_container<'a, T: ElementArena<'a>>(
    arena: &mut T,
    container: Container<'a>,
    config: &ParseConfig,
) {
    let containers = &mut vec![container];

    while let Some(container) = containers.pop() {
        match container {
            Container::Document { content, node } => {
                parse_section_and_headlines(arena, content, node, containers);
            }
            Container::Headline { content, node } => {
                parse_headline_content(arena, content, node, containers, config);
            }
            Container::Block { content, node } => {
                parse_blocks(arena, content, node, containers);
            }
            Container::Inline { content, node } => {
                parse_inlines(arena, content, node, containers);
            }
            Container::List {
                content,
                node,
                indent,
            } => {
                parse_list_items(arena, content, indent, node, containers);
            }
        }
    }
}

pub fn parse_headline_content<'a, T: ElementArena<'a>>(
    arena: &mut T,
    content: &'a str,
    parent: NodeId,
    containers: &mut Vec<Container<'a>>,
    config: &ParseConfig,
) {
    let (tail, (title, content)) = Title::parse(content, config).unwrap();
    let node = arena.push_element(title, parent);
    containers.push(Container::Inline { content, node });
    parse_section_and_headlines(arena, tail, parent, containers);
}

pub fn parse_section_and_headlines<'a, T: ElementArena<'a>>(
    arena: &mut T,
    content: &'a str,
    parent: NodeId,
    containers: &mut Vec<Container<'a>>,
) {
    let content = skip_empty_lines(content);
    if content.is_empty() {
        return;
    }

    let mut last_end = 0;
    for i in memchr_iter(b'\n', content.as_bytes()) {
        if let Ok((mut tail, (headline_content, level))) = parse_headline(&content[last_end..]) {
            if last_end != 0 {
                let node = arena.push_element(Element::Section, parent);
                let content = &content[0..last_end];
                containers.push(Container::Block { content, node });
            }

            let node = arena.push_element(Element::Headline { level }, parent);
            containers.push(Container::Headline {
                content: headline_content,
                node,
            });

            while let Ok((new_tail, (content, level))) = parse_headline(tail) {
                debug_assert_ne!(tail, new_tail);
                let node = arena.push_element(Element::Headline { level }, parent);
                containers.push(Container::Headline { content, node });
                tail = new_tail;
            }
            return;
        }
        last_end = i + 1;
    }

    let node = arena.push_element(Element::Section, parent);
    containers.push(Container::Block { content, node });
}

pub fn parse_blocks<'a, T: ElementArena<'a>>(
    arena: &mut T,
    content: &'a str,
    parent: NodeId,
    containers: &mut Vec<Container<'a>>,
) {
    let mut tail = skip_empty_lines(content);

    if let Some(new_tail) = parse_block(content, arena, parent, containers) {
        tail = skip_empty_lines(new_tail);
    }

    let mut text = tail;
    let mut pos = 0;

    macro_rules! insert_paragraph {
        ($content:expr) => {
            let node = arena.insert_before_last_child(Element::Paragraph, parent);
            containers.push(Container::Inline {
                content: $content,
                node,
            });
        };
    }

    while !tail.is_empty() {
        let i = memchr(b'\n', tail.as_bytes())
            .map(|i| i + 1)
            .unwrap_or_else(|| tail.len());
        if tail.as_bytes()[0..i].iter().all(u8::is_ascii_whitespace) {
            debug_assert_ne!(tail, skip_empty_lines(&tail[i..]));
            insert_paragraph!(&text[0..pos].trim_end_matches('\n'));
            pos = 0;
            tail = skip_empty_lines(&tail[i..]);
            text = tail;
        } else if let Some(new_tail) = parse_block(tail, arena, parent, containers) {
            debug_assert_ne!(tail, new_tail);
            if pos != 0 {
                insert_paragraph!(&text[0..pos].trim_end_matches('\n'));
                pos = 0;
            }
            tail = skip_empty_lines(new_tail);
            text = tail;
        } else {
            debug_assert_ne!(tail, &tail[i..]);
            tail = &tail[i..];
            pos += i;
        }
    }

    if !text.is_empty() {
        insert_paragraph!(&text[0..pos].trim_end_matches('\n'));
    }
}

pub fn parse_block<'a, T: ElementArena<'a>>(
    contents: &'a str,
    arena: &mut T,
    parent: NodeId,
    containers: &mut Vec<Container<'a>>,
) -> Option<&'a str> {
    if let Ok((tail, (fn_def, content))) = FnDef::parse(contents) {
        let node = arena.push_element(fn_def, parent);
        containers.push(Container::Block { content, node });
        return Some(tail);
    } else if let Some((tail, list, content)) = List::parse(contents) {
        let indent = list.indent;
        let node = arena.push_element(list, parent);
        containers.push(Container::List {
            content,
            node,
            indent,
        });
        return Some(tail);
    }

    let contents = contents.trim_start();

    match contents.as_bytes().get(0)? {
        b'C' => {
            if let Ok((tail, clock)) = Clock::parse(contents) {
                arena.push_element(clock, parent);
                return Some(tail);
            }
        }
        b'\'' => {
            // TODO: LaTeX environment
        }
        b'-' => {
            if let Ok((tail, _)) = parse_rule(contents) {
                arena.push_element(Element::Rule, parent);
                return Some(tail);
            }
        }
        b':' => {
            if let Ok((tail, (drawer, content))) = Drawer::parse(contents) {
                let node = arena.push_element(drawer, parent);
                containers.push(Container::Block { content, node });
                return Some(tail);
            } else if let Ok((tail, value)) = parse_fixed_width(contents) {
                arena.push_element(
                    Element::FixedWidth {
                        value: value.into(),
                    },
                    parent,
                );
                return Some(tail);
            }
        }
        b'|' => {
            if let Some(tail) = parse_table(arena, contents, containers, parent) {
                return Some(tail);
            }
        }
        b'#' => {
            if let Ok((tail, (name, args, content))) = parse_block_element(contents) {
                match_block(
                    arena,
                    parent,
                    containers,
                    name.into(),
                    args.map(Into::into),
                    content,
                );
                return Some(tail);
            } else if let Ok((tail, (dyn_block, content))) = DynBlock::parse(contents) {
                let node = arena.push_element(dyn_block, parent);
                containers.push(Container::Block { content, node });
                return Some(tail);
            } else if let Ok((tail, (key, optional, value))) = parse_keyword(contents) {
                if (&*key).eq_ignore_ascii_case("CALL") {
                    arena.push_element(
                        BabelCall {
                            value: value.into(),
                        },
                        parent,
                    );
                } else {
                    arena.push_element(
                        Keyword {
                            key: key.into(),
                            optional: optional.map(Into::into),
                            value: value.into(),
                        },
                        parent,
                    );
                }
                return Some(tail);
            } else if let Ok((tail, value)) = parse_comment(contents) {
                arena.push_element(
                    Element::Comment {
                        value: value.into(),
                    },
                    parent,
                );
                return Some(tail);
            }
        }
        _ => (),
    }

    None
}

pub fn match_block<'a, T: ElementArena<'a>>(
    arena: &mut T,
    parent: NodeId,
    containers: &mut Vec<Container<'a>>,
    name: Cow<'a, str>,
    args: Option<Cow<'a, str>>,
    content: &'a str,
) {
    match &*name.to_uppercase() {
        "CENTER" => {
            let node = arena.push_element(CenterBlock { parameters: args }, parent);
            containers.push(Container::Block { content, node });
        }
        "QUOTE" => {
            let node = arena.push_element(QuoteBlock { parameters: args }, parent);
            containers.push(Container::Block { content, node });
        }
        "COMMENT" => {
            arena.push_element(
                CommentBlock {
                    data: args,
                    contents: content.into(),
                },
                parent,
            );
        }
        "EXAMPLE" => {
            arena.push_element(
                ExampleBlock {
                    data: args,
                    contents: content.into(),
                },
                parent,
            );
        }
        "EXPORT" => {
            arena.push_element(
                ExportBlock {
                    data: args.unwrap_or_default(),
                    contents: content.into(),
                },
                parent,
            );
        }
        "SRC" => {
            let (language, arguments) = match &args {
                Some(Cow::Borrowed(args)) => {
                    let (language, arguments) =
                        args.split_at(args.find(' ').unwrap_or_else(|| args.len()));
                    (language.into(), arguments.into())
                }
                None => (Cow::Borrowed(""), Cow::Borrowed("")),
                _ => unreachable!("`parse_block_element` returns `Some(Cow::Borrowed)` or `None`"),
            };
            arena.push_element(
                SourceBlock {
                    arguments,
                    language,
                    contents: content.into(),
                },
                parent,
            );
        }
        "VERSE" => {
            let node = arena.push_element(VerseBlock { parameters: args }, parent);
            containers.push(Container::Block { content, node });
        }
        _ => {
            let node = arena.push_element(
                SpecialBlock {
                    parameters: args,
                    name,
                },
                parent,
            );
            containers.push(Container::Block { content, node });
        }
    }
}

pub fn parse_inlines<'a, T: ElementArena<'a>>(
    arena: &mut T,
    content: &'a str,
    parent: NodeId,
    containers: &mut Vec<Container<'a>>,
) {
    let mut tail = content;

    if let Some(new_tail) = parse_inline(tail, arena, containers, parent) {
        tail = new_tail;
    }

    let mut text = tail;
    let mut pos = 0;

    let bs = bytes!(b'@', b'<', b'[', b' ', b'(', b'{', b'\'', b'"', b'\n');

    macro_rules! insert_text {
        ($value:expr) => {
            arena.insert_before_last_child(
                Element::Text {
                    value: $value.into(),
                },
                parent,
            );
            pos = 0;
        };
    }

    macro_rules! update_tail {
        ($new_tail:ident) => {
            debug_assert_ne!(tail, $new_tail);
            tail = $new_tail;
            text = $new_tail;
        };
    }

    while let Some(off) = bs.find(tail.as_bytes()) {
        match tail.as_bytes()[off] {
            b'{' => {
                if let Some(new_tail) = parse_inline(&tail[off..], arena, containers, parent) {
                    if pos != 0 {
                        insert_text!(&text[0..pos + off]);
                    }
                    update_tail!(new_tail);
                    continue;
                } else if let Some(new_tail) =
                    parse_inline(&tail[off + 1..], arena, containers, parent)
                {
                    insert_text!(&text[0..pos + off + 1]);
                    update_tail!(new_tail);
                    continue;
                }
            }
            b' ' | b'(' | b'\'' | b'"' | b'\n' => {
                if let Some(new_tail) = parse_inline(&tail[off + 1..], arena, containers, parent) {
                    insert_text!(&text[0..pos + off + 1]);
                    update_tail!(new_tail);
                    continue;
                }
            }
            _ => {
                if let Some(new_tail) = parse_inline(&tail[off..], arena, containers, parent) {
                    if pos != 0 {
                        insert_text!(&text[0..pos + off]);
                    }
                    update_tail!(new_tail);
                    continue;
                }
            }
        }
        tail = &tail[off + 1..];
        pos += off + 1;
    }

    if !text.is_empty() {
        arena.push_element(Element::Text { value: text.into() }, parent);
    }
}

pub fn parse_inline<'a, T: ElementArena<'a>>(
    contents: &'a str,
    arena: &mut T,
    containers: &mut Vec<Container<'a>>,
    parent: NodeId,
) -> Option<&'a str> {
    if contents.len() < 3 {
        return None;
    }

    let bytes = contents.as_bytes();
    match bytes[0] {
        b'@' => {
            if let Ok((tail, snippet)) = Snippet::parse(contents) {
                arena.push_element(snippet, parent);
                return Some(tail);
            }
        }
        b'{' => {
            if let Ok((tail, macros)) = Macros::parse(contents) {
                arena.push_element(macros, parent);
                return Some(tail);
            }
        }
        b'<' => {
            if let Ok((tail, _content)) = parse_radio_target(contents) {
                arena.push_element(Element::RadioTarget, parent);
                return Some(tail);
            } else if let Ok((tail, target)) = Target::parse(contents) {
                arena.push_element(target, parent);
                return Some(tail);
            } else if let Ok((tail, timestamp)) = Timestamp::parse_active(contents) {
                arena.push_element(timestamp, parent);
                return Some(tail);
            } else if let Ok((tail, timestamp)) = Timestamp::parse_diary(contents) {
                arena.push_element(timestamp, parent);
                return Some(tail);
            }
        }
        b'[' => {
            if let Ok((tail, fn_ref)) = FnRef::parse(contents) {
                arena.push_element(fn_ref, parent);
                return Some(tail);
            } else if let Ok((tail, link)) = Link::parse(contents) {
                arena.push_element(link, parent);
                return Some(tail);
            } else if let Ok((tail, cookie)) = Cookie::parse(contents) {
                arena.push_element(cookie, parent);
                return Some(tail);
            } else if let Ok((tail, timestamp)) = Timestamp::parse_inactive(contents) {
                arena.push_element(timestamp, parent);
                return Some(tail);
            }
        }
        b'*' => {
            if let Some((tail, content)) = parse_emphasis(contents, b'*') {
                let node = arena.push_element(Element::Bold, parent);
                containers.push(Container::Inline { content, node });
                return Some(tail);
            }
        }
        b'+' => {
            if let Some((tail, content)) = parse_emphasis(contents, b'+') {
                let node = arena.push_element(Element::Strike, parent);
                containers.push(Container::Inline { content, node });
                return Some(tail);
            }
        }
        b'/' => {
            if let Some((tail, content)) = parse_emphasis(contents, b'/') {
                let node = arena.push_element(Element::Italic, parent);
                containers.push(Container::Inline { content, node });
                return Some(tail);
            }
        }
        b'_' => {
            if let Some((tail, content)) = parse_emphasis(contents, b'_') {
                let node = arena.push_element(Element::Underline, parent);
                containers.push(Container::Inline { content, node });
                return Some(tail);
            }
        }
        b'=' => {
            if let Some((tail, value)) = parse_emphasis(contents, b'=') {
                arena.push_element(
                    Element::Verbatim {
                        value: value.into(),
                    },
                    parent,
                );
                return Some(tail);
            }
        }
        b'~' => {
            if let Some((tail, value)) = parse_emphasis(contents, b'~') {
                arena.push_element(
                    Element::Code {
                        value: value.into(),
                    },
                    parent,
                );
                return Some(tail);
            }
        }
        b's' => {
            if let Ok((tail, inline_src)) = InlineSrc::parse(contents) {
                arena.push_element(inline_src, parent);
                return Some(tail);
            }
        }
        b'c' => {
            if let Ok((tail, inline_call)) = InlineCall::parse(contents) {
                arena.push_element(inline_call, parent);
                return Some(tail);
            }
        }
        _ => (),
    }

    None
}

pub fn parse_list_items<'a, T: ElementArena<'a>>(
    arena: &mut T,
    mut contents: &'a str,
    indent: usize,
    parent: NodeId,
    containers: &mut Vec<Container<'a>>,
) {
    while !contents.is_empty() {
        let (tail, list_item, content) = ListItem::parse(contents, indent);
        let node = arena.push_element(list_item, parent);
        containers.push(Container::Block { content, node });
        contents = tail;
    }
}

pub fn parse_table<'a, T: ElementArena<'a>>(
    arena: &mut T,
    contents: &'a str,
    containers: &mut Vec<Container<'a>>,
    parent: NodeId,
) -> Option<&'a str> {
    if contents.trim_start().starts_with('|') {
        let table_node = arena.push_element(Table::Org { tblfm: None }, parent);

        let mut last_end = 0;
        for start in memchr_iter(b'\n', contents.as_bytes()) {
            let line = contents[last_end..start].trim();
            match TableRow::parse(line) {
                Some(TableRow::Standard) => {
                    let row_node = arena.push_element(TableRow::Standard, table_node);
                    for cell in line[1..].split_terminator('|') {
                        let cell_node = arena.push_element(Element::TableCell, row_node);
                        containers.push(Container::Inline {
                            content: cell.trim(),
                            node: cell_node,
                        });
                    }
                }
                Some(TableRow::Rule) => {
                    arena.push_element(TableRow::Rule, table_node);
                }
                None => return Some(&contents[last_end..]),
            }
            last_end = start + 1;
        }

        Some("")
    } else if let Ok((tail, value)) = parse_table_el(contents) {
        arena.push_element(
            Table::TableEl {
                value: value.into(),
            },
            parent,
        );
        Some(tail)
    } else {
        None
    }
}

pub fn line(input: &str) -> IResult<&str, &str> {
    terminated(not_line_ending, opt(line_ending))(input)
}

pub fn eol(input: &str) -> IResult<&str, &str> {
    verify(line, |s: &str| s.trim().is_empty())(input)
}

pub fn take_lines_while(predicate: impl Fn(&str) -> bool) -> impl Fn(&str) -> IResult<&str, &str> {
    move |input| {
        recognize(many0_count(verify(
            |s: &str| {
                // repeat until eof
                if s.is_empty() {
                    Err(Err::Error(error_position!(s, ErrorKind::Eof)))
                } else {
                    line(s)
                }
            },
            |s: &str| predicate(s),
        )))(input)
    }
}

pub fn take_lines_while1(predicate: impl Fn(&str) -> bool) -> impl Fn(&str) -> IResult<&str, &str> {
    move |input| {
        recognize(many1_count(verify(
            |s: &str| {
                // repeat until eof
                if s.is_empty() {
                    Err(Err::Error(error_position!(s, ErrorKind::Eof)))
                } else {
                    line(s)
                }
            },
            |s: &str| predicate(s),
        )))(input)
    }
}

pub fn skip_empty_lines(input: &str) -> &str {
    take_lines_while(|line| line.trim().is_empty())(input)
        .map(|(tail, _)| tail)
        .unwrap_or(input)
}

pub fn parse_headline(input: &str) -> IResult<&str, (&str, usize)> {
    let (input_, level) = parse_headline_level(input)?;
    let (input_, content) = take_lines_while(move |line| {
        if let Ok((_, l)) = parse_headline_level(line) {
            l > level
        } else {
            true
        }
    })(input_)?;
    Ok((input_, (&input[0..level + content.len()], level)))
}

pub fn parse_headline_level(input: &str) -> IResult<&str, usize> {
    let (input, stars) = take_while1(|c: char| c == '*')(input)?;
    if input.is_empty() || input.starts_with(' ') || input.starts_with('\n') {
        Ok((input, stars.len()))
    } else {
        Err(Err::Error(error_position!(input, ErrorKind::Tag)))
    }
}

pub fn parse_fixed_width(input: &str) -> IResult<&str, &str> {
    take_lines_while1(|line| line == ":" || line.starts_with(": "))(input)
}

pub fn parse_comment(input: &str) -> IResult<&str, &str> {
    take_lines_while1(|line| line == "#" || line.starts_with("# "))(input)
}

pub fn take_one_word(input: &str) -> IResult<&str, &str> {
    take_while1(|c: char| !c.is_ascii_whitespace())(input)
}

#[test]
pub fn test_skip_empty_lines() {
    assert_eq!(skip_empty_lines("foo"), "foo");
    assert_eq!(skip_empty_lines(" foo"), " foo");
    assert_eq!(skip_empty_lines(" \nfoo\n"), "foo\n");
    assert_eq!(skip_empty_lines(" \n\n\nfoo\n"), "foo\n");
    assert_eq!(skip_empty_lines(" \n  \n\nfoo\n"), "foo\n");
    assert_eq!(skip_empty_lines(" \n  \n\n   foo\n"), "   foo\n");
}
