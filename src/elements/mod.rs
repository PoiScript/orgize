//! Org-mode elements module

mod block;
mod clock;
mod cookie;
mod drawer;
mod dyn_block;
mod fn_def;
mod fn_ref;
mod fragment;
mod headline;
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

pub(crate) mod emphasis;

pub use self::{
    block::Block,
    clock::Clock,
    cookie::Cookie,
    drawer::Drawer,
    dyn_block::DynBlock,
    fn_def::FnDef,
    fn_ref::FnRef,
    headline::{Headline, DEFAULT_TODO_KEYWORDS},
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
    timestamp::*,
};

use indextree::NodeId;

/// Org-mode element enum
///
/// Generally, each variant contains a element struct and
/// a set of properties which indicate the position of the
/// element in the original string.
///
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum Element<'a> {
    Block {
        #[cfg_attr(feature = "serde", serde(flatten))]
        block: Block<'a>,
        #[cfg_attr(all(feature = "serde", not(feature = "extra-serde-info")), serde(skip))]
        begin: usize,
        #[cfg_attr(all(feature = "serde", not(feature = "extra-serde-info")), serde(skip))]
        end: usize,
        #[cfg_attr(all(feature = "serde", not(feature = "extra-serde-info")), serde(skip))]
        contents_begin: usize,
        #[cfg_attr(all(feature = "serde", not(feature = "extra-serde-info")), serde(skip))]
        contents_end: usize,
    },
    BabelCall {
        #[cfg_attr(feature = "serde", serde(flatten))]
        call: BabelCall<'a>,
        #[cfg_attr(all(feature = "serde", not(feature = "extra-serde-info")), serde(skip))]
        begin: usize,
        #[cfg_attr(all(feature = "serde", not(feature = "extra-serde-info")), serde(skip))]
        end: usize,
    },
    Section {
        #[cfg_attr(all(feature = "serde", not(feature = "extra-serde-info")), serde(skip))]
        begin: usize,
        #[cfg_attr(all(feature = "serde", not(feature = "extra-serde-info")), serde(skip))]
        end: usize,
        #[cfg_attr(all(feature = "serde", not(feature = "extra-serde-info")), serde(skip))]
        contents_begin: usize,
        #[cfg_attr(all(feature = "serde", not(feature = "extra-serde-info")), serde(skip))]
        contents_end: usize,
    },
    Clock {
        #[cfg_attr(feature = "serde", serde(flatten))]
        clock: Clock<'a>,
        #[cfg_attr(all(feature = "serde", not(feature = "extra-serde-info")), serde(skip))]
        begin: usize,
        #[cfg_attr(all(feature = "serde", not(feature = "extra-serde-info")), serde(skip))]
        end: usize,
    },
    Cookie {
        #[cfg_attr(feature = "serde", serde(flatten))]
        cookie: Cookie<'a>,
        #[cfg_attr(all(feature = "serde", not(feature = "extra-serde-info")), serde(skip))]
        begin: usize,
        #[cfg_attr(all(feature = "serde", not(feature = "extra-serde-info")), serde(skip))]
        end: usize,
    },
    RadioTarget {
        #[cfg_attr(feature = "serde", serde(flatten))]
        radio_target: RadioTarget<'a>,
        #[cfg_attr(all(feature = "serde", not(feature = "extra-serde-info")), serde(skip))]
        begin: usize,
        #[cfg_attr(all(feature = "serde", not(feature = "extra-serde-info")), serde(skip))]
        end: usize,
    },
    Drawer {
        #[cfg_attr(feature = "serde", serde(flatten))]
        drawer: Drawer<'a>,
        #[cfg_attr(all(feature = "serde", not(feature = "extra-serde-info")), serde(skip))]
        begin: usize,
        #[cfg_attr(all(feature = "serde", not(feature = "extra-serde-info")), serde(skip))]
        end: usize,
        #[cfg_attr(all(feature = "serde", not(feature = "extra-serde-info")), serde(skip))]
        contents_begin: usize,
        #[cfg_attr(all(feature = "serde", not(feature = "extra-serde-info")), serde(skip))]
        contents_end: usize,
    },
    Document {
        #[cfg_attr(all(feature = "serde", not(feature = "extra-serde-info")), serde(skip))]
        begin: usize,
        #[cfg_attr(all(feature = "serde", not(feature = "extra-serde-info")), serde(skip))]
        end: usize,
        #[cfg_attr(all(feature = "serde", not(feature = "extra-serde-info")), serde(skip))]
        contents_begin: usize,
        #[cfg_attr(all(feature = "serde", not(feature = "extra-serde-info")), serde(skip))]
        contents_end: usize,
    },
    DynBlock {
        #[cfg_attr(feature = "serde", serde(flatten))]
        dyn_block: DynBlock<'a>,
        #[cfg_attr(all(feature = "serde", not(feature = "extra-serde-info")), serde(skip))]
        begin: usize,
        #[cfg_attr(all(feature = "serde", not(feature = "extra-serde-info")), serde(skip))]
        end: usize,
        #[cfg_attr(all(feature = "serde", not(feature = "extra-serde-info")), serde(skip))]
        contents_begin: usize,
        #[cfg_attr(all(feature = "serde", not(feature = "extra-serde-info")), serde(skip))]
        contents_end: usize,
    },
    FnDef {
        #[cfg_attr(all(feature = "serde", not(feature = "extra-serde-info")), serde(skip))]
        begin: usize,
        #[cfg_attr(all(feature = "serde", not(feature = "extra-serde-info")), serde(skip))]
        end: usize,
        #[cfg_attr(all(feature = "serde", not(feature = "extra-serde-info")), serde(skip))]
        contents_begin: usize,
        #[cfg_attr(all(feature = "serde", not(feature = "extra-serde-info")), serde(skip))]
        contents_end: usize,
        fn_def: FnDef<'a>,
    },
    FnRef {
        #[cfg_attr(feature = "serde", serde(flatten))]
        fn_ref: FnRef<'a>,
        #[cfg_attr(all(feature = "serde", not(feature = "extra-serde-info")), serde(skip))]
        begin: usize,
        #[cfg_attr(all(feature = "serde", not(feature = "extra-serde-info")), serde(skip))]
        end: usize,
    },
    Headline {
        #[cfg_attr(feature = "serde", serde(flatten))]
        headline: Headline<'a>,
        #[cfg_attr(all(feature = "serde", not(feature = "extra-serde-info")), serde(skip))]
        begin: usize,
        #[cfg_attr(all(feature = "serde", not(feature = "extra-serde-info")), serde(skip))]
        end: usize,
        #[cfg_attr(all(feature = "serde", not(feature = "extra-serde-info")), serde(skip))]
        contents_begin: usize,
        #[cfg_attr(all(feature = "serde", not(feature = "extra-serde-info")), serde(skip))]
        contents_end: usize,
    },
    InlineCall {
        #[cfg_attr(feature = "serde", serde(flatten))]
        inline_call: InlineCall<'a>,
        #[cfg_attr(all(feature = "serde", not(feature = "extra-serde-info")), serde(skip))]
        begin: usize,
        #[cfg_attr(all(feature = "serde", not(feature = "extra-serde-info")), serde(skip))]
        end: usize,
    },
    InlineSrc {
        #[cfg_attr(feature = "serde", serde(flatten))]
        inline_src: InlineSrc<'a>,
        #[cfg_attr(all(feature = "serde", not(feature = "extra-serde-info")), serde(skip))]
        begin: usize,
        #[cfg_attr(all(feature = "serde", not(feature = "extra-serde-info")), serde(skip))]
        end: usize,
    },
    Keyword {
        #[cfg_attr(feature = "serde", serde(flatten))]
        keyword: Keyword<'a>,
        #[cfg_attr(all(feature = "serde", not(feature = "extra-serde-info")), serde(skip))]
        begin: usize,
        #[cfg_attr(all(feature = "serde", not(feature = "extra-serde-info")), serde(skip))]
        end: usize,
    },
    Link {
        #[cfg_attr(feature = "serde", serde(flatten))]
        link: Link<'a>,
        #[cfg_attr(all(feature = "serde", not(feature = "extra-serde-info")), serde(skip))]
        begin: usize,
        #[cfg_attr(all(feature = "serde", not(feature = "extra-serde-info")), serde(skip))]
        end: usize,
    },
    List {
        #[cfg_attr(feature = "serde", serde(flatten))]
        list: List,
        #[cfg_attr(all(feature = "serde", not(feature = "extra-serde-info")), serde(skip))]
        begin: usize,
        #[cfg_attr(all(feature = "serde", not(feature = "extra-serde-info")), serde(skip))]
        end: usize,
        #[cfg_attr(all(feature = "serde", not(feature = "extra-serde-info")), serde(skip))]
        contents_begin: usize,
        #[cfg_attr(all(feature = "serde", not(feature = "extra-serde-info")), serde(skip))]
        contents_end: usize,
    },
    ListItem {
        #[cfg_attr(feature = "serde", serde(flatten))]
        list_item: ListItem<'a>,
        #[cfg_attr(all(feature = "serde", not(feature = "extra-serde-info")), serde(skip))]
        begin: usize,
        #[cfg_attr(all(feature = "serde", not(feature = "extra-serde-info")), serde(skip))]
        end: usize,
        #[cfg_attr(all(feature = "serde", not(feature = "extra-serde-info")), serde(skip))]
        contents_begin: usize,
        #[cfg_attr(all(feature = "serde", not(feature = "extra-serde-info")), serde(skip))]
        contents_end: usize,
    },
    Macros {
        #[cfg_attr(feature = "serde", serde(flatten))]
        macros: Macros<'a>,
        #[cfg_attr(all(feature = "serde", not(feature = "extra-serde-info")), serde(skip))]
        begin: usize,
        #[cfg_attr(all(feature = "serde", not(feature = "extra-serde-info")), serde(skip))]
        end: usize,
    },
    Planning {
        #[cfg_attr(feature = "serde", serde(skip))]
        deadline: Option<NodeId>,
        #[cfg_attr(feature = "serde", serde(skip))]
        scheduled: Option<NodeId>,
        #[cfg_attr(feature = "serde", serde(skip))]
        closed: Option<NodeId>,
        #[cfg_attr(all(feature = "serde", not(feature = "extra-serde-info")), serde(skip))]
        begin: usize,
        #[cfg_attr(all(feature = "serde", not(feature = "extra-serde-info")), serde(skip))]
        end: usize,
    },
    Snippet {
        #[cfg_attr(feature = "serde", serde(flatten))]
        snippet: Snippet<'a>,
        #[cfg_attr(all(feature = "serde", not(feature = "extra-serde-info")), serde(skip))]
        begin: usize,
        #[cfg_attr(all(feature = "serde", not(feature = "extra-serde-info")), serde(skip))]
        end: usize,
    },
    Text {
        value: &'a str,
        #[cfg_attr(all(feature = "serde", not(feature = "extra-serde-info")), serde(skip))]
        begin: usize,
        #[cfg_attr(all(feature = "serde", not(feature = "extra-serde-info")), serde(skip))]
        end: usize,
    },
    Paragraph {
        #[cfg_attr(all(feature = "serde", not(feature = "extra-serde-info")), serde(skip))]
        begin: usize,
        #[cfg_attr(all(feature = "serde", not(feature = "extra-serde-info")), serde(skip))]
        end: usize,
        #[cfg_attr(all(feature = "serde", not(feature = "extra-serde-info")), serde(skip))]
        contents_begin: usize,
        #[cfg_attr(all(feature = "serde", not(feature = "extra-serde-info")), serde(skip))]
        contents_end: usize,
    },
    Rule {
        #[cfg_attr(all(feature = "serde", not(feature = "extra-serde-info")), serde(skip))]
        begin: usize,
        #[cfg_attr(all(feature = "serde", not(feature = "extra-serde-info")), serde(skip))]
        end: usize,
    },
    Timestamp {
        #[cfg_attr(feature = "serde", serde(flatten))]
        timestamp: Timestamp<'a>,
        #[cfg_attr(all(feature = "serde", not(feature = "extra-serde-info")), serde(skip))]
        begin: usize,
        #[cfg_attr(all(feature = "serde", not(feature = "extra-serde-info")), serde(skip))]
        end: usize,
    },
    Target {
        #[cfg_attr(feature = "serde", serde(flatten))]
        target: Target<'a>,
        #[cfg_attr(all(feature = "serde", not(feature = "extra-serde-info")), serde(skip))]
        begin: usize,
        #[cfg_attr(all(feature = "serde", not(feature = "extra-serde-info")), serde(skip))]
        end: usize,
    },
    Bold {
        #[cfg_attr(all(feature = "serde", not(feature = "extra-serde-info")), serde(skip))]
        begin: usize,
        #[cfg_attr(all(feature = "serde", not(feature = "extra-serde-info")), serde(skip))]
        end: usize,
        #[cfg_attr(all(feature = "serde", not(feature = "extra-serde-info")), serde(skip))]
        contents_begin: usize,
        #[cfg_attr(all(feature = "serde", not(feature = "extra-serde-info")), serde(skip))]
        contents_end: usize,
    },
    Strike {
        #[cfg_attr(all(feature = "serde", not(feature = "extra-serde-info")), serde(skip))]
        begin: usize,
        #[cfg_attr(all(feature = "serde", not(feature = "extra-serde-info")), serde(skip))]
        end: usize,
        #[cfg_attr(all(feature = "serde", not(feature = "extra-serde-info")), serde(skip))]
        contents_begin: usize,
        #[cfg_attr(all(feature = "serde", not(feature = "extra-serde-info")), serde(skip))]
        contents_end: usize,
    },
    Italic {
        #[cfg_attr(all(feature = "serde", not(feature = "extra-serde-info")), serde(skip))]
        begin: usize,
        #[cfg_attr(all(feature = "serde", not(feature = "extra-serde-info")), serde(skip))]
        end: usize,
        #[cfg_attr(all(feature = "serde", not(feature = "extra-serde-info")), serde(skip))]
        contents_begin: usize,
        #[cfg_attr(all(feature = "serde", not(feature = "extra-serde-info")), serde(skip))]
        contents_end: usize,
    },
    Underline {
        #[cfg_attr(all(feature = "serde", not(feature = "extra-serde-info")), serde(skip))]
        begin: usize,
        #[cfg_attr(all(feature = "serde", not(feature = "extra-serde-info")), serde(skip))]
        end: usize,
        #[cfg_attr(all(feature = "serde", not(feature = "extra-serde-info")), serde(skip))]
        contents_begin: usize,
        #[cfg_attr(all(feature = "serde", not(feature = "extra-serde-info")), serde(skip))]
        contents_end: usize,
    },
    Verbatim {
        #[cfg_attr(all(feature = "serde", not(feature = "extra-serde-info")), serde(skip))]
        begin: usize,
        #[cfg_attr(all(feature = "serde", not(feature = "extra-serde-info")), serde(skip))]
        end: usize,
        value: &'a str,
    },
    Code {
        #[cfg_attr(all(feature = "serde", not(feature = "extra-serde-info")), serde(skip))]
        begin: usize,
        #[cfg_attr(all(feature = "serde", not(feature = "extra-serde-info")), serde(skip))]
        end: usize,
        value: &'a str,
    },
    Comment {
        #[cfg_attr(all(feature = "serde", not(feature = "extra-serde-info")), serde(skip))]
        begin: usize,
        #[cfg_attr(all(feature = "serde", not(feature = "extra-serde-info")), serde(skip))]
        end: usize,
        value: &'a str,
    },
    FixedWidth {
        #[cfg_attr(all(feature = "serde", not(feature = "extra-serde-info")), serde(skip))]
        begin: usize,
        #[cfg_attr(all(feature = "serde", not(feature = "extra-serde-info")), serde(skip))]
        end: usize,
        value: &'a str,
    },
}
