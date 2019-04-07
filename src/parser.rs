//! Parser

use crate::{elements::*, headline::*, objects::*};
use jetscii::bytes;
use memchr::memchr_iter;

#[cfg_attr(test, derive(PartialEq))]
#[derive(Copy, Clone, Debug)]
enum Container {
    Headline(usize),
    Section(usize),
    Drawer,
    Paragraph,
    CtrBlock,
    QteBlock,
    SplBlock,
    DynBlock,
    List(usize, bool),
    ListItem,
    Italic,
    Strike,
    Bold,
    Underline,
}

#[cfg_attr(test, derive(PartialEq))]
#[derive(Debug)]
pub enum Event<'a> {
    HeadlineBeg(Headline<'a>),
    HeadlineEnd,

    SectionBeg,
    SectionEnd,

    ParagraphBeg,
    ParagraphEnd,

    CtrBlockBeg,
    CtrBlockEnd,
    QteBlockBeg,
    QteBlockEnd,
    SplBlockBeg {
        name: &'a str,
        args: Option<&'a str>,
    },
    SplBlockEnd,
    DynBlockBeg {
        name: &'a str,
        args: Option<&'a str>,
    },
    DynBlockEnd,

    CommentBlock {
        args: Option<&'a str>,
        cont: &'a str,
    },
    ExampleBlock {
        args: Option<&'a str>,
        cont: &'a str,
    },
    ExportBlock {
        args: Option<&'a str>,
        cont: &'a str,
    },
    SrcBlock {
        args: Option<&'a str>,
        cont: &'a str,
    },
    VerseBlock {
        args: Option<&'a str>,
        cont: &'a str,
    },

    ListBeg {
        ordered: bool,
    },
    ListEnd {
        ordered: bool,
    },
    ListItemBeg {
        bullet: &'a str,
    },
    ListItemEnd,

    Call {
        value: &'a str,
    },

    Clock,

    Comment(&'a str),
    FixedWidth(&'a str),

    Planning(Planning<'a>),

    DrawerBeg(&'a str),
    DrawerEnd,

    TableStart,
    TableEnd,
    TableCell,

    LatexEnv,
    FnDef {
        label: &'a str,
        cont: &'a str,
    },
    Keyword {
        key: Key<'a>,
        value: &'a str,
    },
    Rule,

    Timestamp(Timestamp<'a>),
    Cookie(Cookie<'a>),
    FnRef {
        label: Option<&'a str>,
        def: Option<&'a str>,
    },
    InlineCall {
        name: &'a str,
        args: &'a str,
        inside_header: Option<&'a str>,
        end_header: Option<&'a str>,
    },
    InlineSrc {
        lang: &'a str,
        option: Option<&'a str>,
        body: &'a str,
    },
    Link {
        path: &'a str,
        desc: Option<&'a str>,
    },
    Macros {
        name: &'a str,
        args: Option<&'a str>,
    },
    RadioTarget {
        target: &'a str,
    },
    Snippet {
        name: &'a str,
        value: &'a str,
    },
    Target {
        target: &'a str,
    },

    BoldBeg,
    BoldEnd,
    ItalicBeg,
    ItalicEnd,
    StrikeBeg,
    StrikeEnd,
    UnderlineBeg,
    UnderlineEnd,

    Verbatim(&'a str),
    Code(&'a str),
    Text(&'a str),
}

pub struct Parser<'a> {
    text: &'a str,
    stack: Vec<(Container, usize, usize)>,
    off: usize,
    ele_buf: Option<(Event<'a>, usize, usize, usize)>,
    obj_buf: Option<(Event<'a>, usize, usize, usize)>,
    keywords: &'a [&'a str],
    list_more_item: bool,
}

impl<'a> Parser<'a> {
    /// creates a new parser from string
    pub fn new(text: &'a str) -> Parser<'a> {
        Parser {
            text,
            stack: Vec::new(),
            off: 0,
            ele_buf: None,
            obj_buf: None,
            list_more_item: false,
            keywords: DEFAULT_KEYWORDS,
        }
    }

    /// returns current offset
    pub fn offset(&self) -> usize {
        self.off
    }

    /// returns current stack depth
    pub fn stack_depth(&self) -> usize {
        self.stack.len()
    }

    pub fn set_keywords(&mut self, keywords: &'a [&'a str]) {
        self.keywords = keywords;
    }

    fn next_section_or_headline(&mut self, text: &'a str) -> Event<'a> {
        let end = Headline::find_level(text, std::usize::MAX);
        if end != 0 {
            self.push_stack(Container::Section(self.off), end, end);
            Event::SectionBeg
        } else {
            self.next_headline(text)
        }
    }

    fn next_headline(&mut self, text: &'a str) -> Event<'a> {
        let (hdl, off, end) = Headline::parse(text, self.keywords);
        self.push_stack(Container::Headline(self.off + off), end, end);
        self.off += off;
        Event::HeadlineBeg(hdl)
    }

    fn next_ele(&mut self, text: &'a str) -> Event<'a> {
        fn skip_empty_lines(text: &str) -> usize {
            let mut i = 0;
            for pos in memchr_iter(b'\n', text.as_bytes()) {
                if text.as_bytes()[i..pos].iter().all(u8::is_ascii_whitespace) {
                    i = pos + 1;
                } else {
                    return i;
                }
            }
            if text.as_bytes()[i..].iter().all(u8::is_ascii_whitespace) {
                text.len()
            } else {
                i
            }
        }

        let start = skip_empty_lines(text);
        if start == text.len() {
            self.off += text.len();
            return self.end();
        };
        let tail = &text[start..];

        let (ele, off, limit, end) = self
            .ele_buf
            .take()
            .or_else(|| self.real_next_ele(tail))
            .unwrap_or_else(|| {
                let mut pos = 0;
                for off in memchr_iter(b'\n', tail.as_bytes()) {
                    if tail.as_bytes()[pos..off]
                        .iter()
                        .all(u8::is_ascii_whitespace)
                    {
                        return (Event::ParagraphBeg, 0, pos + start, off + start);
                    } else if let Some(buf) = self.real_next_ele(&tail[pos..]) {
                        self.ele_buf = Some(buf);
                        return (Event::ParagraphBeg, 0, pos + start, pos + start);
                    }
                    pos = off + 1;
                }
                let len = text.len();
                (
                    Event::ParagraphBeg,
                    0,
                    if text.ends_with('\n') { len - 1 } else { len },
                    len,
                )
            });

        debug_assert!(
            (limit == 0 && end == 0) || (off <= limit && limit <= end && end <= text.len()),
            "{} <= {} <= {} <= {}",
            off,
            limit,
            end,
            text.len()
        );

        match ele {
            Event::DrawerBeg(_) => self.push_stack(Container::Drawer, limit, end),
            Event::ParagraphBeg => self.push_stack(Container::Paragraph, limit, end),
            Event::QteBlockBeg => self.push_stack(Container::QteBlock, limit, end),
            Event::CtrBlockBeg => self.push_stack(Container::CtrBlock, limit, end),
            Event::SplBlockBeg { .. } => self.push_stack(Container::SplBlock, limit, end),
            Event::DynBlockBeg { .. } => self.push_stack(Container::DynBlock, limit, end),
            Event::ListBeg { ordered, .. } => {
                self.push_stack(Container::List(limit, ordered), end, end);
                self.list_more_item = true;
            }
            _ => (),
        }

        self.off += off + start;

        ele
    }

    // returns (event, offset, container limit, container end)
    fn real_next_ele(&self, text: &'a str) -> Option<(Event<'a>, usize, usize, usize)> {
        debug_assert!(!text.starts_with('\n'));

        if text.starts_with("[fn:") {
            if let Some((label, cont, off)) = fn_def::parse(text) {
                return Some((Event::FnDef { label, cont }, off + 1, 0, 0));
            }
        }

        let (tail, line_begin) = text
            .find(|c| c != ' ')
            .map(|off| (&text[off..], off))
            .unwrap_or((text, 0));

        if let Some(ordered) = list::is_item(tail) {
            return Some((Event::ListBeg { ordered }, 0, line_begin, text.len()));
        }

        // TODO: LaTeX environment
        if tail.starts_with("\\begin{") {}

        // rule
        if tail.starts_with("-----") {
            let off = rule::parse(tail);
            if off != 0 {
                return Some((Event::Rule, off + line_begin, 0, 0));
            }
        }

        if tail.starts_with(':') {
            if let Some((name, off, limit, end)) = drawer::parse(tail) {
                return Some((
                    Event::DrawerBeg(name),
                    off + line_begin,
                    limit + line_begin,
                    end + line_begin,
                ));
            }
        }

        // fixed width
        if tail.starts_with(": ") || tail.starts_with(":\n") {
            // let end = line_ends
            //     .skip_while(|&i| {
            //         text[i + 1..].starts_with(": ") || text[i + 1..].starts_with(":\n")
            //     })
            //     .next()
            //     .map(|i| i + 1)
            //     .unwrap_or_else(|| text.len());
            // let off = end - pos;
            // brk!(Element::FixedWidth(&tail[0..off]), off);
        }

        // comment
        if tail.starts_with("# ") || tail.starts_with("#\n") {
            // let end = line_ends
            //     .skip_while(|&i| {
            //         text[i + 1..].starts_with("# ") || text[i + 1..].starts_with("#\n")
            //     })
            //     .next()
            //     .map(|i| i + 1)
            //     .unwrap_or_else(|| text.len());
            // let off = end - pos;
            // brk!(Element::Comment(&tail[0..off]), off);
        }

        if tail.starts_with("#+") {
            block::parse(tail)
                .map(|(name, args, begin, limit, end)| {
                    let cont = &tail[begin..limit];
                    match &*name.to_uppercase() {
                        "COMMENT" => (Event::CommentBlock { args, cont }, end + line_begin, 0, 0),
                        "EXAMPLE" => (Event::ExampleBlock { args, cont }, end + line_begin, 0, 0),
                        "EXPORT" => (Event::ExportBlock { args, cont }, end + line_begin, 0, 0),
                        "SRC" => (Event::SrcBlock { args, cont }, end + line_begin, 0, 0),
                        "VERSE" => (Event::VerseBlock { args, cont }, end + line_begin, 0, 0),
                        "CENTER" => (
                            Event::CtrBlockBeg,
                            begin + line_begin,
                            limit + line_begin,
                            end + line_begin,
                        ),
                        "QUOTE" => (
                            Event::QteBlockBeg,
                            begin + line_begin,
                            limit + line_begin,
                            end + line_begin,
                        ),
                        _ => (
                            Event::SplBlockBeg { name, args },
                            begin + line_begin,
                            limit + line_begin,
                            end + line_begin,
                        ),
                    }
                })
                .or_else(|| {
                    dyn_block::parse(tail).map(|(name, args, begin, limit, end)| {
                        (
                            Event::DynBlockBeg { name, args },
                            begin + line_begin,
                            limit + line_begin,
                            end + line_begin,
                        )
                    })
                })
                .or_else(|| {
                    keyword::parse(tail).map(|(key, value, off)| {
                        if let Key::Call = key {
                            (Event::Call { value }, off + line_begin, 0, 0)
                        } else {
                            (Event::Keyword { key, value }, off + line_begin, 0, 0)
                        }
                    })
                })
        } else {
            None
        }
    }

    fn next_obj(&mut self, text: &'a str) -> Event<'a> {
        let (obj, off, limit, end) = self
            .obj_buf
            .take()
            .or_else(|| self.real_next_obj(text))
            .unwrap_or_else(|| {
                let bs = bytes!(b'@', b' ', b'"', b'(', b'\n', b'{', b'<', b'[');
                let bytes = text.as_bytes();
                let mut pos = 0;

                while let Some(off) = bs.find(&bytes[pos..]) {
                    pos += off + 1;

                    if let Some(buf) = self.real_next_obj(&text[pos..]) {
                        self.obj_buf = Some(buf);
                        return (Event::Text(&text[0..pos]), pos, 0, 0);
                    }
                }

                (Event::Text(text), text.len(), 0, 0)
            });

        debug_assert!(off <= text.len() && limit <= text.len() && end <= text.len());

        self.off += off;

        match obj {
            Event::UnderlineBeg => self.push_stack(Container::Underline, limit, end),
            Event::StrikeBeg => self.push_stack(Container::Strike, limit, end),
            Event::ItalicBeg => self.push_stack(Container::Italic, limit, end),
            Event::BoldBeg => self.push_stack(Container::Bold, limit, end),
            _ => (),
        }

        obj
    }

    fn real_next_obj(&mut self, text: &'a str) -> Option<(Event<'a>, usize, usize, usize)> {
        if text.len() < 3 {
            return None;
        }

        let bytes = text.as_bytes();
        match bytes[0] {
            b'@' if bytes[1] == b'@' => snippet::parse(text)
                .map(|(name, value, off)| (Event::Snippet { name, value }, off, 0, 0)),
            b'{' if bytes[1] == b'{' && bytes[2] == b'{' => macros::parse(text)
                .map(|(name, args, off)| (Event::Macros { name, args }, off, 0, 0)),
            b'<' if bytes[1] == b'<' => {
                if bytes[2] == b'<' {
                    radio_target::parse(text)
                        .map(|(target, off)| (Event::RadioTarget { target }, off, 0, 0))
                } else {
                    target::parse(text).map(|(target, off)| (Event::Target { target }, off, 0, 0))
                }
            }
            b'<' => timestamp::parse_active(text)
                .map(|(timestamp, off)| (Event::Timestamp(timestamp), off, 0, 0))
                .or_else(|| {
                    timestamp::parse_diary(text)
                        .map(|(timestamp, off)| (Event::Timestamp(timestamp), off, 0, 0))
                }),
            b'[' => {
                if text[1..].starts_with("fn:") {
                    fn_ref::parse(text)
                        .map(|(label, def, off)| (Event::FnRef { label, def }, off, 0, 0))
                } else if bytes[1] == b'[' {
                    link::parse(text)
                        .map(|(path, desc, off)| (Event::Link { path, desc }, off, 0, 0))
                } else {
                    cookie::parse(text)
                        .map(|(cookie, off)| (Event::Cookie(cookie), off, 0, 0))
                        .or_else(|| {
                            timestamp::parse_inactive(text)
                                .map(|(timestamp, off)| (Event::Timestamp(timestamp), off, 0, 0))
                        })
                }
            }
            b'{' | b' ' | b'"' | b',' | b'(' | b'\n' => self.next_inline(&text[1..]),
            _ => self.next_inline(text),
        }
    }

    fn next_inline(&mut self, text: &'a str) -> Option<(Event<'a>, usize, usize, usize)> {
        match text.as_bytes()[0] {
            b'*' => emphasis::parse(text, b'*').map(|end| (Event::BoldBeg, 1, end - 1, end)),
            b'+' => emphasis::parse(text, b'+').map(|end| (Event::StrikeBeg, 1, end - 1, end)),
            b'/' => emphasis::parse(text, b'/').map(|end| (Event::ItalicBeg, 1, end - 1, end)),
            b'_' => emphasis::parse(text, b'_').map(|end| (Event::UnderlineBeg, 1, end - 1, end)),
            b'=' => emphasis::parse(text, b'=')
                .map(|end| (Event::Verbatim(&text[1..end]), end + 1, 0, 0)),
            b'~' => {
                emphasis::parse(text, b'~').map(|end| (Event::Code(&text[1..end]), end + 1, 0, 0))
            }
            b's' if text.starts_with("src_") => {
                inline_src::parse(text).map(|(lang, option, body, off)| {
                    (Event::InlineSrc { lang, option, body }, off, 0, 0)
                })
            }
            b'c' if text.starts_with("call_") => {
                inline_call::parse(text).map(|(name, args, inside_header, end_header, off)| {
                    (
                        Event::InlineCall {
                            name,
                            args,
                            inside_header,
                            end_header,
                        },
                        off,
                        0,
                        0,
                    )
                })
            }
            _ => None,
        }
    }

    fn next_list_item(&mut self, ident: usize, text: &'a str) -> Event<'a> {
        let (bullet, off, limit, end, has_more) = list::parse(text, ident);
        self.push_stack(Container::ListItem, limit, end);
        self.off += off;
        self.list_more_item = has_more;
        Event::ListItemBeg { bullet }
    }

    #[inline]
    fn push_stack(&mut self, container: Container, limit: usize, end: usize) {
        self.stack
            .push((container, self.off + limit, self.off + end));
    }

    #[inline]
    fn end(&mut self) -> Event<'a> {
        let (container, _, _) = self.stack.pop().unwrap();
        match container {
            Container::Bold => Event::BoldEnd,
            Container::Drawer => Event::DrawerEnd,
            Container::CtrBlock => Event::CtrBlockEnd,
            Container::DynBlock => Event::DynBlockEnd,
            Container::Headline(_) => Event::HeadlineEnd,
            Container::Italic => Event::ItalicEnd,
            Container::List(_, ordered) => Event::ListEnd { ordered },
            Container::ListItem => Event::ListItemEnd,
            Container::Paragraph => Event::ParagraphEnd,
            Container::QteBlock => Event::QteBlockEnd,
            Container::Section(_) => Event::SectionEnd,
            Container::SplBlock => Event::SplBlockEnd,
            Container::Strike => Event::StrikeEnd,
            Container::Underline => Event::UnderlineEnd,
        }
    }
}

impl<'a> Iterator for Parser<'a> {
    type Item = Event<'a>;

    fn next(&mut self) -> Option<Event<'a>> {
        if let Some(&(container, limit, end)) = self.stack.last() {
            debug_assert!(
                self.off <= limit && limit <= end && end <= self.text.len(),
                "{} <= {} <= {} <= {}",
                self.off,
                limit,
                end,
                self.text.len()
            );
            Some(if self.off >= limit {
                self.off = end;
                self.end()
            } else {
                let tail = &self.text[self.off..limit];
                match container {
                    Container::Headline(beg) => {
                        debug_assert!(self.off >= beg);
                        if self.off == beg {
                            self.next_section_or_headline(tail)
                        } else {
                            self.next_headline(tail)
                        }
                    }
                    Container::Drawer
                    | Container::DynBlock
                    | Container::CtrBlock
                    | Container::QteBlock
                    | Container::SplBlock
                    | Container::ListItem => self.next_ele(tail),
                    Container::Section(beg) => {
                        // planning should be the first line of section
                        if self.off == beg {
                            if let Some((planning, off)) = Planning::parse(tail) {
                                self.off += off;
                                Event::Planning(planning)
                            } else {
                                self.next_ele(tail)
                            }
                        } else {
                            self.next_ele(tail)
                        }
                    }
                    Container::List(ident, _) => {
                        if self.list_more_item {
                            self.next_list_item(ident, tail)
                        } else {
                            self.end()
                        }
                    }
                    Container::Paragraph
                    | Container::Bold
                    | Container::Underline
                    | Container::Italic
                    | Container::Strike => self.next_obj(tail),
                }
            })
        } else if self.off < self.text.len() {
            Some(self.next_section_or_headline(&self.text[self.off..]))
        } else {
            None
        }
    }
}

#[test]
fn parse() {
    use self::Event::*;

    let expected = vec![
        HeadlineBeg(Headline {
            level: 1,
            priority: None,
            keyword: None,
            title: "Title 1",
            tags: None,
        }),
        SectionBeg,
        ParagraphBeg,
        BoldBeg,
        Text("Section 1"),
        BoldEnd,
        ParagraphEnd,
        SectionEnd,
        HeadlineBeg(Headline {
            level: 2,
            priority: None,
            keyword: None,
            title: "Title 2",
            tags: None,
        }),
        SectionBeg,
        ParagraphBeg,
        UnderlineBeg,
        Text("Section 2"),
        UnderlineEnd,
        ParagraphEnd,
        SectionEnd,
        HeadlineEnd,
        HeadlineEnd,
        HeadlineBeg(Headline {
            level: 1,
            priority: None,
            keyword: None,
            title: "Title 3",
            tags: None,
        }),
        SectionBeg,
        ParagraphBeg,
        ItalicBeg,
        Text("Section 3"),
        ItalicEnd,
        ParagraphEnd,
        SectionEnd,
        HeadlineEnd,
        HeadlineBeg(Headline {
            level: 1,
            priority: None,
            keyword: None,
            title: "Title 4",
            tags: None,
        }),
        SectionBeg,
        ParagraphBeg,
        Verbatim("Section 4"),
        ParagraphEnd,
        SectionEnd,
        HeadlineEnd,
    ];

    assert_eq!(
        Parser::new(
            r#"* Title 1
*Section 1*
** Title 2
_Section 2_
* Title 3
/Section 3/
* Title 4
=Section 4="#
        )
        .collect::<Vec<_>>(),
        expected
    );
}
