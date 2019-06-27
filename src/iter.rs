use indextree::{Arena, NodeEdge, Traverse};

use crate::elements::Element;

#[derive(Debug)]
pub enum Event<'a> {
    Start(&'a Element<'a>),
    End(&'a Element<'a>),
}

pub struct Iter<'a> {
    pub(crate) arena: &'a Arena<Element<'a>>,
    pub(crate) traverse: Traverse<'a, Element<'a>>,
}

impl<'a> Iterator for Iter<'a> {
    type Item = Event<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.traverse.next().map(|edge| match edge {
            NodeEdge::Start(e) => Event::Start(&self.arena[e].data),
            NodeEdge::End(e) => Event::End(&self.arena[e].data),
        })
    }
}
