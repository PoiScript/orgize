use elements::*;
use headline::*;
use objects::*;

#[cfg_attr(test, derive(PartialEq))]
#[derive(Copy, Clone, Debug)]
pub enum Container {
    Headline {
        beg: usize,
        end: usize,
    },
    Section {
        end: usize,
    },

    Paragraph {
        cont_end: usize,
        end: usize,
    },
    CtrBlock {
        cont_end: usize,
        end: usize,
    },
    QteBlock {
        cont_end: usize,
        end: usize,
    },
    SplBlock {
        cont_end: usize,
        end: usize,
    },
    DynBlock {
        cont_end: usize,
        end: usize,
    },

    List {
        ident: usize,
        ordered: bool,
        cont_end: usize,
        end: usize,
    },
    ListItem {
        end: usize,
    },

    Italic {
        end: usize,
    },
    Strike {
        end: usize,
    },
    Bold {
        end: usize,
    },
    Underline {
        end: usize,
    },
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
    ListItemBeg,
    ListItemEnd,

    AffKeywords,

    Call,

    Clock,

    Comment(&'a str),
    FixedWidth(&'a str),

    TableStart,
    TableEnd,
    TableCell,

    LatexEnv,
    FnDef {
        label: &'a str,
        cont: &'a str,
    },
    Keyword {
        key: &'a str,
        value: &'a str,
    },
    Rule,

    Cookie(Cookie<'a>),
    FnRef(FnRef<'a>),
    InlineCall(InlineCall<'a>),
    InlineSrc(InlineSrc<'a>),
    Link(Link<'a>),
    Macros(Macros<'a>),
    RadioTarget(RadioTarget<'a>),
    Snippet(Snippet<'a>),
    Target(Target<'a>),

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
    stack: Vec<Container>,
    off: usize,
    ele_buf: Option<(Element<'a>, usize)>,
    obj_buf: Option<(Object<'a>, usize)>,
}

impl<'a> Parser<'a> {
    pub fn new(text: &'a str) -> Parser<'a> {
        Parser {
            text,
            stack: Vec::new(),
            off: 0,
            ele_buf: None,
            obj_buf: None,
        }
    }

    fn start_sec_or_hdl(&mut self, tail: &'a str) -> Event<'a> {
        let end = Headline::find_level(tail, std::usize::MAX);
        if end != 0 {
            self.stack.push(Container::Section {
                end: self.off + end,
            });
            Event::SectionBeg
        } else {
            self.start_hdl(tail)
        }
    }

    fn start_hdl(&mut self, tail: &'a str) -> Event<'a> {
        let (hdl, off, end) = Headline::parse(tail);
        self.stack.push(Container::Headline {
            beg: self.off + off,
            end: self.off + end,
        });
        self.off += off;
        Event::HeadlineBeg(hdl)
    }

    fn next_ele(&mut self, end: usize) -> Event<'a> {
        let (ele, off) = self
            .ele_buf
            .take()
            .map(|(ele, off)| (Some(ele), off))
            .unwrap_or_else(|| {
                let (off, ele, next_2) = Element::next_2(&self.text[self.off..end]);
                self.ele_buf = next_2;
                (ele, off)
            });

        if let Some(ele) = ele {
            match ele {
                Element::Paragraph { cont_end, end } => self.stack.push(Container::Paragraph {
                    cont_end: cont_end + self.off,
                    end: end + self.off,
                }),
                Element::QteBlock { end, cont_end, .. } => self.stack.push(Container::QteBlock {
                    cont_end: cont_end + self.off,
                    end: end + self.off,
                }),
                Element::CtrBlock { end, cont_end, .. } => self.stack.push(Container::CtrBlock {
                    cont_end: cont_end + self.off,
                    end: end + self.off,
                }),
                Element::SplBlock { end, cont_end, .. } => self.stack.push(Container::SplBlock {
                    cont_end: cont_end + self.off,
                    end: end + self.off,
                }),
                Element::DynBlock { end, cont_end, .. } => self.stack.push(Container::DynBlock {
                    cont_end: cont_end + self.off,
                    end: end + self.off,
                }),
                Element::List {
                    ident,
                    ordered,
                    cont_end,
                    end,
                } => self.stack.push(Container::List {
                    ident,
                    ordered,
                    cont_end: cont_end + self.off,
                    end: end + self.off,
                }),
                _ => (),
            }

            self.off += off;

            match ele {
                Element::Comment(c) => Event::Comment(c),
                Element::CommentBlock { args, cont } => Event::CommentBlock { args, cont },
                Element::CtrBlock { .. } => Event::CtrBlockBeg,
                Element::DynBlock { name, args, .. } => Event::DynBlockBeg { name, args },
                Element::ExampleBlock { args, cont } => Event::ExampleBlock { args, cont },
                Element::ExportBlock { args, cont } => Event::ExportBlock { args, cont },
                Element::FixedWidth(f) => Event::FixedWidth(f),
                Element::FnDef { label, cont } => Event::FnDef { label, cont },
                Element::Keyword { key, value } => Event::Keyword { key, value },
                Element::List { ordered, .. } => Event::ListBeg { ordered },
                Element::Paragraph { .. } => Event::ParagraphBeg,
                Element::QteBlock { .. } => Event::QteBlockBeg,
                Element::Rule => Event::Rule,
                Element::SplBlock { name, args, .. } => Event::SplBlockBeg { name, args },
                Element::SrcBlock { args, cont } => Event::SrcBlock { args, cont },
                Element::VerseBlock { args, cont } => Event::VerseBlock { args, cont },
            }
        } else {
            self.off += off;
            self.end()
        }
    }

    fn next_obj(&mut self, end: usize) -> Event<'a> {
        let (obj, off) = self.obj_buf.take().unwrap_or_else(|| {
            let (obj, off, next_2) = Object::next_2(&self.text[self.off..end]);
            self.obj_buf = next_2;
            (obj, off)
        });

        match obj {
            Object::Underline { end } => self.stack.push(Container::Underline {
                end: self.off + end,
            }),
            Object::Strike { end } => self.stack.push(Container::Strike {
                end: self.off + end,
            }),
            Object::Italic { end } => self.stack.push(Container::Italic {
                end: self.off + end,
            }),
            Object::Bold { end } => self.stack.push(Container::Bold {
                end: self.off + end,
            }),
            _ => (),
        }

        self.off += off;

        match obj {
            Object::Bold { .. } => Event::BoldBeg,
            Object::Code(c) => Event::Code(c),
            Object::Cookie(c) => Event::Cookie(c),
            Object::FnRef(f) => Event::FnRef(f),
            Object::InlineCall(i) => Event::InlineCall(i),
            Object::InlineSrc(i) => Event::InlineSrc(i),
            Object::Italic { .. } => Event::ItalicBeg,
            Object::Link(l) => Event::Link(l),
            Object::Macros(m) => Event::Macros(m),
            Object::RadioTarget(r) => Event::RadioTarget(r),
            Object::Snippet(s) => Event::Snippet(s),
            Object::Strike { .. } => Event::StrikeBeg,
            Object::Target(t) => Event::Target(t),
            Object::Text(t) => Event::Text(t),
            Object::Underline { .. } => Event::UnderlineBeg,
            Object::Verbatim(v) => Event::Verbatim(v),
        }
    }

    fn next_list_item(&mut self, end: usize, ident: usize) -> Event<'a> {
        let (beg, end) = List::parse_item(&self.text[self.off..end], ident);
        self.stack.push(Container::ListItem {
            end: self.off + end,
        });
        self.off += beg;
        Event::ListItemBeg
    }

    fn end(&mut self) -> Event<'a> {
        match self.stack.pop().unwrap() {
            Container::Bold { .. } => Event::BoldEnd,
            Container::CtrBlock { .. } => Event::CtrBlockEnd,
            Container::DynBlock { .. } => Event::DynBlockEnd,
            Container::Headline { .. } => Event::HeadlineEnd,
            Container::Italic { .. } => Event::ItalicEnd,
            Container::List { ordered, .. } => Event::ListEnd { ordered },
            Container::ListItem { .. } => Event::ListItemEnd,
            Container::Paragraph { .. } => Event::ParagraphEnd,
            Container::QteBlock { .. } => Event::QteBlockEnd,
            Container::Section { .. } => Event::SectionEnd,
            Container::SplBlock { .. } => Event::SplBlockEnd,
            Container::Strike { .. } => Event::StrikeEnd,
            Container::Underline { .. } => Event::UnderlineEnd,
        }
    }

    fn check_off(&self) {
        use self::Container::*;

        if let Some(container) = self.stack.last() {
            match *container {
                Headline { end, .. }
                | Section { end }
                | List { end, .. }
                | ListItem { end }
                | Italic { end }
                | Strike { end }
                | Bold { end }
                | Underline { end } => {
                    debug_assert!(self.off <= end);
                }
                Paragraph { cont_end, end } => {
                    debug_assert!(self.off <= end);
                    debug_assert!(self.off <= cont_end);
                }
                CtrBlock { cont_end, end }
                | QteBlock { cont_end, end }
                | SplBlock { cont_end, end }
                | DynBlock { cont_end, end } => {
                    debug_assert!(self.off <= cont_end);
                    debug_assert!(self.off <= end);
                }
            }
        }
    }
}

impl<'a> Iterator for Parser<'a> {
    type Item = Event<'a>;

    fn next(&mut self) -> Option<Event<'a>> {
        // self.check_off();

        if self.stack.is_empty() {
            if self.off >= self.text.len() {
                None
            } else {
                let tail = &self.text[self.off..];
                Some(self.start_sec_or_hdl(tail))
            }
        } else {
            let last = *self.stack.last_mut().unwrap();

            Some(match last {
                Container::Headline { beg, end } => {
                    let tail = &self.text[self.off..];
                    if self.off >= end {
                        self.end()
                    } else if self.off == beg {
                        self.start_sec_or_hdl(tail)
                    } else {
                        self.start_hdl(tail)
                    }
                }
                Container::DynBlock { cont_end, end, .. }
                | Container::CtrBlock { cont_end, end, .. }
                | Container::QteBlock { cont_end, end, .. }
                | Container::SplBlock { cont_end, end, .. } => {
                    if self.off >= cont_end {
                        self.off = end;
                        self.end()
                    } else {
                        self.next_ele(cont_end)
                    }
                }
                Container::List {
                    cont_end,
                    end,
                    ident,
                    ..
                } => {
                    if self.off >= cont_end {
                        self.off = end;
                        self.end()
                    } else {
                        self.next_list_item(cont_end, ident)
                    }
                }
                Container::ListItem { end } => {
                    if self.off >= end {
                        self.end()
                    } else {
                        self.next_ele(end)
                    }
                }
                Container::Section { end } => {
                    if self.off >= end {
                        self.end()
                    } else {
                        self.next_ele(end)
                    }
                }
                Container::Paragraph { cont_end, end } => {
                    if self.off >= cont_end {
                        self.off = end;
                        self.end()
                    } else {
                        self.next_obj(cont_end)
                    }
                }
                Container::Bold { end }
                | Container::Underline { end }
                | Container::Italic { end }
                | Container::Strike { end } => {
                    if self.off >= end {
                        self.off += 1;
                        self.end()
                    } else {
                        self.next_obj(end)
                    }
                }
            })
        }
    }
}

#[test]
fn parse() {
    use self::Event::*;

    let expected = vec![
        HeadlineBeg(Headline::new(1, None, None, "Title 1", None)),
        SectionBeg,
        ParagraphBeg,
        BoldBeg,
        Text("Section 1"),
        BoldEnd,
        ParagraphEnd,
        SectionEnd,
        HeadlineBeg(Headline::new(2, None, None, "Title 2", None)),
        SectionBeg,
        ParagraphBeg,
        UnderlineBeg,
        Text("Section 2"),
        UnderlineEnd,
        ParagraphEnd,
        SectionEnd,
        HeadlineEnd,
        HeadlineEnd,
        HeadlineBeg(Headline::new(1, None, None, "Title 3", None)),
        SectionBeg,
        ParagraphBeg,
        ItalicBeg,
        Text("Section 3"),
        ItalicEnd,
        ParagraphEnd,
        SectionEnd,
        HeadlineEnd,
        HeadlineBeg(Headline::new(1, None, None, "Title 4", None)),
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
