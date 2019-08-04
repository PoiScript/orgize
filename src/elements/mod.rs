//! Org-mode elements

mod block;
mod clock;
mod cookie;
mod drawer;
mod dyn_block;
mod emphasis;
mod fn_def;
mod fn_ref;
mod inline_call;
mod inline_src;
mod keyword;
mod link;
mod list;
mod macros;
mod planning;
mod radio_target;
mod rule;
mod snippet;
mod target;
mod timestamp;
mod title;

pub(crate) use block::Block;
pub(crate) use emphasis::parse as parse_emphasis;

pub use self::{
    block::{
        CenterBlock, CommentBlock, ExampleBlock, ExportBlock, QuoteBlock, SourceBlock,
        SpecialBlock, VerseBlock,
    },
    clock::Clock,
    cookie::Cookie,
    drawer::Drawer,
    dyn_block::DynBlock,
    fn_def::FnDef,
    fn_ref::FnRef,
    inline_call::InlineCall,
    inline_src::InlineSrc,
    keyword::{BabelCall, Keyword},
    link::Link,
    list::{List, ListItem},
    macros::Macros,
    planning::Planning,
    radio_target::RadioTarget,
    rule::Rule,
    snippet::Snippet,
    target::Target,
    timestamp::{Datetime, Timestamp},
    title::Title,
};

/// Org-mode element enum
#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(tag = "type", rename_all = "snake_case"))]
pub enum Element<'a> {
    SpecialBlock(SpecialBlock<'a>),
    QuoteBlock(QuoteBlock<'a>),
    CenterBlock(CenterBlock<'a>),
    VerseBlock(VerseBlock<'a>),
    CommentBlock(CommentBlock<'a>),
    ExampleBlock(ExampleBlock<'a>),
    ExportBlock(ExportBlock<'a>),
    SourceBlock(SourceBlock<'a>),
    BabelCall(BabelCall<'a>),
    Section,
    Clock(Clock<'a>),
    Cookie(Cookie<'a>),
    RadioTarget(RadioTarget),
    Drawer(Drawer<'a>),
    Document,
    DynBlock(DynBlock<'a>),
    FnDef(FnDef<'a>),
    FnRef(FnRef<'a>),
    Headline,
    InlineCall(InlineCall<'a>),
    InlineSrc(InlineSrc<'a>),
    Keyword(Keyword<'a>),
    Link(Link<'a>),
    List(List),
    ListItem(ListItem<'a>),
    Macros(Macros<'a>),
    Planning(Planning<'a>),
    Snippet(Snippet<'a>),
    Text { value: &'a str },
    Paragraph,
    Rule,
    Timestamp(Timestamp<'a>),
    Target(Target<'a>),
    Bold,
    Strike,
    Italic,
    Underline,
    Verbatim { value: &'a str },
    Code { value: &'a str },
    Comment { value: &'a str },
    FixedWidth { value: &'a str },
    Title(Title<'a>),
}

impl Element<'_> {
    pub fn is_container(&self) -> bool {
        match self {
            Element::SpecialBlock(_)
            | Element::QuoteBlock(_)
            | Element::CenterBlock(_)
            | Element::VerseBlock(_)
            | Element::Bold
            | Element::Document
            | Element::DynBlock(_)
            | Element::Headline
            | Element::Italic
            | Element::List(_)
            | Element::ListItem(_)
            | Element::Paragraph
            | Element::Section
            | Element::Strike
            | Element::Underline
            | Element::Title(_) => true,
            _ => false,
        }
    }
}

macro_rules! impl_from {
    ($($ele0:ident),*; $($ele1:ident),*) => {
        $(
            impl<'a> From<$ele0<'a>> for Element<'a> {
                fn from(ele: $ele0<'a>) -> Element<'a> {
                    Element::$ele0(ele)
                }
            }
        )*
        $(
            impl<'a> From<$ele1> for Element<'a> {
                fn from(ele: $ele1) -> Element<'a> {
                    Element::$ele1(ele)
                }
            }
        )*
    };
}

impl_from!(
    BabelCall,
    CenterBlock,
    Clock,
    CommentBlock,
    Cookie,
    Drawer,
    DynBlock,
    ExampleBlock,
    ExportBlock,
    FnDef,
    FnRef,
    InlineCall,
    InlineSrc,
    Keyword,
    Link,
    ListItem,
    Macros,
    Planning,
    QuoteBlock,
    Snippet,
    SourceBlock,
    SpecialBlock,
    Target,
    Timestamp,
    VerseBlock;
    RadioTarget,
    List
);
