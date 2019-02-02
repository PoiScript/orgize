use elements::*;
use headline::*;
use objects::*;

#[cfg_attr(test, derive(PartialEq))]
#[derive(Copy, Clone, Debug)]
pub enum Container {
    Headline { beg: usize, end: usize },
    Section { end: usize },
    Paragraph { cont_end: usize, end: usize },
    CtrBlock { cont_end: usize, end: usize },
    QteBlock { cont_end: usize, end: usize },
    SplBlock { cont_end: usize, end: usize },
    DynBlock { cont_end: usize, end: usize },
    List { ident: usize, ordered: bool },
    ListItem { cont_end: usize, end: usize },
    Italic { end: usize },
    Strike { end: usize },
    Bold { end: usize },
    Underline { end: usize },
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
    has_more_item: bool,
}

impl<'a> Parser<'a> {
    pub fn new(text: &'a str) -> Parser<'a> {
        Parser {
            text,
            stack: Vec::new(),
            off: 0,
            ele_buf: None,
            obj_buf: None,
            has_more_item: false,
        }
    }

    fn next_sec_or_hdl(&mut self) -> Event<'a> {
        let end = Headline::find_level(&self.text[self.off..], std::usize::MAX);
        debug_assert!(end <= self.text.len());
        if end != 0 {
            self.stack.push(Container::Section {
                end: self.off + end,
            });
            Event::SectionBeg
        } else {
            self.next_hdl()
        }
    }

