use elements::*;
use headline::*;
use objects::*;

#[cfg_attr(test, derive(PartialEq))]
#[derive(Copy, Clone, Debug)]
pub enum Container {
    Block,
    Bold,
    Drawer,
    Headline { beg: usize, end: usize },
    Italic,
    LatexEnv,
    List,
    Paragraph,
    Section { end: usize },
    StrikeThrough,
    Table,
    Underline,
}

#[cfg_attr(test, derive(PartialEq, Debug))]
pub enum Event<'a> {
    StartHeadline(Headline<'a>),
    EndHeadline,

    StartSection,
    EndSection,

    Paragraph,
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

    Comment,

    TableStart,
    TableEnd,
    TableCell,

    LatexEnv,
    StrikeThrough,
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
    Bold(&'a str),
    Verbatim(&'a str),
    Italic(&'a str),
    Strike(&'a str),
    Underline(&'a str),
    Code(&'a str),

    Text(&'a str),
}

pub struct Parser<'a> {
    text: &'a str,
    stack: Vec<Container>,
    off: usize,
}

impl<'a> Parser<'a> {
    pub fn new(text: &'a str) -> Parser<'a> {
        Parser {
            text,
            stack: Vec::new(),
            off: 0,
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

    fn end_section(&mut self) -> Event<'a> {
        self.stack.pop();
        Event::EndSection
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

    fn end_headline(&mut self) -> Event<'a> {
        self.stack.pop();
        Event::EndHeadline
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
                        self.end_headline()
                    } else if self.off == beg {
                        self.start_section_or_headline(tail)
                    } else {
                        self.start_headline(tail)
                    }
                }
                Container::Section { end } => {
                    if self.off >= end {
                        self.end_section()
                    } else {
                        match Element::find_elem(&self.text[self.off..end]) {
                            (Element::Paragraph(_), off) => {
                                self.off += off;
                                Event::Paragraph
                            }
                        }
                    }
                }
                _ => unimplemented!(),
            })
        }
    }
}

#[test]
fn parse() {
    use self::Event::*;

    let expected = vec![
        StartHeadline(Headline::new(1, None, None, "Title 1", None)),
        StartSection,
        Paragraph,
        EndSection,
        StartHeadline(Headline::new(2, None, None, "Title 2", None)),
        StartSection,
        Paragraph,
        EndSection,
        EndHeadline,
        EndHeadline,
        StartHeadline(Headline::new(1, None, None, "Title 3", None)),
        StartSection,
        Paragraph,
        EndSection,
        EndHeadline,
        StartHeadline(Headline::new(1, None, None, "Title 4 ", None)),
        StartSection,
        Paragraph,
        EndSection,
        EndHeadline,
    ];

    assert_eq!(
        Parser::new("* Title 1\nSection 1\n** Title 2\nSection 2\n* Title 3\nSection 3\n* Title 4 \nSection 4")
            .collect::<Vec<_>>(),
        expected
    );
}
