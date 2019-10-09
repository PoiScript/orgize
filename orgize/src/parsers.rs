// parser related functions

use std::borrow::Cow;
use std::iter::once;
use std::marker::PhantomData;

use indextree::{Arena, NodeId};
use jetscii::{bytes, BytesConst};
use memchr::{memchr, memchr_iter};
use nom::{bytes::complete::take_while1, combinator::verify, error::ParseError, IResult};

use crate::config::ParseConfig;
use crate::elements::{
    block::parse_block_element, emphasis::parse_emphasis, keyword::parse_keyword,
    radio_target::parse_radio_target, rule::parse_rule, table::parse_table_el, BabelCall,
    CenterBlock, Clock, CommentBlock, Cookie, Drawer, DynBlock, Element, ExampleBlock, ExportBlock,
    FnDef, FnRef, InlineCall, InlineSrc, Keyword, Link, List, ListItem, Macros, QuoteBlock,
    Snippet, SourceBlock, SpecialBlock, Table, TableRow, Target, Timestamp, Title, VerseBlock,
};

pub trait ElementArena<'a> {
    fn append_element<T: Into<Element<'a>>>(&mut self, element: T, parent: NodeId) -> NodeId;
    fn insert_before_last_child<T: Into<Element<'a>>>(
        &mut self,
        element: T,
        parent: NodeId,
    ) -> NodeId;
}

impl<'a> ElementArena<'a> for Arena<Element<'a>> {
    fn append_element<T: Into<Element<'a>>>(&mut self, element: T, parent: NodeId) -> NodeId {
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
            self.append_element(element, parent)
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
    fn append_element<T: Into<Element<'a>>>(&mut self, element: T, parent: NodeId) -> NodeId {
        let node = self.arena.new_node(element.into().into_owned());
        parent.append(node, self.arena);
        node
    }

    fn insert_before_last_child<T: Into<Element<'a>>>(
        &mut self,
        element: T,
        parent: NodeId,
    ) -> NodeId {
        self.arena
            .insert_before_last_child(element.into().into_owned(), parent)
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
    let node = arena.append_element(title, parent);
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
    for i in memchr_iter(b'\n', content.as_bytes()).chain(once(content.len())) {
        if let Some((mut tail, (headline_content, level))) = parse_headline(&content[last_end..]) {
            if last_end != 0 {
                let node = arena.append_element(Element::Section, parent);
                let content = &content[0..last_end];
                containers.push(Container::Block { content, node });
            }

            let node = arena.append_element(Element::Headline { level }, parent);
            containers.push(Container::Headline {
                content: headline_content,
                node,
            });

            while let Some((new_tail, (content, level))) = parse_headline(tail) {
                debug_assert_ne!(tail, new_tail);
                let node = arena.append_element(Element::Headline { level }, parent);
                containers.push(Container::Headline { content, node });
                tail = new_tail;
            }
            return;
        }
        last_end = i + 1;
    }

    let node = arena.append_element(Element::Section, parent);
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

    while !tail.is_empty() {
        let i = memchr(b'\n', tail.as_bytes())
            .map(|i| i + 1)
            .unwrap_or_else(|| tail.len());
        if tail.as_bytes()[0..i].iter().all(u8::is_ascii_whitespace) {
            let node = arena.append_element(Element::Paragraph, parent);

            containers.push(Container::Inline {
                content: &text[0..pos].trim_end_matches('\n'),
                node,
            });

            pos = 0;
            debug_assert_ne!(tail, skip_empty_lines(&tail[i..]));
            tail = skip_empty_lines(&tail[i..]);
            text = tail;
        } else if let Some(new_tail) = parse_block(tail, arena, parent, containers) {
            if pos != 0 {
                let node = arena.insert_before_last_child(Element::Paragraph, parent);

                containers.push(Container::Inline {
                    content: &text[0..pos].trim_end_matches('\n'),
                    node,
                });

                pos = 0;
            }
            debug_assert_ne!(tail, skip_empty_lines(new_tail));
            tail = skip_empty_lines(new_tail);
            text = tail;
        } else {
            debug_assert_ne!(tail, &tail[i..]);
            tail = &tail[i..];
            pos += i;
        }
    }

    if !text.is_empty() {
        let node = arena.append_element(Element::Paragraph, parent);

        containers.push(Container::Inline {
            content: &text[0..pos].trim_end_matches('\n'),
            node,
        });
    }
}

