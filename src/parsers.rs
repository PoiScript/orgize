// parser related functions

use indextree::{Arena, NodeId};
use jetscii::bytes;
use memchr::{memchr, memchr2, memchr_iter};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_till},
    character::complete::space0,
    error::ErrorKind,
    error_position, Err, IResult,
};

use crate::config::ParseConfig;
use crate::elements::*;

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

pub fn parse_title<'a>(
    arena: &mut Arena<Element<'a>>,
    content: &'a str,
    parent: NodeId,
    containers: &mut Vec<Container<'a>>,
    config: &ParseConfig,
) -> &'a str {
    let (tail, title) = Title::parse(content, config).unwrap();
    let content = title.raw;
    let node = arena.new_node(Element::Title(title));
    parent.append(node, arena);
    containers.push(Container::Inline { content, node });
    tail
}

pub fn parse_section_and_headlines<'a>(
    arena: &mut Arena<Element<'a>>,
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
        if let Some((mut tail, headline_content)) = parse_headline(&content[last_end..]) {
            if last_end != 0 {
                let node = arena.new_node(Element::Section);
                parent.append(node, arena);
                containers.push(Container::Block {
                    content: &content[0..last_end],
                    node,
                });
            }
            let node = arena.new_node(Element::Headline);
            parent.append(node, arena);
            containers.push(Container::Headline {
                content: headline_content,
                node,
            });
            while let Some((new_tail, content)) = parse_headline(tail) {
                let node = arena.new_node(Element::Headline);
                parent.append(node, arena);
                containers.push(Container::Headline { content, node });
                tail = new_tail;
            }
            return;
        }
        last_end = i + 1;
    }

    let node = arena.new_node(Element::Section);
    parent.append(node, arena);
    containers.push(Container::Block { content, node });
}

pub fn parse_headline(text: &str) -> Option<(&str, &str)> {
    let level = get_headline_level(text)?;

    for i in memchr_iter(b'\n', text.as_bytes()) {
        if let Some(l) = get_headline_level(&text[i + 1..]) {
            if l <= level {
                return Some((&text[i + 1..], &text[0..i + 1]));
            }
        }
    }

    Some(("", text))
}

pub fn get_headline_level(text: &str) -> Option<usize> {
    if let Some(off) = memchr2(b'\n', b' ', text.as_bytes()) {
        if off > 0 && text[0..off].as_bytes().iter().all(|&c| c == b'*') {
            Some(off)
        } else {
            None
        }
    } else if !text.is_empty() && text.as_bytes().iter().all(|&c| c == b'*') {
        Some(text.len())
    } else {
        None
    }
}

pub fn parse_blocks<'a>(
    arena: &mut Arena<Element<'a>>,
    content: &'a str,
    parent: NodeId,
    containers: &mut Vec<Container<'a>>,
) {
    let mut tail = skip_empty_lines(content);

    if let Some((new_tail, element)) = parse_block(content, arena, containers) {
        parent.append(element, arena);
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
            parent.append(node, arena);
            containers.push(Container::Inline {
                content: &text[0..pos].trim_end_matches('\n'),
                node,
            });
            text = tail;
            pos = 0;
        } else if let Some((new_tail, element)) = parse_block(tail, arena, containers) {
            if pos != 0 {
                let node = arena.new_node(Element::Paragraph);
                parent.append(node, arena);
                containers.push(Container::Inline {
                    content: &text[0..pos].trim_end_matches('\n'),
                    node,
                });
                pos = 0;
            }
            parent.append(element, arena);
            tail = skip_empty_lines(new_tail);
            text = tail;
        } else {
            tail = &tail[i..];
            pos += i;
        }
    }

    if !text.is_empty() {
        let node = arena.new_node(Element::Paragraph);
        parent.append(node, arena);
        containers.push(Container::Inline {
            content: &text[0..pos].trim_end_matches('\n'),
            node,
        });
    }
}

