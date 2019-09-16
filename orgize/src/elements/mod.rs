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
mod table;
mod target;
mod timestamp;
mod title;

pub(crate) use self::{
    block::parse_block_element, emphasis::parse_emphasis, keyword::parse_keyword,
    radio_target::parse_radio_target, rule::parse_rule, table::parse_table_el,
};

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
    snippet::Snippet,
    table::{Table, TableRow},
    target::Target,
    timestamp::{Datetime, Timestamp},
    title::Title,
};

use std::borrow::Cow;

/// Orgize Element Enum
#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
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
    Document,
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
    Macros(Macros<'a>),
    Snippet(Snippet<'a>),
    Text { value: Cow<'a, str> },
    Paragraph,
    Rule,
    Timestamp(Timestamp<'a>),
    Target(Target<'a>),
    Bold,
    Strike,
    Italic,
    Underline,
    Verbatim { value: Cow<'a, str> },
    Code { value: Cow<'a, str> },
    Comment { value: Cow<'a, str> },
    FixedWidth { value: Cow<'a, str> },
    Title(Title<'a>),
    Table(Table<'a>),
    TableRow(TableRow),
    TableCell,
}

impl Element<'_> {
    pub fn is_container(&self) -> bool {
        use Element::*;

        match self {
            SpecialBlock(_)
            | QuoteBlock(_)
            | CenterBlock(_)
            | VerseBlock(_)
            | Bold
            | Document
            | DynBlock(_)
            | Headline { .. }
            | Italic
            | List(_)
            | ListItem(_)
            | Paragraph
            | Section
            | Strike
            | Underline
            | Title(_)
            | Table(_)
            | TableRow(_)
            | TableCell => true,
            _ => false,
        }
    }

    pub fn into_owned(self) -> Element<'static> {
        use Element::*;

        match self {
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
            Document => Document,
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
            Paragraph => Paragraph,
            Rule => Rule,
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
            Comment { value } => Comment {
                value: value.into_owned().into(),
            },
            FixedWidth { value } => FixedWidth {
                value: value.into_owned().into(),
            },
            Title(e) => Title(e.into_owned()),
            Table(e) => Table(e.into_owned()),
            TableRow(e) => TableRow(e),
            TableCell => TableCell,
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
    QuoteBlock,
    Snippet,
    SourceBlock,
    SpecialBlock,
    Target,
    Timestamp,
    Table,
    Title,
    VerseBlock;
    List,
    TableRow
);