pub fn parse_block<'a, T: ElementArena<'a>>(
    contents: &'a str,
    arena: &mut T,
    parent: NodeId,
    containers: &mut Vec<Container<'a>>,
) -> Option<&'a str> {
    if let Some((tail, (fn_def, content))) = FnDef::parse(contents) {
        let node = arena.append_element(fn_def, parent);
        containers.push(Container::Block { content, node });
        return Some(tail);
    } else if let Some((tail, list, content)) = List::parse(contents) {
        let indent = list.indent;
        let node = arena.append_element(list, parent);
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
            let (tail, clock) = Clock::parse(contents)?;
            arena.append_element(clock, parent);
            Some(tail)
        }
        b'\'' => {
            // TODO: LaTeX environment
            None
        }
        b'-' => {
            let tail = parse_rule(contents)?;
            arena.append_element(Element::Rule, parent);
            Some(tail)
        }
        b':' => {
            if let Some((tail, (drawer, content))) = Drawer::parse(contents) {
                let node = arena.append_element(drawer, parent);
                containers.push(Container::Block { content, node });
                Some(tail)
            } else {
                let (tail, value) = parse_fixed_width(contents)?;
                let value = value.into();
                arena.append_element(Element::FixedWidth { value }, parent);
                Some(tail)
            }
        }
        b'|' => {
            let tail = parse_table(arena, contents, containers, parent)?;
            Some(tail)
        }
        b'#' => {
            if let Some((tail, (name, args, content))) = parse_block_element(contents) {
                match_block(
                    arena,
                    parent,
                    containers,
                    name.into(),
                    args.map(Into::into),
                    content,
                );
                Some(tail)
            } else if let Some((tail, (dyn_block, content))) = DynBlock::parse(contents) {
                let node = arena.append_element(dyn_block, parent);
                containers.push(Container::Block { content, node });
                Some(tail)
            } else if let Some((tail, (key, optional, value))) = parse_keyword(contents) {
                if (&*key).eq_ignore_ascii_case("CALL") {
                    arena.append_element(
                        BabelCall {
                            value: value.into(),
                        },
                        parent,
                    );
                } else {
                    arena.append_element(
                        Keyword {
                            key: key.into(),
                            optional: optional.map(Into::into),
                            value: value.into(),
                        },
                        parent,
                    );
                }
                Some(tail)
            } else {
                let (tail, value) = parse_comment(contents)?;
                let value = value.into();
                arena.append_element(Element::Comment { value }, parent);
                Some(tail)
            }
        }
        _ => None,
    }
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
            let node = arena.append_element(CenterBlock { parameters: args }, parent);
            containers.push(Container::Block { content, node });
        }
        "QUOTE" => {
            let node = arena.append_element(QuoteBlock { parameters: args }, parent);
            containers.push(Container::Block { content, node });
        }
        "COMMENT" => {
            arena.append_element(
                CommentBlock {
                    data: args,
                    contents: content.into(),
                },
                parent,
            );
        }
        "EXAMPLE" => {
            arena.append_element(
                ExampleBlock {
                    data: args,
                    contents: content.into(),
                },
                parent,
            );
        }
        "EXPORT" => {
            arena.append_element(
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
            arena.append_element(
                SourceBlock {
                    arguments,
                    language,
                    contents: content.into(),
                },
                parent,
            );
        }
        "VERSE" => {
            let node = arena.append_element(VerseBlock { parameters: args }, parent);
            containers.push(Container::Block { content, node });
        }
        _ => {
            let node = arena.append_element(
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

struct InlinePositions<'a> {
    bytes: &'a [u8],
    position: usize,
    next: Option<usize>,
}

impl InlinePositions<'_> {
    fn new(bytes: &[u8]) -> InlinePositions {
        InlinePositions {
            bytes,
            position: 0,
            next: Some(0),
        }
    }
}

impl Iterator for InlinePositions<'_> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        lazy_static::lazy_static! {
            static ref PRE_BYTES: BytesConst =
                bytes!(b'@', b'<', b'[', b' ', b'(', b'{', b'\'', b'"', b'\n');
        }

        self.next.take().or_else(|| {
            PRE_BYTES.find(&self.bytes[self.position..]).map(|i| {
                self.position += i + 1;

                match self.bytes[self.position - 1] {
                    b'{' => {
                        self.next = Some(self.position);
                        self.position - 1
                    }
                    b' ' | b'(' | b'\'' | b'"' | b'\n' => self.position,
                    _ => self.position - 1,
                }
            })
        })
    }
}

