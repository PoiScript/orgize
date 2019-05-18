//! Parser

use crate::{elements::*, headline::*, objects::*};
use jetscii::bytes;
use memchr::memchr_iter;

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
        indent: usize,
        ordered: bool,
    },
    ListEnd {
        indent: usize,
        ordered: bool,
    },
    ListItemBeg {
        bullet: &'a str,
    },
    ListItemEnd,

    Call {
        value: &'a str,
    },

    Clock(Clock<'a>),

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
    Keyword(Keyword<'a>),
    Rule,

    Timestamp(Timestamp<'a>),
    Cookie(Cookie<'a>),
    FnRef(FnRef<'a>),
    InlineCall(InlineCall<'a>),
    InlineSrc(InlineSrc<'a>),
    Link(Link<'a>),
    Macros(Macros<'a>),
    RadioTarget {
        target: &'a str,
    },
    Snippet(Snippet<'a>),
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
    todo_keywords: &'a [&'a str],
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
            todo_keywords: DEFAULT_TODO_KEYWORDS,
        }
    }

    /// creates a new parser from string, with the specified keywords
    pub fn with_todo_keywrods(text: &'a str, todo_keywords: &'a [&'a str]) -> Parser<'a> {
        Parser {
            text,
            stack: Vec::new(),
            off: 0,
            ele_buf: None,
            obj_buf: None,
            todo_keywords,
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

    /// set todo keywords
    pub fn set_todo_keywords(&mut self, todo_keywords: &'a [&'a str]) {
        self.todo_keywords = todo_keywords;
    }

    /// set text
    pub fn set_text(&mut self, text: &'a str) {
        self.off = 0;
        self.stack.clear();
        self.ele_buf = None;
        self.obj_buf = None;
        self.text = text;
    }

    /// skip the current container if exists and return its Event
    pub fn skip_container(&mut self) -> Option<Event<'a>> {
        let (container, _, end) = self.stack.pop()?;
        self.off = end;
        Some(match container {
            Container::Bold => Event::BoldEnd,
            Container::Drawer => Event::DrawerEnd,
            Container::CtrBlock => Event::CtrBlockEnd,
            Container::DynBlock => Event::DynBlockEnd,
            Container::Headline(_) => Event::HeadlineEnd,
            Container::Italic => Event::ItalicEnd,
            Container::List(indent, ordered) => Event::ListEnd { indent, ordered },
            Container::ListItem => Event::ListItemEnd,
            Container::Paragraph => Event::ParagraphEnd,
            Container::QteBlock => Event::QteBlockEnd,
            Container::Section(_) => Event::SectionEnd,
            Container::SplBlock => Event::SplBlockEnd,
            Container::Strike => Event::StrikeEnd,
            Container::Underline => Event::UnderlineEnd,
        })
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
        let (hdl, off, end) = Headline::parse(text, self.todo_keywords);
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
                for i in memchr_iter(b'\n', tail.as_bytes()) {
                    if tail.as_bytes()[pos..i].iter().all(u8::is_ascii_whitespace) {
                        return (Event::ParagraphBeg, 0, pos - 1 + start, i + 1 + start);
                    } else if let Some(buf) = self.real_next_ele(&tail[pos..]) {
                        self.ele_buf = Some(buf);
                        return (Event::ParagraphBeg, 0, pos - 1 + start, pos + start);
                    }
                    pos = i + 1;
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
            Event::ListBeg { ordered, indent } => {
                self.push_stack(Container::List(indent, ordered), limit, end)
            }
            _ => (),
        }

        self.off += off + start;

        ele
    }

    // returns (event, offset, container limit, container end)
    fn real_next_ele(&mut self, text: &'a str) -> Option<(Event<'a>, usize, usize, usize)> {
        debug_assert!(!text.starts_with('\n'));

        if let Some((label, cont, off)) = fn_def::parse(text) {
            return Some((Event::FnDef { label, cont }, off + 1, 0, 0));
        } else if let Some((indent, ordered, limit, end)) = list::parse(text) {
            return Some((Event::ListBeg { indent, ordered }, 0, limit, end));
        }

        let (tail, line_begin) = text
            .find(|c| c != ' ')
            .map(|off| (&text[off..], off))
            .unwrap_or((text, 0));

        if let Some((clock, off)) = Clock::parse(tail) {
            return Some((Event::Clock(clock), off + line_begin, 0, 0));
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
                    Keyword::parse(tail).map(|(key, option, value, off)| {
                        (
                            if key.eq_ignore_ascii_case("CALL") {
                                Event::Call { value }
                            } else {
                                Event::Keyword(Keyword::new(key, option, value))
                            },
                            off + line_begin,
                            0,
                            0,
                        )
                    })
                })
        } else {
            None
        }
    }

    fn next_obj(&mut self, text: &'a str) -> Event<'a> {
        let bytes = text.as_bytes();
        let (obj, off, limit, end) = self
            .obj_buf
            .take()
            .or_else(|| match bytes[0] {
                b'{' | b' ' | b'"' | b',' | b'(' | b'\n' => {
                    if let Some(buf) = self.real_next_obj(&text[1..]) {
                        self.obj_buf = Some(buf);
                        Some((Event::Text(&text[0..1]), 1, 0, 0))
                    } else {
                        None
                    }
                }
                _ => self.real_next_obj(text),
            })
            .unwrap_or_else(|| {
                let bs = bytes!(b'@', b' ', b'"', b'(', b'\n', b'{', b'<', b'[');
                let mut pos = 0;
                while let Some(off) = bs.find(&bytes[pos..]) {
                    pos += off;
                    match bytes[pos] {
                        b'{' | b' ' | b'"' | b',' | b'(' | b'\n' => {
                            if let Some(buf) = self.real_next_obj(&text[pos + 1..]) {
                                self.obj_buf = Some(buf);
                                return (Event::Text(&text[0..=pos]), pos + 1, 0, 0);
                            }
                        }
                        _ => {
                            if let Some(buf) = self.real_next_obj(&text[pos..]) {
                                self.obj_buf = Some(buf);
                                return (Event::Text(&text[0..pos]), pos, 0, 0);
                            }
                        }
                    }
                    pos += 1;
                }
                (Event::Text(text), text.len(), 0, 0)
            });

        debug_assert!(
            (limit == 0 && end == 0) || (off <= limit && limit <= end && end <= text.len()),
            "{} <= {} <= {} <= {}",
            off,
            limit,
            end,
            text.len()
        );

        match obj {
            Event::UnderlineBeg => self.push_stack(Container::Underline, limit, end),
            Event::StrikeBeg => self.push_stack(Container::Strike, limit, end),
            Event::ItalicBeg => self.push_stack(Container::Italic, limit, end),
            Event::BoldBeg => self.push_stack(Container::Bold, limit, end),
            _ => (),
        }

        self.off += off;

        obj
    }

    fn real_next_obj(&self, text: &'a str) -> Option<(Event<'a>, usize, usize, usize)> {
        if text.len() < 3 {
            None
        } else {
            let bytes = text.as_bytes();
            match bytes[0] {
                b'@' if bytes[1] == b'@' => {
                    Snippet::parse(text).map(|(snippet, off)| (Event::Snippet(snippet), off, 0, 0))
                }
                b'{' if bytes[1] == b'{' && bytes[2] == b'{' => {
                    Macros::parse(text).map(|(macros, off)| (Event::Macros(macros), off, 0, 0))
                }
                b'<' if bytes[1] == b'<' => {
                    if bytes[2] == b'<' {
                        radio_target::parse(text)
                            .map(|(target, off)| (Event::RadioTarget { target }, off, 0, 0))
                    } else {
                        target::parse(text)
                            .map(|(target, off)| (Event::Target { target }, off, 0, 0))
                    }
                }
                b'<' => Timestamp::parse_active(text)
                    .or_else(|| Timestamp::parse_diary(text))
                    .map(|(ts, off)| (Event::Timestamp(ts), off, 0, 0)),
                b'[' => {
                    if text[1..].starts_with("fn:") {
                        FnRef::parse(text).map(|(fn_ref, off)| (Event::FnRef(fn_ref), off, 0, 0))
                    } else if bytes[1] == b'[' {
                        Link::parse(text).map(|(link, off)| (Event::Link(link), off, 0, 0))
                    } else if let Some((cookie, off)) = Cookie::parse(text) {
                        Some((Event::Cookie(cookie), off, 0, 0))
                    } else {
                        Timestamp::parse_inactive(text)
                            .map(|(ts, off)| (Event::Timestamp(ts), off, 0, 0))
                    }
                }
                b'*' => emphasis::parse(text, b'*').map(|end| (Event::BoldBeg, 1, end - 1, end)),
                b'+' => emphasis::parse(text, b'+').map(|end| (Event::StrikeBeg, 1, end - 1, end)),
                b'/' => emphasis::parse(text, b'/').map(|end| (Event::ItalicBeg, 1, end - 1, end)),
                b'_' => {
                    emphasis::parse(text, b'_').map(|end| (Event::UnderlineBeg, 1, end - 1, end))
                }
                b'=' => emphasis::parse(text, b'=')
                    .map(|end| (Event::Verbatim(&text[1..end - 1]), end, 0, 0)),
                b'~' => emphasis::parse(text, b'~')
                    .map(|end| (Event::Code(&text[1..end - 1]), end, 0, 0)),
                b's' if text.starts_with("src_") => {
                    InlineSrc::parse(text).map(|(src, off)| (Event::InlineSrc(src), off, 0, 0))
                }
                b'c' if text.starts_with("call_") => {
                    InlineCall::parse(text).map(|(call, off)| (Event::InlineCall(call), off, 0, 0))
                }
                _ => None,
            }
        }
    }

    fn next_list_item(&self, text: &'a str, indent: usize) -> (&'a str, usize, usize, usize) {
        use std::iter::once;

        debug_assert!(&text[0..indent].trim().is_empty());
        let off = &text[indent..].find(' ').unwrap() + 1 + indent;

        let bytes = text.as_bytes();
        let mut lines = memchr_iter(b'\n', bytes)
            .map(|i| i + 1)
            .chain(once(text.len()));
        let mut pos = lines.next().unwrap();

        for i in lines {
            let line = &text[pos..i];
            if let Some(line_indent) = line.find(|c: char| !c.is_whitespace()) {
                if line_indent == indent {
                    return (&text[indent..off], off, pos, pos);
                }
            }
            pos = i;
        }

        (&text[indent..off], off, text.len(), text.len())
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
            Container::List(indent, ordered) => Event::ListEnd { indent, ordered },
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
            // eprint!("{:1$}", ' ', self.stack_depth());

            debug_assert!(
                self.off <= limit && limit <= end && end <= self.text.len(),
                "{} <= {} <= {} <= {}",
                self.off,
                limit,
                end,
                self.text.len()
            );

            let tail = &self.text[self.off..limit];

            // eprintln!("{:?} {:?}", container, tail);

            Some(match container {
                Container::Headline(beg) => {
                    if self.off >= limit {
                        self.off = end;
                        self.stack.pop();
                        Event::HeadlineEnd
                    } else if self.off == beg {
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
                | Container::ListItem => {
                    if self.off >= limit {
                        self.off = end;
                        self.end()
                    } else {
                        self.next_ele(tail)
                    }
                }
                Container::Section(beg) => {
                    // planning should be the first line of section
                    if self.off >= limit {
                        self.off = end;
                        self.stack.pop();
                        Event::SectionEnd
                    } else if self.off == beg {
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
                Container::List(indent, ordered) => {
                    if self.off < limit {
                        let (bullet, off, limit, end) = self.next_list_item(tail, indent);
                        self.push_stack(Container::ListItem, limit, end);
                        self.off += off;
                        Event::ListItemBeg { bullet }
                    } else {
                        self.off = end;
                        self.stack.pop();
                        Event::ListEnd { indent, ordered }
                    }
                }
                Container::Paragraph
                | Container::Bold
                | Container::Underline
                | Container::Italic
                | Container::Strike => {
                    if self.off >= limit {
                        self.off = end;
                        self.end()
                    } else {
                        self.next_obj(tail)
                    }
                }
            })
        } else if self.off < self.text.len() {
            Some(self.next_section_or_headline(&self.text[self.off..]))
        } else {
            None
        }
    }
}