    fn next_hdl(&mut self) -> Event<'a> {
        let tail = &self.text[self.off..];
        let (hdl, off, end) = Headline::parse(tail);
        debug_assert!(end <= self.text.len());
        self.stack.push(Container::Headline {
            beg: self.off + off,
            end: self.off + end,
        });
        self.off += off;
        Event::HeadlineBeg(hdl)
    }

    fn next_ele(&mut self, text: &'a str) -> Event<'a> {
        let (ele, off) = self
            .ele_buf
            .take()
            .map(|(ele, off)| (Some(ele), off))
            .unwrap_or_else(|| {
                let (ele, off, next_ele) = Element::next_2(text);
                self.ele_buf = next_ele;
                (ele, off)
            });

        debug_assert!(off <= text.len());

        self.off += off;

        match ele {
            Some(Element::Paragraph { cont_end, end }) => {
                debug_assert!(cont_end <= text.len() && end <= text.len());
                self.stack.push(Container::Paragraph {
                    cont_end: cont_end + self.off,
                    end: end + self.off,
                });
                Event::ParagraphBeg
            }
            Some(Element::QteBlock { end, cont_end, .. }) => {
                debug_assert!(cont_end <= text.len() && end <= text.len());
                self.stack.push(Container::QteBlock {
                    cont_end: cont_end + self.off,
                    end: end + self.off,
                });
                Event::QteBlockBeg
            }
            Some(Element::CtrBlock { end, cont_end, .. }) => {
                debug_assert!(cont_end <= text.len() && end <= text.len());
                self.stack.push(Container::CtrBlock {
                    cont_end: cont_end + self.off,
                    end: end + self.off,
                });
                Event::CtrBlockBeg
            }
            Some(Element::SplBlock {
                name,
                args,
                end,
                cont_end,
            }) => {
                debug_assert!(cont_end <= text.len() && end <= text.len());
                self.stack.push(Container::SplBlock {
                    cont_end: cont_end + self.off,
                    end: end + self.off,
                });
                Event::SplBlockBeg { name, args }
            }
            Some(Element::DynBlock {
                name,
                args,
                cont_end,
                end,
            }) => {
                debug_assert!(cont_end <= text.len() && end <= text.len());
                self.stack.push(Container::DynBlock {
                    cont_end: cont_end + self.off,
                    end: end + self.off,
                });
                Event::DynBlockBeg { name, args }
            }
            Some(Element::List { ident, ordered }) => {
                self.stack.push(Container::List { ident, ordered });
                self.has_more_item = true;
                Event::ListBeg { ordered }
            }
            Some(Element::Call { value }) => Event::Call { value },
            Some(Element::Comment(c)) => Event::Comment(c),
            Some(Element::CommentBlock { args, cont }) => Event::CommentBlock { args, cont },
            Some(Element::ExampleBlock { args, cont }) => Event::ExampleBlock { args, cont },
            Some(Element::ExportBlock { args, cont }) => Event::ExportBlock { args, cont },
            Some(Element::FixedWidth(f)) => Event::FixedWidth(f),
            Some(Element::FnDef { label, cont }) => Event::FnDef { label, cont },
            Some(Element::Keyword { key, value }) => Event::Keyword { key, value },
            Some(Element::Rule) => Event::Rule,
            Some(Element::SrcBlock { args, cont }) => Event::SrcBlock { args, cont },
            Some(Element::VerseBlock { args, cont }) => Event::VerseBlock { args, cont },
            None => self.end(),
        }
    }

    fn next_obj(&mut self, text: &'a str) -> Event<'a> {
        let (obj, off) = self.obj_buf.take().unwrap_or_else(|| {
            let (obj, off, next_obj) = Object::next_2(text);
            self.obj_buf = next_obj;
            (obj, off)
        });

        debug_assert!(off <= text.len());

        match obj {
            Object::Underline { end } => {
                debug_assert!(end <= text.len());
                self.stack.push(Container::Underline {
                    end: self.off + end,
                });
            }
            Object::Strike { end } => {
                debug_assert!(end <= text.len());
                self.stack.push(Container::Strike {
                    end: self.off + end,
                });
            }
            Object::Italic { end } => {
                debug_assert!(end <= text.len());
                self.stack.push(Container::Italic {
                    end: self.off + end,
                });
            }
            Object::Bold { end } => {
                debug_assert!(end <= text.len());
                self.stack.push(Container::Bold {
                    end: self.off + end,
                });
            }
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

    fn next_list_item(&mut self, ident: usize) -> Event<'a> {
        let (bullet, cont_beg, cont_end, end, has_more) =
            List::parse(&self.text[self.off..], ident);
        self.stack.push(Container::ListItem {
            cont_end: self.off + cont_end,
            end: self.off + end,
        });
        self.off += cont_beg;
        self.has_more_item = has_more;
        Event::ListItemBeg { bullet }
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
}

impl<'a> Iterator for Parser<'a> {
    type Item = Event<'a>;

    fn next(&mut self) -> Option<Event<'a>> {
        if self.stack.is_empty() {
            if self.off >= self.text.len() {
                None
            } else {
                Some(self.next_sec_or_hdl())
            }
        } else {
            let last = *self.stack.last_mut().unwrap();

            Some(match last {
                Container::Headline { beg, end } => {
                    if self.off >= end {
                        self.end()
                    } else if self.off == beg {
                        self.next_sec_or_hdl()
                    } else {
                        self.next_hdl()
                    }
                }
                Container::DynBlock { cont_end, end, .. }
                | Container::CtrBlock { cont_end, end, .. }
                | Container::QteBlock { cont_end, end, .. }
                | Container::SplBlock { cont_end, end, .. }
                | Container::ListItem { cont_end, end } => {
                    let text = &self.text[self.off..cont_end];
                    if self.off >= cont_end {
                        self.off = end;
                        self.end()
                    } else {
                        self.next_ele(text)
                    }
                }
                Container::List { ident, .. } => {
                    if self.has_more_item {
                        self.next_list_item(ident)
                    } else {
                        self.end()
                    }
                }
                Container::Section { end } => {
                    let text = &self.text[self.off..end];
                    if self.off >= end {
                        self.end()
                    } else {
                        self.next_ele(text)
                    }
                }
                Container::Paragraph { cont_end, end } => {
                    let text = &self.text[self.off..cont_end];
                    if self.off >= cont_end {
                        self.off = end;
                        self.end()
                    } else {
                        self.next_obj(text)
                    }
                }
                Container::Bold { end }
                | Container::Underline { end }
                | Container::Italic { end }
                | Container::Strike { end } => {
                    let text = &self.text[self.off..end];
                    if self.off >= end {
                        self.off += 1;
                        self.end()
                    } else {
                        self.next_obj(text)
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
