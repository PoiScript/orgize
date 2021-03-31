//! Org-mode elements

pub(crate) mod block;
pub(crate) mod clock;
pub(crate) mod comment;
pub(crate) mod cookie;
pub(crate) mod drawer;
pub(crate) mod dyn_block;
pub(crate) mod emphasis;
pub(crate) mod fixed_width;
pub(crate) mod fn_def;
pub(crate) mod fn_ref;
pub(crate) mod inline_call;
pub(crate) mod inline_src;
pub(crate) mod keyword;
pub(crate) mod latex;
pub(crate) mod link;
pub(crate) mod list;
pub(crate) mod macros;
pub(crate) mod planning;
pub(crate) mod radio_target;
pub(crate) mod rule;
pub(crate) mod snippet;
pub(crate) mod table;
pub(crate) mod target;
pub(crate) mod timestamp;
pub(crate) mod title;

pub use self::{
    block::{
        CenterBlock, CommentBlock, ExampleBlock, ExportBlock, QuoteBlock, SourceBlock,
        SpecialBlock, VerseBlock,
    },
    clock::Clock,
    comment::Comment,
    cookie::Cookie,
    drawer::Drawer,
    dyn_block::DynBlock,
    fixed_width::FixedWidth,
    fn_def::FnDef,
    fn_ref::FnRef,
    inline_call::InlineCall,
    inline_src::InlineSrc,
    keyword::{BabelCall, Keyword},
    latex::LatexEnvironment,
    link::Link,
    list::{List, ListItem},
    macros::Macros,
    planning::Planning,
    rule::Rule,
    snippet::Snippet,
    table::{Table, TableCell, TableRow},
    target::Target,
    timestamp::{Datetime, Timestamp},
    title::Title,
};

use std::borrow::Cow;

/// Element Enum
#[derive(Debug)]
#[cfg_attr(feature = "ser", derive(serde::Serialize))]
#[cfg_attr(feature = "ser", serde(tag = "type", rename_all = "kebab-case"))]
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
    RadioTarget,
    Drawer(Drawer<'a>),
    Document { pre_blank: usize },
    DynBlock(DynBlock<'a>),
    FnDef(FnDef<'a>),
    FnRef(FnRef<'a>),
    Headline { level: usize },
    InlineCall(InlineCall<'a>),
    InlineSrc(InlineSrc<'a>),
    Keyword(Keyword<'a>),
    Link(Link<'a>),
    List(List),
    ListItem(ListItem<'a>),
    LatexEnvironment(LatexEnvironment<'a>),
    Macros(Macros<'a>),
    Snippet(Snippet<'a>),
    Text { value: Cow<'a, str> },
    Paragraph { post_blank: usize },
    Rule(Rule),
    Timestamp(Timestamp<'a>),
    Target(Target<'a>),
    Bold,
    Strike,
    Italic,
    Underline,
    Verbatim { value: Cow<'a, str> },
    Code { value: Cow<'a, str> },
    Comment(Comment<'a>),
    FixedWidth(FixedWidth<'a>),
    Title(Title<'a>),
    Table(Table<'a>),
    TableRow(TableRow),
    TableCell(TableCell),
}

impl Element<'_> {
    pub fn is_container(&self) -> bool {
        match self {
            Element::SpecialBlock(_)
            | Element::QuoteBlock(_)
            | Element::CenterBlock(_)
            | Element::VerseBlock(_)
            | Element::Bold
            | Element::Document { .. }
            | Element::DynBlock(_)
            | Element::Headline { .. }
            | Element::Italic
            | Element::List(_)
            | Element::ListItem(_)
            | Element::Paragraph { .. }
            | Element::Section
            | Element::Strike
            | Element::Underline
            | Element::Title(_)
            | Element::Table(_)
            | Element::TableRow(TableRow::Header)
            | Element::TableRow(TableRow::Body)
            | Element::TableCell(_) => true,
            _ => false,
        }
    }

    pub fn into_owned(self) -> Element<'static> {
        use Element::*;

        match self {
            LatexEnvironment(e) => LatexEnvironment(e.into_owned()),
            SpecialBlock(e) => SpecialBlock(e.into_owned()),
            QuoteBlock(e) => QuoteBlock(e.into_owned()),
            CenterBlock(e) => CenterBlock(e.into_owned()),
            VerseBlock(e) => VerseBlock(e.into_owned()),
            CommentBlock(e) => CommentBlock(e.into_owned()),
            ExampleBlock(e) => ExampleBlock(e.into_owned()),
            ExportBlock(e) => ExportBlock(e.into_owned()),
            SourceBlock(e) => SourceBlock(e.into_owned()),
            BabelCall(e) => BabelCall(e.into_owned()),
            Section => Section,
            Clock(e) => Clock(e.into_onwed()),
            Cookie(e) => Cookie(e.into_owned()),
            RadioTarget => RadioTarget,
            Drawer(e) => Drawer(e.into_owned()),
            Document { pre_blank } => Document { pre_blank },
            DynBlock(e) => DynBlock(e.into_owned()),
            FnDef(e) => FnDef(e.into_owned()),
            FnRef(e) => FnRef(e.into_owned()),
            Headline { level } => Headline { level },
            InlineCall(e) => InlineCall(e.into_owned()),
            InlineSrc(e) => InlineSrc(e.into_owned()),
            Keyword(e) => Keyword(e.into_owned()),
            Link(e) => Link(e.into_owned()),
            List(e) => List(e),
            ListItem(e) => ListItem(e.into_owned()),
            Macros(e) => Macros(e.into_owned()),
            Snippet(e) => Snippet(e.into_owned()),
            Text { value } => Text {
                value: value.into_owned().into(),
            },
            Paragraph { post_blank } => Paragraph { post_blank },
            Rule(e) => Rule(e),
            Timestamp(e) => Timestamp(e.into_owned()),
            Target(e) => Target(e.into_owned()),
            Bold => Bold,
            Strike => Strike,
            Italic => Italic,
            Underline => Underline,
            Verbatim { value } => Verbatim {
                value: value.into_owned().into(),
            },
            Code { value } => Code {
                value: value.into_owned().into(),
            },
            Comment(e) => Comment(e.into_owned()),
            FixedWidth(e) => FixedWidth(e.into_owned()),
            Title(e) => Title(e.into_owned()),
            Table(e) => Table(e.into_owned()),
            TableRow(e) => TableRow(e),
            TableCell(e) => TableCell(e),
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
    Comment,
    CommentBlock,
    Cookie,
    Drawer,
    DynBlock,
    ExampleBlock,
    ExportBlock,
    FixedWidth,
    FnDef,
    FnRef,
    InlineCall,
    InlineSrc,
    Keyword,
    Link,
    ListItem,
    Macros,
    QuoteBlock,
    Snippet,
    SourceBlock,
    SpecialBlock,
    Table,
    Target,
    Timestamp,
    Title,
    VerseBlock;
    List,
    Rule,
    TableRow
);