pub fn parse_block<'a>(
    contents: &'a str,
    arena: &mut Arena<Element<'a>>,
    containers: &mut Vec<Container<'a>>,
) -> Option<(&'a str, NodeId)> {
    if let Some((tail, node)) = prase_table(arena, contents, containers) {
        return Some((tail, node));
    }

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

    if let Ok((tail, clock)) = Clock::parse(tail) {
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
        if let Ok((tail, (drawer, content))) = Drawer::parse(tail) {
            let node = arena.new_node(drawer.into());
            containers.push(Container::Block { content, node });
            return Some((tail, node));
        }
    }

    // FixedWidth
    if tail == ":" || tail.starts_with(": ") || tail.starts_with(":\n") {
        let mut last_end = 1; // ":"
        for i in memchr_iter(b'\n', contents.as_bytes()) {
            last_end = i + 1;
            let tail = contents[last_end..].trim_start();
            if !(tail == ":" || tail.starts_with(": ") || tail.starts_with(":\n")) {
                let fixed_width = arena.new_node(Element::FixedWidth {
                    value: &contents[0..last_end],
                });
                return Some((&contents[last_end..], fixed_width));
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
            let line = contents[last_end..].trim_start();
            if !(line == "#" || line.starts_with("# ") || line.starts_with("#\n")) {
                let comment = arena.new_node(Element::Comment {
                    value: &contents[0..last_end],
                });
                return Some((&contents[last_end..], comment));
            }
        }
        let comment = arena.new_node(Element::Comment {
            value: &contents[0..last_end],
        });
        return Some((&contents[last_end..], comment));
    }

    if tail.starts_with("#+") {
        if let Ok((tail, (block, content))) = Block::parse(tail) {
            match &*block.name.to_uppercase() {
                "CENTER" => {
                    let node = arena.new_node(Element::CenterBlock(CenterBlock {
                        parameters: block.args,
                    }));
                    containers.push(Container::Block { content, node });
                    Some((tail, node))
                }
                "QUOTE" => {
                    let node = arena.new_node(Element::QuoteBlock(QuoteBlock {
                        parameters: block.args,
                    }));
                    containers.push(Container::Block { content, node });
                    Some((tail, node))
                }
                "COMMENT" => {
                    let node = arena.new_node(Element::CommentBlock(CommentBlock {
                        data: block.args,
                        contents: content,
                    }));
                    Some((tail, node))
                }
                "EXAMPLE" => {
                    let node = arena.new_node(Element::ExampleBlock(ExampleBlock {
                        data: block.args,
                        contents: content,
                    }));
                    Some((tail, node))
                }
                "EXPORT" => {
                    let node = arena.new_node(Element::ExportBlock(ExportBlock {
                        data: block.args.unwrap_or(""),
                        contents: content,
                    }));
                    Some((tail, node))
                }
                "SRC" => {
                    let (language, arguments) = block
                        .args
                        .map(|args| args.split_at(args.find(' ').unwrap_or_else(|| args.len())))
                        .unwrap_or(("", ""));
                    let node = arena.new_node(Element::SourceBlock(SourceBlock {
                        arguments,
                        language,
                        contents: content,
                    }));
                    Some((tail, node))
                }
                "VERSE" => {
                    let node = arena.new_node(Element::VerseBlock(VerseBlock {
                        parameters: block.args,
                    }));
                    containers.push(Container::Block { content, node });
                    Some((tail, node))
                }
                _ => {
                    let node = arena.new_node(Element::SpecialBlock(SpecialBlock {
                        parameters: block.args,
                        name: block.name,
                    }));
                    containers.push(Container::Block { content, node });
                    Some((tail, node))
                }
            }
        } else if let Ok((tail, (dyn_block, content))) = DynBlock::parse(tail) {
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

pub fn parse_inlines<'a>(
    arena: &mut Arena<Element<'a>>,
    content: &'a str,
    parent: NodeId,
    containers: &mut Vec<Container<'a>>,
) {
    let mut tail = content;

    if let Some((new_tail, element)) = parse_inline(tail, arena, containers) {
        parent.append(element, arena);
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
                        parent.append(node, arena);
                        pos = 0;
                    }
                    parent.append(element, arena);
                    tail = new_tail;
                    text = new_tail;
                    continue;
                } else if let Some((new_tail, element)) =
                    parse_inline(&tail[off + 1..], arena, containers)
                {
                    let node = arena.new_node(Element::Text {
                        value: &text[0..pos + off + 1],
                    });
                    parent.append(node, arena);
                    pos = 0;
                    parent.append(element, arena);
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
                    parent.append(node, arena);
                    pos = 0;
                    parent.append(element, arena);
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
                        parent.append(node, arena);
                        pos = 0;
                    }
                    parent.append(element, arena);
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
        parent.append(node, arena);
    }
}

pub fn parse_inline<'a>(
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
                    .ok()
                    .map(|(tail, fn_ref)| (tail, arena.new_node(fn_ref.into())))
            } else if bytes[1] == b'[' {
                Link::parse(contents)
                    .ok()
                    .map(|(tail, element)| (tail, arena.new_node(element)))
            } else {
                Cookie::parse(contents)
                    .map(|(tail, cookie)| (tail, cookie.into()))
                    .or_else(|_| {
                        Timestamp::parse_inactive(contents)
                            .map(|(tail, timestamp)| (tail, timestamp.into()))
                    })
                    .ok()
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

pub fn parse_list_items<'a>(
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
        parent.append(node, arena);
        containers.push(Container::Block { content, node });
        contents = tail;
    }
}

