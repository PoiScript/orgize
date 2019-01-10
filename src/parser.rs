use elements::*;
use headline::*;
use objects::*;

#[cfg_attr(test, derive(PartialEq))]
#[derive(Copy, Clone, Debug)]
pub enum Container {
    Headline { beg: usize, end: usize },
    Section { end: usize },

    Paragraph { end: usize, trailing: usize },

    Block,
    Drawer,
    LatexEnv,
    List,
    Table,

    Italic { end: usize },
    Strike { end: usize },
    Bold { end: usize },
    Underline { end: usize },
}

#[cfg_attr(test, derive(PartialEq, Debug))]
pub enum Event<'a> {
    StartHeadline(Headline<'a>),
    EndHeadline,

    StartSection,
    EndSection,

    StartParagraph,
    EndParagraph,

    BlockStart,
    BlockEnd,
    DynBlockStart,
    DynBlockEnd,
    ListStart,
    ListEnd,
    ParagraphStart,
    ParagraphEnd,

    AffKeywords,

    Call,

    Clock,

    Comment(&'a str),

    TableStart,
    TableEnd,
    TableCell,

    LatexEnv,
    FnDef(FnDef<'a>),
    Keyword(Keyword<'a>),
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

    StartBold,
    EndBold,
    StartItalic,
    EndItalic,
    StartStrike,
    EndStrike,
    StartUnderline,
    EndUnderline,

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

    fn start_section_or_headline(&mut self, tail: &'a str) -> Event<'a> {
        let end = Headline::find_level(tail, std::usize::MAX);
        if end != 0 {
            self.stack.push(Container::Section {
                end: self.off + end,
            });
            Event::StartSection
        } else {
            self.start_headline(tail)
        }
    }

    fn start_headline(&mut self, tail: &'a str) -> Event<'a> {
        let (hdl, off, end) = Headline::parse(tail);
        self.stack.push(Container::Headline {
            beg: self.off + off,
            end: self.off + end,
        });
        self.off += off;
        Event::StartHeadline(hdl)
    }

    fn next_ele(&mut self, end: usize) -> Event<'a> {
        let (ele, off) = if let Some((ele, off)) = std::mem::replace(&mut self.ele_buf, None) {
            (Some(ele), off)
        } else {
            let (off, ele, next_2) = Element::next_2(&self.text[self.off..end]);
            self.ele_buf = next_2;
            (ele, off)
        };

        self.off += off;

        if let Some(ele) = ele {
            if let Element::Paragraph { end, trailing } = ele {
                self.stack.push(Container::Paragraph {
                    end: end + self.off - off,
                    trailing: trailing + self.off - off,
                });
            }
            ele.into()
        } else {
            self.end()
        }
    }

    fn next_obj(&mut self, end: usize) -> Event<'a> {
        let (obj, off) = if let Some((obj, off)) = std::mem::replace(&mut self.obj_buf, None) {
            (obj, off)
        } else {
            let (obj, off, next_2) = Object::next_2(&self.text[self.off..end]);
            self.obj_buf = next_2;
            (obj, off)
        };

        self.off += off;

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

        obj.into()
    }

    fn end(&mut self) -> Event<'a> {
        match self.stack.pop().unwrap() {
            Container::Paragraph { .. } => Event::EndParagraph,
            Container::Underline { .. } => Event::EndUnderline,
            Container::Section { .. } => Event::EndSection,
            Container::Strike { .. } => Event::EndStrike,
            Container::Headline { .. } => Event::EndHeadline,
            Container::Italic { .. } => Event::EndItalic,
            Container::Bold { .. } => Event::EndBold,
            _ => unimplemented!(),
        }
    }
}

impl<'a> Iterator for Parser<'a> {
    type Item = Event<'a>;

    fn next(&mut self) -> Option<Event<'a>> {
        let tail = &self.text[self.off..];

        if self.stack.is_empty() {
            if self.off >= self.text.len() {
                None
            } else {
                Some(self.start_section_or_headline(tail))
            }
        } else {
            let last = *self.stack.last_mut()?;

            Some(match last {
                Container::Headline { beg, end } => {
                    if self.off >= end {
                        self.end()
                    } else if self.off == beg {
                        self.start_section_or_headline(tail)
                    } else {
                        self.start_headline(tail)
                    }
                }
                Container::Section { end } => {
                    if self.off >= end {
                        self.end()
                    } else {
                        self.next_ele(end)
                    }
                }
                Container::Paragraph { end, trailing } => {
                    if self.off >= end {
                        self.off = trailing;
                        self.end()
                    } else {
                        self.next_obj(end)
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
                _ => unimplemented!(),
            })
        }
    }
}

impl<'a> From<Object<'a>> for Event<'a> {
    fn from(obj: Object<'a>) -> Self {
        match obj {
            Object::Bold { .. } => Event::StartBold,
            Object::Code(c) => Event::Code(c),
            Object::Cookie(c) => Event::Cookie(c),
            Object::FnRef(f) => Event::FnRef(f),
            Object::InlineCall(i) => Event::InlineCall(i),
            Object::InlineSrc(i) => Event::InlineSrc(i),
            Object::Italic { .. } => Event::StartItalic,
            Object::Link(l) => Event::Link(l),
            Object::Macros(m) => Event::Macros(m),
            Object::RadioTarget(r) => Event::RadioTarget(r),
            Object::Snippet(s) => Event::Snippet(s),
            Object::Strike { .. } => Event::StartStrike,
            Object::Target(t) => Event::Target(t),
            Object::Text(t) => Event::Text(t),
            Object::Underline { .. } => Event::StartUnderline,
            Object::Verbatim(v) => Event::Verbatim(v),
        }
    }
}

impl<'a> From<Element<'a>> for Event<'a> {
    fn from(ele: Element<'a>) -> Self {
        match ele {
            Element::Comment(c) => Event::Comment(c),
            Element::FnDef(fd) => Event::FnDef(fd),
            Element::Keyword(kw) => Event::Keyword(kw),
            Element::Paragraph { .. } => Event::StartParagraph,
            Element::Rule => Event::Rule,
        }
    }
}

#[test]
fn parse() {
    use self::Event::*;

    let expected = vec![
        StartHeadline(Headline::new(1, None, None, "Title 1", None)),
        StartSection,
        StartParagraph,
        StartBold,
        Text("Section 1"),
        EndBold,
        EndParagraph,
        EndSection,
        StartHeadline(Headline::new(2, None, None, "Title 2", None)),
        StartSection,
        StartParagraph,
        StartUnderline,
        Text("Section 2"),
        EndUnderline,
        EndParagraph,
        EndSection,
        EndHeadline,
        EndHeadline,
        StartHeadline(Headline::new(1, None, None, "Title 3", None)),
        StartSection,
        StartParagraph,
        StartItalic,
        Text("Section 3"),
        EndItalic,
        EndParagraph,
        EndSection,
        EndHeadline,
        StartHeadline(Headline::new(1, None, None, "Title 4", None)),
        StartSection,
        StartParagraph,
        Verbatim("Section 4"),
        EndParagraph,
        EndSection,
        EndHeadline,
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
