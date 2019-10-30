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
    radio_target::parse_radio_target, BabelCall, CenterBlock, Clock, Comment, CommentBlock, Cookie,
    Drawer, DynBlock, Element, ExampleBlock, ExportBlock, FixedWidth, FnDef, FnRef, InlineCall,
    InlineSrc, Keyword, Link, List, ListItem, Macros, QuoteBlock, Rule, Snippet, SourceBlock,
    SpecialBlock, Table, TableRow, Target, Timestamp, Title, VerseBlock,
};

pub trait ElementArena<'a> {
    fn append<T>(&mut self, element: T, parent: NodeId) -> NodeId
    where
        T: Into<Element<'a>>;
    fn insert_before_last_child<T>(&mut self, element: T, parent: NodeId) -> NodeId
    where
        T: Into<Element<'a>>;
    fn set<T>(&mut self, node: NodeId, element: T)
    where
        T: Into<Element<'a>>;
}

impl<'a> ElementArena<'a> for Arena<Element<'a>> {
    fn append<T>(&mut self, element: T, parent: NodeId) -> NodeId
    where
        T: Into<Element<'a>>,
    {
        let node = self.new_node(element.into());
        parent.append(node, self);
        node
    }

    fn insert_before_last_child<T>(&mut self, element: T, parent: NodeId) -> NodeId
    where
        T: Into<Element<'a>>,
    {
        if let Some(child) = self[parent].last_child() {
            let node = self.new_node(element.into());
            child.insert_before(node, self);
            node
        } else {
            self.append(element, parent)
        }
    }

    fn set<T>(&mut self, node: NodeId, element: T)
    where
        T: Into<Element<'a>>,
    {
        *self[node].get_mut() = element.into();
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
    fn append<T>(&mut self, element: T, parent: NodeId) -> NodeId
    where
        T: Into<Element<'a>>,
    {
        self.arena.append(element.into().into_owned(), parent)
    }

    fn insert_before_last_child<T>(&mut self, element: T, parent: NodeId) -> NodeId
    where
        T: Into<Element<'a>>,
    {
        self.arena
            .insert_before_last_child(element.into().into_owned(), parent)
    }

    fn set<T>(&mut self, node: NodeId, element: T)
    where
        T: Into<Element<'a>>,
    {
        self.arena.set(node, element.into().into_owned());
    }
}