pub fn prase_table<'a>(
    arena: &mut Arena<Element<'a>>,
    contents: &'a str,
    containers: &mut Vec<Container<'a>>,
) -> Option<(&'a str, NodeId)> {
    if contents.trim_start().starts_with('|') {
        let table_node = arena.new_node(Element::Table(Table::Org { tblfm: None }));

        let mut last_end = 0;
        for start in memchr_iter(b'\n', contents.as_bytes()) {
            let line = contents[last_end..start].trim();
            match TableRow::parse(line) {
                Some(TableRow::Standard) => {
                    let row_node = arena.new_node(Element::TableRow(TableRow::Standard));
                    table_node.append(row_node, arena);
                    for cell in line[1..].split_terminator('|') {
                        let cell_node = arena.new_node(Element::TableCell);
                        row_node.append(cell_node, arena);
                        containers.push(Container::Inline {
                            content: cell.trim(),
                            node: cell_node,
                        });
                    }
                }
                Some(TableRow::Rule) => {
                    let row_node = arena.new_node(Element::TableRow(TableRow::Rule));
                    table_node.append(row_node, arena);
                }
                None => return Some((&contents[last_end..], table_node)),
            }
            last_end = start + 1;
        }

        Some(("", table_node))
    } else if contents.trim_start().starts_with("+-")
        && contents[0..memchr(b'\n', contents.as_bytes()).unwrap_or_else(|| contents.len())]
            .trim()
            .as_bytes()
            .iter()
            .any(|&c| c != b'+' || c != b'-')
    {
        let mut last_end = 0;
        for start in memchr_iter(b'\n', contents.as_bytes()) {
            let line = contents[last_end..start].trim();
            if !line.starts_with('|') && !line.starts_with('+') {
                return {
                    Some((
                        &contents[last_end..],
                        arena.new_node(Element::Table(Table::TableEl {
                            value: &contents[0..last_end],
                        })),
                    ))
                };
            }
            last_end = start + 1;
        }

        Some((
            "",
            arena.new_node(Element::Table(Table::TableEl { value: contents })),
        ))
    } else {
        None
    }
}

pub fn eol(input: &str) -> IResult<&str, ()> {
    let (input, _) = space0(input)?;
    if input.is_empty() {
        Ok(("", ()))
    } else {
        let (input, _) = tag("\n")(input)?;
        Ok((input, ()))
    }
}

pub fn take_until_eol(input: &str) -> IResult<&str, &str> {
    if let Some(i) = memchr(b'\n', input.as_bytes()) {
        Ok((&input[i + 1..], input[0..i].trim()))
    } else {
        Ok(("", input.trim()))
    }
}

pub fn take_lines_till(predicate: impl Fn(&str) -> bool) -> impl Fn(&str) -> IResult<&str, &str> {
    move |input| {
        let mut start = 0;
        for i in memchr_iter(b'\n', input.as_bytes()) {
            if predicate(input[start..i].trim()) {
                return Ok((&input[i + 1..], &input[0..start]));
            }
            start = i + 1;
        }

        if predicate(input[start..].trim()) {
            Ok(("", &input[0..start]))
        } else {
            Err(Err::Error(error_position!(input, ErrorKind::TakeTill1)))
        }
    }
}

pub fn take_one_word(input: &str) -> IResult<&str, &str> {
    alt((take_till(|c: char| c == ' ' || c == '\t'), |input| {
        Ok(("", input))
    }))(input)
}

pub fn skip_empty_lines(contents: &str) -> &str {
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
