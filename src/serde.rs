use indextree::{Arena, NodeId};
use serde::ser::{SerializeSeq, Serializer};
use serde::Serialize;

use crate::elements::Element;
use crate::org::Org;

impl Serialize for Org<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer
            .serialize_newtype_struct("ElementNode", &ElementNode::new(self.document, &self.arena))
    }
}

#[derive(Serialize)]
struct ElementNode<'a> {
    #[serde(flatten)]
    element: &'a Element<'a>,
    #[serde(skip_serializing_if = "Option::is_none")]
    children: Option<ElementChildrenNode<'a>>,
}

impl<'a> ElementNode<'a> {
    fn new(node_id: NodeId, arena: &'a Arena<Element<'a>>) -> Self {
        let node = &arena[node_id];
        ElementNode {
            element: &node.data,
            children: node
                .first_child()
                .map(|first| ElementChildrenNode { first, arena }),
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
            seq.serialize_element(&ElementNode::new(node, &self.arena))?;
        }
        seq.end()
    }
}