#[derive(Debug)]
pub enum Container<'a> {
    // Block, List Item
    Block { content: &'a str, node: NodeId },
    // Pargraph, Inline Markup
    Inline { content: &'a str, node: NodeId },
    // Headline
    Headline { content: &'a str, node: NodeId },
    // Document
    Document { content: &'a str, node: NodeId },
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
    let node = arena.append(title, parent);
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
                let node = arena.append(Element::Section, parent);
                let content = &content[0..last_end];
                containers.push(Container::Block { content, node });
            }

            let node = arena.append(Element::Headline { level }, parent);
            containers.push(Container::Headline {
                content: headline_content,
                node,
            });

            while let Some((new_tail, (content, level))) = parse_headline(tail) {
                debug_assert_ne!(tail, new_tail);
                let node = arena.append(Element::Headline { level }, parent);
                containers.push(Container::Headline { content, node });
                tail = new_tail;
            }
            return;
        }
        last_end = i + 1;
    }

    let node = arena.append(Element::Section, parent);
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
            let (tail_, blank) = blank_lines(&tail[i..]);
            debug_assert_ne!(tail, tail_);
            tail = tail_;

            let node = arena.append(
                Element::Paragraph {
                    // including current line (&tail[0..i])
                    post_blank: blank + 1,
                },
                parent,
            );

            containers.push(Container::Inline {
                content: &text[0..pos].trim_end(),
                node,
            });

            pos = 0;
            text = tail;
        } else if let Some(new_tail) = parse_block(tail, arena, parent, containers) {
            if pos != 0 {
                let node =
                    arena.insert_before_last_child(Element::Paragraph { post_blank: 0 }, parent);

                containers.push(Container::Inline {
                    content: &text[0..pos].trim_end(),
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
        let node = arena.append(Element::Paragraph { post_blank: 0 }, parent);

        containers.push(Container::Inline {
            content: &text[0..pos].trim_end(),
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
    match contents
        .as_bytes()
        .iter()
        .find(|c| !c.is_ascii_whitespace())?
    {
        b'[' => {
            let (tail, (fn_def, content)) = FnDef::parse(contents)?;
            let node = arena.append(fn_def, parent);
            containers.push(Container::Block { content, node });
            Some(tail)
        }
        b'0'..=b'9' | b'*' => {
            let tail = parse_list(arena, contents, parent, containers)?;
            Some(tail)
        }
        b'C' => {
            let (tail, clock) = Clock::parse(contents)?;
            arena.append(clock, parent);
            Some(tail)
        }
        b'\'' => {
            // TODO: LaTeX environment
            None
        }
        b'-' => {
            if let Some((tail, rule)) = Rule::parse(contents) {
                arena.append(rule, parent);
                Some(tail)
            } else {
                let tail = parse_list(arena, contents, parent, containers)?;
                Some(tail)
            }
        }
        b':' => {
            if let Some((tail, (drawer, content))) = Drawer::parse(contents) {
                let node = arena.append(drawer, parent);
                containers.push(Container::Block { content, node });
                Some(tail)
            } else {
                let (tail, fixed_width) = FixedWidth::parse(contents)?;
                arena.append(fixed_width, parent);
                Some(tail)
            }
        }
        b'|' => {
            let tail = parse_org_table(arena, contents, containers, parent);
            Some(tail)
        }
        b'+' => {
            if let Some((tail, table)) = Table::parse_table_el(contents) {
                arena.append(table, parent);
                Some(tail)
            } else {
                let tail = parse_list(arena, contents, parent, containers)?;
                Some(tail)
            }
        }
        b'#' => {
            if let Some((tail, (name, args, content, blank))) = parse_block_element(contents) {
                match_block(
                    arena,
                    parent,
                    containers,
                    name.into(),
                    args.map(Into::into),
                    content,
                    blank,
                );
                Some(tail)
            } else if let Some((tail, (dyn_block, content))) = DynBlock::parse(contents) {
                let node = arena.append(dyn_block, parent);
                containers.push(Container::Block { content, node });
                Some(tail)
            } else if let Some((tail, (key, optional, value, blank))) = parse_keyword(contents) {
                if (&*key).eq_ignore_ascii_case("CALL") {
                    arena.append(
                        BabelCall {
                            value: value.into(),
                            post_blank: blank,
                        },
                        parent,
                    );
                } else {
                    arena.append(
                        Keyword {
                            key: key.into(),
                            optional: optional.map(Into::into),
                            value: value.into(),
                            post_blank: blank,
                        },
                        parent,
                    );
                }
                Some(tail)
            } else {
                let (tail, comment) = Comment::parse(contents)?;
                arena.append(comment, parent);
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
    parameters: Option<Cow<'a, str>>,
    content: &'a str,
    post_blank: usize,
) {
    match &*name.to_uppercase() {
        "CENTER" => {
            let (content, pre_blank) = blank_lines(content);
            let node = arena.append(
                CenterBlock {
                    parameters,
                    pre_blank,
                    post_blank,
                },
                parent,
            );
            containers.push(Container::Block { content, node });
        }
        "QUOTE" => {
            let (content, pre_blank) = blank_lines(content);
            let node = arena.append(
                QuoteBlock {
                    parameters,
                    pre_blank,
                    post_blank,
                },
                parent,
            );
            containers.push(Container::Block { content, node });
        }
        "VERSE" => {
            let (content, pre_blank) = blank_lines(content);
            let node = arena.append(
                VerseBlock {
                    parameters,
                    pre_blank,
                    post_blank,
                },
                parent,
            );
            containers.push(Container::Block { content, node });
        }
        "COMMENT" => {
            arena.append(
                CommentBlock {
                    data: parameters,
                    contents: content.into(),
                    post_blank,
                },
                parent,
            );
        }
        "EXAMPLE" => {
            arena.append(
                ExampleBlock {
                    data: parameters,
                    contents: content.into(),
                    post_blank,
                },
                parent,
            );
        }
        "EXPORT" => {
            arena.append(
                ExportBlock {
                    data: parameters.unwrap_or_default(),
                    contents: content.into(),
                    post_blank,
                },
                parent,
            );
        }
        "SRC" => {
            let (language, arguments) = match &parameters {
                Some(Cow::Borrowed(args)) => {
                    let (language, arguments) =
                        args.split_at(args.find(' ').unwrap_or_else(|| args.len()));
                    (language.into(), arguments.into())
                }
                None => (Cow::Borrowed(""), Cow::Borrowed("")),
                _ => unreachable!("`parse_block_element` returns `Some(Cow::Borrowed)` or `None`"),
            };
            arena.append(
                SourceBlock {
                    arguments,
                    language,
                    contents: content.into(),
                    post_blank,
                },
                parent,
            );
        }
        _ => {
            let (content, pre_blank) = blank_lines(content);
            let node = arena.append(
                SpecialBlock {
                    parameters,
                    name,
                    pre_blank,
                    post_blank,
                },
                parent,
            );
            containers.push(Container::Block { content, node });
        }
    }
}

struct InlinePositions<'a> {
    bytes: &'a [u8],
    pos: usize,
    next: Option<usize>,
}

impl InlinePositions<'_> {
    fn new(bytes: &[u8]) -> InlinePositions {
        InlinePositions {
            bytes,
            pos: 0,
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
            PRE_BYTES.find(&self.bytes[self.pos..]).map(|i| {
                self.pos += i + 1;

                match self.bytes[self.pos - 1] {
                    b'{' => {
                        self.next = Some(self.pos);
                        self.pos - 1
                    }
                    b' ' | b'(' | b'\'' | b'"' | b'\n' => self.pos,
                    _ => self.pos - 1,
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
        arena.append(Element::Text { value: tail.into() }, parent);
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
            arena.append(snippet, parent);
            Some(tail)
        }
        b'{' => {
            let (tail, macros) = Macros::parse(contents)?;
            arena.append(macros, parent);
            Some(tail)
        }
        b'<' => {
            if let Some((tail, _content)) = parse_radio_target(contents) {
                arena.append(Element::RadioTarget, parent);
                Some(tail)
            } else if let Some((tail, target)) = Target::parse(contents) {
                arena.append(target, parent);
                Some(tail)
            } else if let Some((tail, timestamp)) = Timestamp::parse_active(contents) {
                arena.append(timestamp, parent);
                Some(tail)
            } else {
                let (tail, timestamp) = Timestamp::parse_diary(contents)?;
                arena.append(timestamp, parent);
                Some(tail)
            }
        }
        b'[' => {
            if let Some((tail, fn_ref)) = FnRef::parse(contents) {
                arena.append(fn_ref, parent);
                Some(tail)
            } else if let Some((tail, link)) = Link::parse(contents) {
                arena.append(link, parent);
                Some(tail)
            } else if let Some((tail, cookie)) = Cookie::parse(contents) {
                arena.append(cookie, parent);
                Some(tail)
            } else {
                let (tail, timestamp) = Timestamp::parse_inactive(contents)?;
                arena.append(timestamp, parent);
                Some(tail)
            }
        }
        b'*' => {
            let (tail, content) = parse_emphasis(contents, b'*')?;
            let node = arena.append(Element::Bold, parent);
            containers.push(Container::Inline { content, node });
            Some(tail)
        }
        b'+' => {
            let (tail, content) = parse_emphasis(contents, b'+')?;
            let node = arena.append(Element::Strike, parent);
            containers.push(Container::Inline { content, node });
            Some(tail)
        }
        b'/' => {
            let (tail, content) = parse_emphasis(contents, b'/')?;
            let node = arena.append(Element::Italic, parent);
            containers.push(Container::Inline { content, node });
            Some(tail)
        }
        b'_' => {
            let (tail, content) = parse_emphasis(contents, b'_')?;
            let node = arena.append(Element::Underline, parent);
            containers.push(Container::Inline { content, node });
            Some(tail)
        }
        b'=' => {
            let (tail, value) = parse_emphasis(contents, b'=')?;
            let value = value.into();
            arena.append(Element::Verbatim { value }, parent);
            Some(tail)
        }
        b'~' => {
            let (tail, value) = parse_emphasis(contents, b'~')?;
            let value = value.into();
            arena.append(Element::Code { value }, parent);
            Some(tail)
        }
        b's' => {
            let (tail, inline_src) = InlineSrc::parse(contents)?;
            arena.append(inline_src, parent);
            Some(tail)
        }
        b'c' => {
            let (tail, inline_call) = InlineCall::parse(contents)?;
            arena.append(inline_call, parent);
            Some(tail)
        }
        _ => None,
    }
}

pub fn parse_list<'a, T: ElementArena<'a>>(
    arena: &mut T,
    contents: &'a str,
    parent: NodeId,
    containers: &mut Vec<Container<'a>>,
) -> Option<&'a str> {
    let (mut tail, (first_item, content)) = ListItem::parse(contents)?;
    let first_item_indent = first_item.indent;
    let first_item_ordered = first_item.ordered;

    let parent = arena.append(Element::Document { pre_blank: 0 }, parent); // placeholder

    let node = arena.append(first_item, parent);
    containers.push(Container::Block { content, node });

    while let Some((tail_, (item, content))) = ListItem::parse(tail) {
        if item.indent == first_item_indent {
            let node = arena.append(item, parent);
            containers.push(Container::Block { content, node });
            debug_assert_ne!(tail, tail_);
            tail = tail_;
        } else {
            break;
        }
    }

    let (tail, blank) = blank_lines(tail);

    arena.set(
        parent,
        List {
            indent: first_item_indent,
            ordered: first_item_ordered,
            post_blank: blank,
        },
    );

    Some(tail)
}

pub fn parse_org_table<'a, T: ElementArena<'a>>(
    arena: &mut T,
    contents: &'a str,
    containers: &mut Vec<Container<'a>>,
    parent: NodeId,
) -> &'a str {
    let (tail, contents) = take_lines_while(|line| line.trim_start().starts_with('|'))(contents);
    let (tail, blank) = blank_lines(tail);

    let parent = arena.append(
        Table::Org {
            tblfm: None,
            post_blank: blank,
        },
        parent,
    );

    let mut last_end = 0;
    for start in memchr_iter(b'\n', contents.as_bytes()).chain(once(contents.len())) {
        let line = contents[last_end..start].trim_start();
        if line.starts_with("|-") {
            arena.append(TableRow::Rule, parent);
        } else {
            let parent = arena.append(TableRow::Standard, parent);
            for content in line.split_terminator('|').skip(1) {
                let node = arena.append(Element::TableCell, parent);
                containers.push(Container::Inline { content, node });
            }
        }
        last_end = start + 1;
    }

    tail
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
    verify(line, |s: &str| {
        s.as_bytes().iter().all(|c| c.is_ascii_whitespace())
    })(input)
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
    take_lines_while(|line| line.as_bytes().iter().all(|c| c.is_ascii_whitespace()))(input).0
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

pub fn blank_lines(input: &str) -> (&str, usize) {
    let bytes = input.as_bytes();
    let mut blank = 0;
    let mut last_end = 0;
    for i in memchr_iter(b'\n', bytes) {
        if bytes[last_end..i].iter().all(u8::is_ascii_whitespace) {
            blank += 1;
        } else {
            break;
        }
        last_end = 1 + i;
    }
    (&input[last_end..], blank)
}

#[test]
pub fn test_blank_lines() {
    assert_eq!(blank_lines("foo"), ("foo", 0));
    assert_eq!(blank_lines(" foo"), (" foo", 0));
    assert_eq!(blank_lines("  \t\nfoo\n"), ("foo\n", 1));
    assert_eq!(blank_lines("\n    \r\n\nfoo\n"), ("foo\n", 3));
    assert_eq!(blank_lines("\r\n   \n  \r\n   foo\n"), ("   foo\n", 3));
}
