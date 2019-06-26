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

pub mod emphasis;

pub use self::{
    block::Block,
    clock::Clock,
    cookie::Cookie,
    drawer::Drawer,
    dyn_block::DynBlock,
    fn_def::FnDef,
    fn_ref::FnRef,
    headline::Headline,
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

#[derive(Debug)]
pub enum Element<'a> {
    Root,
    Block {
        block: Block<'a>,
        begin: usize,
        end: usize,
        contents_begin: usize,
        contents_end: usize,
    },
    BabelCall {
        call: BabelCall<'a>,
        begin: usize,
        end: usize,
    },
    Section {
        begin: usize,
        end: usize,
        contents_begin: usize,
        contents_end: usize,
    },
    Clock {
        clock: Clock<'a>,
        begin: usize,
        end: usize,
    },
    Cookie {
        cookie: Cookie<'a>,
        begin: usize,
        end: usize,
    },
    RadioTarget {
        radio_target: RadioTarget<'a>,
        begin: usize,
        end: usize,
    },
    Drawer {
        drawer: Drawer<'a>,
        begin: usize,
        end: usize,
        contents_begin: usize,
        contents_end: usize,
    },
    Document {
        begin: usize,
        end: usize,
    },
    DynBlock {
        dyn_block: DynBlock<'a>,
        begin: usize,
        end: usize,
        contents_begin: usize,
        contents_end: usize,
    },
    FnDef {
        begin: usize,
        end: usize,
        contents_begin: usize,
        contents_end: usize,
        fn_def: FnDef<'a>,
    },
    FnRef {
        fn_ref: FnRef<'a>,
        begin: usize,
        end: usize,
    },
    Headline {
        begin: usize,
        end: usize,
        contents_begin: usize,
        contents_end: usize,
        headline: Headline<'a>,
    },
    InlineCall {
        inline_call: InlineCall<'a>,
        begin: usize,
        end: usize,
    },
    InlineSrc {
        inline_src: InlineSrc<'a>,
        begin: usize,
        end: usize,
    },
    Keyword {
        keyword: Keyword<'a>,
        begin: usize,
        end: usize,
    },
    Link {
        link: Link<'a>,
        begin: usize,
        end: usize,
    },
    List {
        list: List,
        begin: usize,
        end: usize,
        contents_begin: usize,
        contents_end: usize,
    },
    ListItem {
        list_item: ListItem<'a>,
        begin: usize,
        end: usize,
        contents_begin: usize,
        contents_end: usize,
    },
    Macros {
        macros: Macros<'a>,
        begin: usize,
        end: usize,
    },
    Planning(Planning<'a>),
    Snippet {
        begin: usize,
        end: usize,
        snippet: Snippet<'a>,
    },
    Text {
        value: &'a str,
        begin: usize,
        end: usize,
    },
    Paragraph {
        begin: usize,
        end: usize,
        contents_begin: usize,
        contents_end: usize,
    },
    Rule {
        begin: usize,
        end: usize,
    },
    Timestamp {
        begin: usize,
        end: usize,
        timestamp: Timestamp<'a>,
    },
    Target {
        target: Target<'a>,
        begin: usize,
        end: usize,
    },
    Bold {
        begin: usize,
        end: usize,
        contents_begin: usize,
        contents_end: usize,
    },
    Strike {
        begin: usize,
        end: usize,
        contents_begin: usize,
        contents_end: usize,
    },
    Italic {
        begin: usize,
        end: usize,
        contents_begin: usize,
        contents_end: usize,
    },
    Underline {
        begin: usize,
        end: usize,
        contents_begin: usize,
        contents_end: usize,
    },
    Verbatim {
        begin: usize,
        end: usize,
        value: &'a str,
    },
    Code {
        begin: usize,
        end: usize,
        value: &'a str,
    },
}
