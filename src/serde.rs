use indextree::{Arena, NodeId};
use serde::ser::{SerializeSeq, SerializeStruct, Serializer};
use serde::Serialize;

use crate::elements::Element;
use crate::org::Org;

impl Serialize for Org<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_newtype_struct(
            "Element",
            &ElementNode {
                node: self.root,
                arena: &self.arena,
            },
        )
    }
}

struct ElementNode<'a> {
    node: NodeId,
    arena: &'a Arena<Element<'a>>,
}

impl Serialize for ElementNode<'_> {
    #[allow(unused_variables)]
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut state;
        match &self.arena[self.node].data {
            Element::Document { begin, end } => {
                state = serializer.serialize_struct("Element::Document", 2)?;
                state.serialize_field("type", "document")?;
                if cfg!(feature = "extra-serde-info") {
                    state.serialize_field("begin", begin)?;
                    state.serialize_field("end", end)?;
                }
            }
            Element::Block {
                block,
                begin,
                end,
                contents_begin,
                contents_end,
            } => {
                state = serializer.serialize_struct("Element::Block", 2)?;
                state.serialize_field("type", "block")?;
                if cfg!(feature = "extra-serde-info") {
                    state.serialize_field("begin", begin)?;
                    state.serialize_field("end", end)?;
                    state.serialize_field("contents_begin", contents_begin)?;
                    state.serialize_field("contents_end", contents_end)?;
                }
            }
            Element::Section {
                begin,
                end,
                contents_begin,
                contents_end,
            } => {
                state = serializer.serialize_struct("Element::Section", 2)?;
                state.serialize_field("type", "section")?;
                if cfg!(feature = "extra-serde-info") {
                    state.serialize_field("begin", begin)?;
                    state.serialize_field("end", end)?;
                    state.serialize_field("contents_begin", contents_begin)?;
                    state.serialize_field("contents_end", contents_end)?;
                }
            }
            Element::Drawer {
                drawer,
                begin,
                end,
                contents_begin,
                contents_end,
            } => {
                state = serializer.serialize_struct("Element::Drawer", 2)?;
                state.serialize_field("type", "drawer")?;
                if cfg!(feature = "extra-serde-info") {
                    state.serialize_field("begin", begin)?;
                    state.serialize_field("end", end)?;
                    state.serialize_field("contents_begin", contents_begin)?;
                    state.serialize_field("contents_end", contents_end)?;
                }
            }
            Element::DynBlock {
                dyn_block,
                begin,
                end,
                contents_begin,
                contents_end,
            } => {
                state = serializer.serialize_struct("Element::DynBlock", 2)?;
                state.serialize_field("type", "dynamic_block")?;
                if cfg!(feature = "extra-serde-info") {
                    state.serialize_field("begin", begin)?;
                    state.serialize_field("end", end)?;
                    state.serialize_field("contents_begin", contents_begin)?;
                    state.serialize_field("contents_end", contents_end)?;
                }
            }
            Element::FnDef {
                begin,
                end,
                contents_begin,
                contents_end,
                fn_def,
            } => {
                state = serializer.serialize_struct("Element::FnDef", 2)?;
                state.serialize_field("type", "footnote_definition")?;
                if cfg!(feature = "extra-serde-info") {
                    state.serialize_field("begin", begin)?;
                    state.serialize_field("end", end)?;
                    state.serialize_field("contents_begin", contents_begin)?;
                    state.serialize_field("contents_end", contents_end)?;
                }
            }
            Element::Headline {
                begin,
                end,
                contents_begin,
                contents_end,
                headline,
            } => {
                state = serializer.serialize_struct("Element::Headline", 2)?;
                state.serialize_field("type", "headline")?;
                state.serialize_field("level", &headline.level)?;
                state.serialize_field("title", &headline.title)?;
                if let Some(prior) = &headline.priority {
                    state.serialize_field("priority", prior)?;
                }
                if let Some(kw) = &headline.keyword {
                    state.serialize_field("keyword", kw)?;
                }
                if !headline.tags.is_empty() {
                    state.serialize_field("tags", &headline.tags)?;
                }
                if cfg!(feature = "extra-serde-info") {
                    state.serialize_field("begin", begin)?;
                    state.serialize_field("end", end)?;
                    state.serialize_field("contents_begin", contents_begin)?;
                    state.serialize_field("contents_end", contents_end)?;
                }
            }
            Element::List {
                list,
                begin,
                end,
                contents_begin,
                contents_end,
            } => {
                state = serializer.serialize_struct("Element::List", 2)?;
                state.serialize_field("type", "list")?;
                if cfg!(feature = "extra-serde-info") {
                    state.serialize_field("begin", begin)?;
                    state.serialize_field("end", end)?;
                    state.serialize_field("contents_begin", contents_begin)?;
                    state.serialize_field("contents_end", contents_end)?;
                }
            }
            Element::ListItem {
                list_item,
                begin,
                end,
                contents_begin,
                contents_end,
            } => {
                state = serializer.serialize_struct("Element::ListItem", 2)?;
                state.serialize_field("type", "list_item")?;
                state.serialize_field("bullet", list_item.bullet)?;
                if cfg!(feature = "extra-serde-info") {
                    state.serialize_field("begin", begin)?;
                    state.serialize_field("end", end)?;
                    state.serialize_field("contents_begin", contents_begin)?;
                    state.serialize_field("contents_end", contents_end)?;
                }
            }
            Element::Paragraph {
                begin,
                end,
                contents_begin,
                contents_end,
            } => {
                state = serializer.serialize_struct("Element::Paragraph", 2)?;
                state.serialize_field("type", "paragraph")?;
                if cfg!(feature = "extra-serde-info") {
                    state.serialize_field("begin", begin)?;
                    state.serialize_field("end", end)?;
                    state.serialize_field("contents_begin", contents_begin)?;
                    state.serialize_field("contents_end", contents_end)?;
                }
            }
            Element::Clock { clock, begin, end } => {
                state = serializer.serialize_struct("Element::Clock", 2)?;
                state.serialize_field("type", "clock")?;
                if cfg!(feature = "extra-serde-info") {
                    state.serialize_field("begin", begin)?;
                    state.serialize_field("end", end)?;
                }
            }
            Element::BabelCall { value, begin, end } => {
                state = serializer.serialize_struct("Element::BabelCall", 2)?;
                state.serialize_field("type", "babel_call")?;
                if cfg!(feature = "extra-serde-info") {
                    state.serialize_field("begin", begin)?;
                    state.serialize_field("end", end)?;
                }
            }
            Element::Cookie { cookie, begin, end } => {
                state = serializer.serialize_struct("Element::Cookie", 2)?;
                state.serialize_field("type", "cookie")?;
                if cfg!(feature = "extra-serde-info") {
                    state.serialize_field("begin", begin)?;
                    state.serialize_field("end", end)?;
                }
            }
            Element::FnRef { fn_ref, begin, end } => {
                state = serializer.serialize_struct("Element::FnRef", 2)?;
                state.serialize_field("type", "footnote_reference")?;
                if cfg!(feature = "extra-serde-info") {
                    state.serialize_field("begin", begin)?;
                    state.serialize_field("end", end)?;
                }
            }
            Element::InlineCall {
                inline_call,
                begin,
                end,
            } => {
                state = serializer.serialize_struct("Element::InlineCall", 2)?;
                state.serialize_field("type", "inline_call")?;
                if cfg!(feature = "extra-serde-info") {
                    state.serialize_field("begin", begin)?;
                    state.serialize_field("end", end)?;
                }
            }
            Element::InlineSrc {
                inline_src,
                begin,
                end,
            } => {
                state = serializer.serialize_struct("Element::InlineSrc", 2)?;
                state.serialize_field("type", "inlne_source_block")?;
                if cfg!(feature = "extra-serde-info") {
                    state.serialize_field("begin", begin)?;
                    state.serialize_field("end", end)?;
                }
            }
            Element::Keyword {
                keyword,
                begin,
                end,
            } => {
                state = serializer.serialize_struct("Element::Keyword", 2)?;
                state.serialize_field("type", "keyword")?;
                state.serialize_field("key", keyword.key)?;
                if let Some(option) = keyword.option {
                    state.serialize_field("option", option)?;
                }
                state.serialize_field("value", keyword.value)?;
                if cfg!(feature = "extra-serde-info") {
                    state.serialize_field("begin", begin)?;
                    state.serialize_field("end", end)?;
                }
            }
            Element::Link { link, begin, end } => {
                state = serializer.serialize_struct("Element::Link", 2)?;
                state.serialize_field("type", "link")?;
                state.serialize_field("path", link.path)?;
                if let Some(desc) = link.desc {
                    state.serialize_field("desc", desc)?;
                }
                if cfg!(feature = "extra-serde-info") {
                    state.serialize_field("begin", begin)?;
                    state.serialize_field("end", end)?;
                }
            }
            Element::Macros { macros, begin, end } => {
                state = serializer.serialize_struct("Element::Macros", 2)?;
                state.serialize_field("type", "macros")?;
                if cfg!(feature = "extra-serde-info") {
                    state.serialize_field("begin", begin)?;
                    state.serialize_field("end", end)?;
                }
            }
            Element::Planning(_) => {
                state = serializer.serialize_struct("Element::Planning", 2)?;
                state.serialize_field("type", "planning")?;
            }
            Element::Snippet {
                begin,
                end,
                snippet,
            } => {
                state = serializer.serialize_struct("Element::Snippet", 2)?;
                state.serialize_field("type", "snippet")?;
                if cfg!(feature = "extra-serde-info") {
                    state.serialize_field("begin", begin)?;
                    state.serialize_field("end", end)?;
                }
            }
            Element::Text { value, begin, end } => {
                state = serializer.serialize_struct("Element::Text", 2)?;
                state.serialize_field("type", "text")?;
                state.serialize_field("value", value)?;
                if cfg!(feature = "extra-serde-info") {
                    state.serialize_field("begin", begin)?;
                    state.serialize_field("end", end)?;
                }
            }
            Element::Rule { begin, end } => {
                state = serializer.serialize_struct("Element::Rule", 2)?;
                state.serialize_field("type", "rule")?;
                if cfg!(feature = "extra-serde-info") {
                    state.serialize_field("begin", begin)?;
                    state.serialize_field("end", end)?;
                }
            }
            Element::Timestamp {
                begin,
                end,
                timestamp,
            } => {
                state = serializer.serialize_struct("Element::Timestamp", 2)?;
                state.serialize_field("type", "timestamp")?;
                if cfg!(feature = "extra-serde-info") {
                    state.serialize_field("begin", begin)?;
                    state.serialize_field("end", end)?;
                }
            }
            Element::Bold {
                begin,
                end,
                contents_begin,
                contents_end,
            } => {
                state = serializer.serialize_struct("Element::Bold", 2)?;
                state.serialize_field("type", "bold")?;
                if cfg!(feature = "extra-serde-info") {
                    state.serialize_field("begin", begin)?;
                    state.serialize_field("end", end)?;
                    state.serialize_field("contents_begin", contents_begin)?;
                    state.serialize_field("contents_end", contents_end)?;
                }
            }
            Element::Strike {
                begin,
                end,
                contents_begin,
                contents_end,
            } => {
                state = serializer.serialize_struct("Element::Strike", 2)?;
                state.serialize_field("type", "strike")?;
                if cfg!(feature = "extra-serde-info") {
                    state.serialize_field("begin", begin)?;
                    state.serialize_field("end", end)?;
                    state.serialize_field("contents_begin", contents_begin)?;
                    state.serialize_field("contents_end", contents_end)?;
                }
            }
            Element::Italic {
                begin,
                end,
                contents_begin,
                contents_end,
            } => {
                state = serializer.serialize_struct("Element::Italic", 2)?;
                state.serialize_field("type", "italic")?;
                if cfg!(feature = "extra-serde-info") {
                    state.serialize_field("begin", begin)?;
                    state.serialize_field("end", end)?;
                    state.serialize_field("contents_begin", contents_begin)?;
                    state.serialize_field("contents_end", contents_end)?;
                }
            }
            Element::Underline {
                begin,
                end,
                contents_begin,
                contents_end,
            } => {
                state = serializer.serialize_struct("Element::Underline", 2)?;
                state.serialize_field("type", "underline")?;
                if cfg!(feature = "extra-serde-info") {
                    state.serialize_field("begin", begin)?;
                    state.serialize_field("end", end)?;
                    state.serialize_field("contents_begin", contents_begin)?;
                    state.serialize_field("contents_end", contents_end)?;
                }
            }
            Element::Code { begin, end, value } => {
                state = serializer.serialize_struct("Element::Code", 2)?;
                state.serialize_field("type", "code")?;
                state.serialize_field("value", value)?;
                if cfg!(feature = "extra-serde-info") {
                    state.serialize_field("begin", begin)?;
                    state.serialize_field("end", end)?;
                }
            }
            Element::Verbatim { begin, end, value } => {
                state = serializer.serialize_struct("Element::Verbatim", 2)?;
                state.serialize_field("type", "verbatim")?;
                state.serialize_field("value", value)?;
                if cfg!(feature = "extra-serde-info") {
                    state.serialize_field("begin", begin)?;
                    state.serialize_field("end", end)?;
                }
            }
            Element::RadioTarget {
                radio_target,
                begin,
                end,
            } => {
                state = serializer.serialize_struct("Element::RadioTarget", 2)?;
                state.serialize_field("type", "radio_target")?;
                if cfg!(feature = "extra-serde-info") {
                    state.serialize_field("begin", begin)?;
                    state.serialize_field("end", end)?;
                }
            }
            Element::Target { target, begin, end } => {
                state = serializer.serialize_struct("Element::Target", 2)?;
                state.serialize_field("type", "target")?;
                if cfg!(feature = "extra-serde-info") {
                    state.serialize_field("begin", begin)?;
                    state.serialize_field("end", end)?;
                }
            }
        }
        if let Some(first) = self.arena[self.node].first_child() {
            state.serialize_field(
                "children",
                &ElementChildrenNode {
                    first,
                    arena: self.arena,
                },
            )?;
        }
        state.end()
    }
}

struct ElementChildrenNode<'a> {
    first: NodeId,
    arena: &'a Arena<Element<'a>>,
}

impl Serialize for ElementChildrenNode<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut seq = serializer.serialize_seq(None)?;
        for node in self.first.following_siblings(&self.arena) {
            seq.serialize_element(&ElementNode {
                node,
                arena: &self.arena,
            })?;
        }
        seq.end()
    }
}