pub fn parse_inlines<'a, T: ElementArena<'a>>(
    arena: &mut T,
    content: &'a str,
    parent: NodeId,
    containers: &mut Vec<Container<'a>>,
) {
    let mut tail = content;

    if let Some(tail_) = parse_inline(tail, arena, containers, parent) {
        tail = tail_;
    }

    while let Some((tail_, i)) = InlinePositions::new(tail.as_bytes())
        .filter_map(|i| parse_inline(&tail[i..], arena, containers, parent).map(|tail| (tail, i)))
        .next()
    {
        if i != 0 {
            arena.insert_before_last_child(
                Element::Text {
                    value: tail[0..i].into(),
                },
                parent,
            );
        }
        tail = tail_;
    }

    if !tail.is_empty() {
        arena.append_element(Element::Text { value: tail.into() }, parent);
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

    match contents.as_bytes()[0] {
        b'@' => {
            let (tail, snippet) = Snippet::parse(contents)?;
            arena.append_element(snippet, parent);
            Some(tail)
        }
        b'{' => {
            let (tail, macros) = Macros::parse(contents)?;
            arena.append_element(macros, parent);
            Some(tail)
        }
        b'<' => {
            if let Some((tail, _content)) = parse_radio_target(contents) {
                arena.append_element(Element::RadioTarget, parent);
                Some(tail)
            } else if let Some((tail, target)) = Target::parse(contents) {
                arena.append_element(target, parent);
                Some(tail)
            } else if let Some((tail, timestamp)) = Timestamp::parse_active(contents) {
                arena.append_element(timestamp, parent);
                Some(tail)
            } else {
                let (tail, timestamp) = Timestamp::parse_diary(contents)?;
                arena.append_element(timestamp, parent);
                Some(tail)
            }
        }
        b'[' => {
            if let Some((tail, fn_ref)) = FnRef::parse(contents) {
                arena.append_element(fn_ref, parent);
                Some(tail)
            } else if let Some((tail, link)) = Link::parse(contents) {
                arena.append_element(link, parent);
                Some(tail)
            } else if let Some((tail, cookie)) = Cookie::parse(contents) {
                arena.append_element(cookie, parent);
                Some(tail)
            } else {
                let (tail, timestamp) = Timestamp::parse_inactive(contents)?;
                arena.append_element(timestamp, parent);
                Some(tail)
            }
        }
        b'*' => {
            let (tail, content) = parse_emphasis(contents, b'*')?;
            let node = arena.append_element(Element::Bold, parent);
            containers.push(Container::Inline { content, node });
            Some(tail)
        }
        b'+' => {
            let (tail, content) = parse_emphasis(contents, b'+')?;
            let node = arena.append_element(Element::Strike, parent);
            containers.push(Container::Inline { content, node });
            Some(tail)
        }
        b'/' => {
            let (tail, content) = parse_emphasis(contents, b'/')?;
            let node = arena.append_element(Element::Italic, parent);
            containers.push(Container::Inline { content, node });
            Some(tail)
        }
        b'_' => {
            let (tail, content) = parse_emphasis(contents, b'_')?;
            let node = arena.append_element(Element::Underline, parent);
            containers.push(Container::Inline { content, node });
            Some(tail)
        }
        b'=' => {
            let (tail, value) = parse_emphasis(contents, b'=')?;
            let value = value.into();
            arena.append_element(Element::Verbatim { value }, parent);
            Some(tail)
        }
        b'~' => {
            let (tail, value) = parse_emphasis(contents, b'~')?;
            let value = value.into();
            arena.append_element(Element::Code { value }, parent);
            Some(tail)
        }
        b's' => {
            let (tail, inline_src) = InlineSrc::parse(contents)?;
            arena.append_element(inline_src, parent);
            Some(tail)
        }
        b'c' => {
            let (tail, inline_call) = InlineCall::parse(contents)?;
            arena.append_element(inline_call, parent);
            Some(tail)
        }
        _ => None,
    }
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
        let node = arena.append_element(list_item, parent);
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
        let table_node = arena.append_element(Table::Org { tblfm: None }, parent);

        let mut last_end = 0;
        for start in memchr_iter(b'\n', contents.as_bytes()).chain(once(contents.len())) {
            let line = contents[last_end..start].trim();
            match TableRow::parse(line) {
                Some(TableRow::Standard) => {
                    let row_node = arena.append_element(TableRow::Standard, table_node);
                    for cell in line[1..].split_terminator('|') {
                        let cell_node = arena.append_element(Element::TableCell, row_node);
                        containers.push(Container::Inline {
                            content: cell.trim(),
                            node: cell_node,
                        });
                    }
                }
                Some(TableRow::Rule) => {
                    arena.append_element(TableRow::Rule, table_node);
                }
                None => return Some(&contents[last_end..]),
            }
            last_end = start + 1;
        }

        Some("")
    } else {
        let (tail, value) = parse_table_el(contents)?;
        let value = value.into();
        arena.append_element(Table::TableEl { value }, parent);

        Some(tail)
    }
}

