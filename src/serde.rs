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
                node: self.document,
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
        let node = &self.arena[self.node];

        macro_rules! ser {
            ($state:ident, $begin:ident, $end:ident) => {
                if cfg!(feature = "extra-serde-info") {
                    $state.serialize_field("begin", $begin)?;
                    $state.serialize_field("end", $end)?;
                }
            };
            ($state:ident, $begin:ident, $end:ident, $contents_begin:ident, $contents_end:ident) => {
                if cfg!(feature = "extra-serde-info") {
                    $state.serialize_field("begin", $begin)?;
                    $state.serialize_field("end", $end)?;
                    $state.serialize_field("contents_begin", $contents_begin)?;
                    $state.serialize_field("contents_end", $contents_end)?;
                }
                if let Some(first) = node.first_child() {
                    $state.serialize_field(
                        "children",
                        &ElementChildrenNode {
                            first,
                            arena: self.arena,
                        },
                    )?;
                }
            };
        }

        match &node.data {
            Element::Root => {
                let mut state = serializer.serialize_struct("Element::Root", 2)?;
                state.serialize_field("type", "root")?;
                if let Some(first) = node.first_child() {
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
            Element::Document {
                begin,
                end,
                contents_begin,
                contents_end,
            } => {
                let mut state = serializer.serialize_struct("Element::Document", 2)?;
                state.serialize_field("type", "document")?;
                ser!(state, begin, end, contents_begin, contents_end);
                state.end()
            }
            Element::Block {
                block,
                begin,
                end,
                contents_begin,
                contents_end,
            } => {
                let mut state = serializer.serialize_struct("Element::Block", 2)?;
                state.serialize_field("type", "block")?;
                ser!(state, begin, end, contents_begin, contents_end);
                state.end()
            }
            Element::Section {
                begin,
                end,
                contents_begin,
                contents_end,
            } => {
                let mut state = serializer.serialize_struct("Element::Section", 2)?;
                state.serialize_field("type", "section")?;
                ser!(state, begin, end, contents_begin, contents_end);
                state.end()
            }
            Element::Drawer {
                drawer,
                begin,
                end,
                contents_begin,
                contents_end,
            } => {
                let mut state = serializer.serialize_struct("Element::Drawer", 2)?;
                state.serialize_field("type", "drawer")?;
                ser!(state, begin, end, contents_begin, contents_end);
                state.end()
            }
            Element::DynBlock {
                dyn_block,
                begin,
                end,
                contents_begin,
                contents_end,
            } => {
                let mut state = serializer.serialize_struct("Element::DynBlock", 2)?;
                state.serialize_field("type", "dynamic_block")?;
                ser!(state, begin, end, contents_begin, contents_end);
                state.end()
            }
            Element::FnDef {
                begin,
                end,
                contents_begin,
                contents_end,
                fn_def,
            } => {
                let mut state = serializer.serialize_struct("Element::FnDef", 2)?;
                state.serialize_field("type", "footnote_definition")?;
                ser!(state, begin, end, contents_begin, contents_end);
                state.end()
            }
            Element::Headline {
                begin,
                end,
                contents_begin,
                contents_end,
                headline,
            } => {
                let mut state = serializer.serialize_struct("Element::Headline", 6)?;
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
                ser!(state, begin, end, contents_begin, contents_end);
                state.end()
            }
            Element::List {
                list,
                begin,
                end,
                contents_begin,
                contents_end,
            } => {
                let mut state = serializer.serialize_struct("Element::List", 2)?;
                state.serialize_field("type", "list")?;
                ser!(state, begin, end, contents_begin, contents_end);
                state.end()
            }
            Element::ListItem {
                list_item,
                begin,
                end,
                contents_begin,
                contents_end,
            } => {
                let mut state = serializer.serialize_struct("Element::ListItem", 3)?;
                state.serialize_field("type", "list_item")?;
                state.serialize_field("bullet", list_item.bullet)?;
                ser!(state, begin, end, contents_begin, contents_end);
                state.end()
            }
            Element::Paragraph {
                begin,
                end,
                contents_begin,
                contents_end,
            } => {
                let mut state = serializer.serialize_struct("Element::Paragraph", 2)?;
                state.serialize_field("type", "paragraph")?;
                ser!(state, begin, end, contents_begin, contents_end);
                state.end()
            }
            Element::Clock { clock, begin, end } => {
                let mut state = serializer.serialize_struct("Element::Clock", 1)?;
                state.serialize_field("type", "clock")?;
                ser!(state, begin, end);
                state.end()
            }
            Element::BabelCall { call, begin, end } => {
                let mut state = serializer.serialize_struct("Element::BabelCall", 1)?;
                state.serialize_field("type", "babel_call")?;
                ser!(state, begin, end);
                state.end()
            }
            Element::Cookie { cookie, begin, end } => {
                let mut state = serializer.serialize_struct("Element::Cookie", 1)?;
                state.serialize_field("type", "cookie")?;
                ser!(state, begin, end);
                state.end()
            }
            Element::FnRef { fn_ref, begin, end } => {
                let mut state = serializer.serialize_struct("Element::FnRef", 1)?;
                state.serialize_field("type", "footnote_reference")?;
                ser!(state, begin, end);
                state.end()
            }
            Element::InlineCall {
                inline_call,
                begin,
                end,
            } => {
                let mut state = serializer.serialize_struct("Element::InlineCall", 1)?;
                state.serialize_field("type", "inline_call")?;
                ser!(state, begin, end);
                state.end()
            }
            Element::InlineSrc {
                inline_src,
                begin,
                end,
            } => {
                let mut state = serializer.serialize_struct("Element::InlineSrc", 1)?;
                state.serialize_field("type", "inlne_source_block")?;
                ser!(state, begin, end);
                state.end()
            }
            Element::Keyword {
                keyword,
                begin,
                end,
            } => {
                let mut state = serializer.serialize_struct("Element::Keyword", 4)?;
                state.serialize_field("type", "keyword")?;
                state.serialize_field("key", keyword.key)?;
                if let Some(option) = keyword.option {
                    state.serialize_field("option", option)?;
                }
                state.serialize_field("value", keyword.value)?;
                ser!(state, begin, end);
                state.end()
            }
            Element::Link { link, begin, end } => {
                let mut state = serializer.serialize_struct("Element::Link", 3)?;
                state.serialize_field("type", "link")?;
                state.serialize_field("path", link.path)?;
                if let Some(desc) = link.desc {
                    state.serialize_field("desc", desc)?;
                }
                ser!(state, begin, end);
                state.end()
            }
            Element::Macros { macros, begin, end } => {
                let mut state = serializer.serialize_struct("Element::Macros", 1)?;
                state.serialize_field("type", "macros")?;
                ser!(state, begin, end);
                state.end()
            }
            Element::Planning {
                deadline,
                scheduled,
                closed,
                begin,
                end,
            } => {
                let mut state = serializer.serialize_struct("Element::Planning", 4)?;
                state.serialize_field("type", "planning")?;
                if let Some(node) = deadline {
                    state.serialize_field(
                        "deadline",
                        &ElementNode {
                            node: *node,
                            arena: &self.arena,
                        },
                    )?;
                }
                if let Some(node) = closed {
                    state.serialize_field(
                        "closed",
                        &ElementNode {
                            node: *node,
                            arena: &self.arena,
                        },
                    )?;
                }
                if let Some(node) = scheduled {
                    state.serialize_field(
                        "scheduled",
                        &ElementNode {
                            node: *node,
                            arena: &self.arena,
                        },
                    )?;
                }
                ser!(state, begin, end);
                state.end()
            }
            Element::Snippet {
                begin,
                end,
                snippet,
            } => {
                let mut state = serializer.serialize_struct("Element::Snippet", 2)?;
                state.serialize_field("type", "snippet")?;
                ser!(state, begin, end);
                state.end()
            }
            Element::Text { value, begin, end } => {
                let mut state = serializer.serialize_struct("Element::Text", 2)?;
                state.serialize_field("type", "text")?;
                state.serialize_field("value", value)?;
                ser!(state, begin, end);
                state.end()
            }
            Element::Rule { begin, end } => {
                let mut state = serializer.serialize_struct("Element::Rule", 1)?;
                state.serialize_field("type", "rule")?;
                ser!(state, begin, end);
                state.end()
            }
            Element::Timestamp {
                begin,
                end,
                timestamp,
            } => {
                let mut state = serializer.serialize_struct("Element::Timestamp", 1)?;
                state.serialize_field("type", "timestamp")?;
                ser!(state, begin, end);
                state.end()
            }
            Element::Bold {
                begin,
                end,
                contents_begin,
                contents_end,
            } => {
                let mut state = serializer.serialize_struct("Element::Bold", 2)?;
                state.serialize_field("type", "bold")?;
                ser!(state, begin, end, contents_begin, contents_end);
                state.end()
            }
            Element::Strike {
                begin,
                end,
                contents_begin,
                contents_end,
            } => {
                let mut state = serializer.serialize_struct("Element::Strike", 2)?;
                state.serialize_field("type", "strike")?;
                ser!(state, begin, end, contents_begin, contents_end);
                state.end()
            }
            Element::Italic {
                begin,
                end,
                contents_begin,
                contents_end,
            } => {
                let mut state = serializer.serialize_struct("Element::Italic", 2)?;
                state.serialize_field("type", "italic")?;
                ser!(state, begin, end, contents_begin, contents_end);
                state.end()
            }
            Element::Underline {
                begin,
                end,
                contents_begin,
                contents_end,
            } => {
                let mut state = serializer.serialize_struct("Element::Underline", 2)?;
                state.serialize_field("type", "underline")?;
                ser!(state, begin, end, contents_begin, contents_end);
                state.end()
            }
            Element::Code { begin, end, value } => {
                let mut state = serializer.serialize_struct("Element::Code", 2)?;
                state.serialize_field("type", "code")?;
                state.serialize_field("value", value)?;
                ser!(state, begin, end);
                state.end()
            }
            Element::Verbatim { begin, end, value } => {
                let mut state = serializer.serialize_struct("Element::Verbatim", 2)?;
                state.serialize_field("type", "verbatim")?;
                state.serialize_field("value", value)?;
                ser!(state, begin, end);
                state.end()
            }
            Element::RadioTarget {
                radio_target,
                begin,
                end,
            } => {
                let mut state = serializer.serialize_struct("Element::RadioTarget", 1)?;
                state.serialize_field("type", "radio_target")?;
                ser!(state, begin, end);
                state.end()
            }
            Element::Target { target, begin, end } => {
                let mut state = serializer.serialize_struct("Element::Target", 1)?;
                state.serialize_field("type", "target")?;
                ser!(state, begin, end);
                state.end()
            }
        }
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