pub fn line<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&str, &str, E> {
    if let Some(i) = memchr(b'\n', input.as_bytes()) {
        if i > 0 && input.as_bytes()[i - 1] == b'\r' {
            Ok((&input[i + 1..], &input[0..i - 1]))
        } else {
            Ok((&input[i + 1..], &input[0..i]))
        }
    } else {
        Ok(("", input))
    }
}

pub fn eol<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&str, &str, E> {
    verify(line, |s: &str| s.trim().is_empty())(input)
}

pub fn take_lines_while(predicate: impl Fn(&str) -> bool) -> impl Fn(&str) -> (&str, &str) {
    move |input| {
        let mut last_end = 0;
        for i in memchr_iter(b'\n', input.as_bytes()) {
            if i > 0 && input.as_bytes()[i - 1] == b'\r' {
                if !predicate(&input[last_end..i - 1]) {
                    return (&input[last_end..], &input[0..last_end]);
                }
            } else if !predicate(&input[last_end..i]) {
                return (&input[last_end..], &input[0..last_end]);
            }
            last_end = i + 1;
        }
        if !predicate(&input[last_end..]) {
            (&input[last_end..], &input[0..last_end])
        } else {
            ("", input)
        }
    }
}

pub fn skip_empty_lines(input: &str) -> &str {
    take_lines_while(|line| line.trim().is_empty())(input).0
}

pub fn parse_headline(input: &str) -> Option<(&str, (&str, usize))> {
    let (input_, level) = parse_headline_level(input)?;
    let (input_, content) = take_lines_while(move |line| {
        parse_headline_level(line)
            .map(|(_, l)| l > level)
            .unwrap_or(true)
    })(input_);
    Some((input_, (&input[0..level + content.len()], level)))
}

pub fn parse_headline_level(input: &str) -> Option<(&str, usize)> {
    let (input, stars) = take_while1::<_, _, ()>(|c: char| c == '*')(input).ok()?;

    if input.starts_with(' ') || input.starts_with('\n') || input.is_empty() {
        Some((input, stars.len()))
    } else {
        None
    }
}

pub fn parse_fixed_width(input: &str) -> Option<(&str, &str)> {
    let (input, content) = take_lines_while(|line| line == ":" || line.starts_with(": "))(input);

    if !content.is_empty() {
        Some((input, content))
    } else {
        None
    }
}

pub fn parse_comment(input: &str) -> Option<(&str, &str)> {
    let (input, content) = take_lines_while(|line| line == "#" || line.starts_with("# "))(input);

    if !content.is_empty() {
        Some((input, content))
    } else {
        None
    }
}

pub fn take_one_word<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&str, &str, E> {
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
