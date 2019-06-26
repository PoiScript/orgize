use indextree::{Arena, NodeId};

use crate::elements::*;

#[derive(Debug)]
pub enum Container<'a> {
    Block(&'a Block<'a>),
    Bold,
    Document,
    DynBlock(&'a DynBlock<'a>),
    Headline(&'a Headline<'a>),
    Italic,
    List(&'a List),
    ListItem(&'a ListItem<'a>),
    Paragraph,
    Section,
    Strike,
    Underline,
}

#[derive(Debug)]
pub enum Event<'a> {
    Start(Container<'a>),
    End(Container<'a>),
    Clock(&'a Clock<'a>),
    Cookie(&'a Cookie<'a>),
    Drawer(&'a Drawer<'a>),
    FnDef(&'a FnDef<'a>),
    FnRef(&'a FnRef<'a>),
    InlineCall(&'a InlineCall<'a>),
    InlineSrc(&'a InlineSrc<'a>),
    Keyword(&'a Keyword<'a>),
    Link(&'a Link<'a>),
    Macros(&'a Macros<'a>),
    Planning(&'a Planning<'a>),
    RadioTarget(&'a RadioTarget<'a>),
    Rule,
    Snippet(&'a Snippet<'a>),
    Target(&'a Target<'a>),
    Timestamp(&'a Timestamp<'a>),
    Text(&'a str),
    Code(&'a str),
    Verbatim(&'a str),
    BabelCall(&'a BabelCall<'a>),
}

enum State {
    Start,
    End,
    Empty,
    Finished,
}

pub struct Iter<'a> {
    arena: &'a Arena<Element<'a>>,
    node: NodeId,
    state: State,
}

impl<'a> Iter<'a> {
    pub(crate) fn new(arena: &'a Arena<Element<'a>>, node: NodeId) -> Self {
        Iter {
            arena,
            node,
            state: State::Start,
        }
    }

    fn start_event(&mut self) -> Option<Event<'a>> {
        let node = &self.arena[self.node];
        match &node.data {
            Element::Root => {
                self.state = State::Finished;
                None
            }
            Element::BabelCall { call, .. } => {
                self.state = State::Start;
                Some(Event::BabelCall(call))
            }
            Element::Verbatim { value, .. } => {
                self.state = State::Start;
                Some(Event::Verbatim(value))
            }
            Element::Code { value, .. } => {
                self.state = State::Start;
                Some(Event::Code(value))
            }
            Element::Text { value, .. } => {
                self.state = State::Start;
                Some(Event::Text(value))
            }
            Element::Block { block, .. } => {
                if node.first_child().is_none() {
                    self.state = State::Empty;
                } else {
                    self.state = State::Start;
                }
                Some(Event::Start(Container::Block(block)))
            }
            Element::Bold { .. } => {
                if node.first_child().is_none() {
                    self.state = State::Empty;
                } else {
                    self.state = State::Start;
                }
                Some(Event::Start(Container::Bold))
            }
            Element::Document { .. } => {
                if node.first_child().is_none() {
                    self.state = State::Empty;
                } else {
                    self.state = State::Start;
                }

                Some(Event::Start(Container::Document))
            }
            Element::DynBlock { dyn_block, .. } => {
                if node.first_child().is_none() {
                    self.state = State::Empty;
                } else {
                    self.state = State::Start;
                }
                Some(Event::Start(Container::DynBlock(dyn_block)))
            }
            Element::Headline { headline, .. } => {
                if node.first_child().is_none() {
                    self.state = State::Empty;
                } else {
                    self.state = State::Start;
                }
                Some(Event::Start(Container::Headline(headline)))
            }
            Element::Italic { .. } => {
                if node.first_child().is_none() {
                    self.state = State::Empty;
                } else {
                    self.state = State::Start;
                }
                Some(Event::Start(Container::Italic))
            }
            Element::List { list, .. } => {
                if node.first_child().is_none() {
                    self.state = State::Empty;
                } else {
                    self.state = State::Start;
                }
                Some(Event::Start(Container::List(list)))
            }
            Element::ListItem { list_item, .. } => {
                if node.first_child().is_none() {
                    self.state = State::Empty;
                } else {
                    self.state = State::Start;
                }
                Some(Event::Start(Container::ListItem(list_item)))
            }
            Element::Paragraph { .. } => {
                if node.first_child().is_none() {
                    self.state = State::Empty;
                } else {
                    self.state = State::Start;
                }
                Some(Event::Start(Container::Paragraph))
            }
            Element::Section { .. } => {
                if node.first_child().is_none() {
                    self.state = State::Empty;
                } else {
                    self.state = State::Start;
                }
                Some(Event::Start(Container::Section))
            }
            Element::Strike { .. } => {
                if node.first_child().is_none() {
                    self.state = State::Empty;
                } else {
                    self.state = State::Start;
                }
                Some(Event::Start(Container::Strike))
            }
            Element::Underline { .. } => {
                if node.first_child().is_none() {
                    self.state = State::Empty;
                } else {
                    self.state = State::Start;
                }
                Some(Event::Start(Container::Underline))
            }
            Element::Clock { clock, .. } => {
                self.state = State::Start;
                Some(Event::Clock(clock))
            }
            Element::Cookie { cookie, .. } => {
                self.state = State::Start;
                Some(Event::Cookie(cookie))
            }
            Element::Drawer { drawer, .. } => {
                self.state = State::Start;
                Some(Event::Drawer(drawer))
            }
            Element::FnDef { fn_def, .. } => {
                self.state = State::Start;
                Some(Event::FnDef(fn_def))
            }
            Element::FnRef { fn_ref, .. } => {
                self.state = State::Start;
                Some(Event::FnRef(fn_ref))
            }
            Element::InlineCall { inline_call, .. } => {
                self.state = State::Start;
                Some(Event::InlineCall(inline_call))
            }
            Element::InlineSrc { inline_src, .. } => {
                self.state = State::Start;
                Some(Event::InlineSrc(inline_src))
            }
            Element::Keyword { keyword, .. } => {
                self.state = State::Start;
                Some(Event::Keyword(keyword))
            }
            Element::Link { link, .. } => {
                self.state = State::Start;
                Some(Event::Link(link))
            }
            Element::Macros { macros, .. } => {
                self.state = State::Start;
                Some(Event::Macros(macros))
            }
            Element::Planning(planning) => {
                self.state = State::Start;
                Some(Event::Planning(planning))
            }
            Element::RadioTarget { radio_target, .. } => {
                self.state = State::Start;
                Some(Event::RadioTarget(radio_target))
            }
            Element::Rule { .. } => {
                self.state = State::Start;
                Some(Event::Rule)
            }
            Element::Snippet { snippet, .. } => {
                self.state = State::Start;
                Some(Event::Snippet(snippet))
            }
            Element::Target { target, .. } => {
                self.state = State::Start;
                Some(Event::Target(target))
            }
            Element::Timestamp { timestamp, .. } => {
                self.state = State::Start;
                Some(Event::Timestamp(timestamp))
            }
        }
    }

    fn end_event(&mut self) -> Option<Event<'a>> {
        let node = &self.arena[self.node];
        match &node.data {
            Element::Root => {
                self.state = State::Finished;
                None
            }
            Element::BabelCall { call, .. } => {
                self.state = State::End;
                Some(Event::BabelCall(call))
            }
            Element::Verbatim { value, .. } => {
                self.state = State::End;
                Some(Event::Verbatim(value))
            }
            Element::Code { value, .. } => {
                self.state = State::End;
                Some(Event::Code(value))
            }
            Element::Text { value, .. } => {
                self.state = State::End;
                Some(Event::Text(value))
            }
            Element::Block { block, .. } => {
                self.state = State::End;
                Some(Event::End(Container::Block(block)))
            }
            Element::Bold { .. } => {
                self.state = State::End;
                Some(Event::End(Container::Bold))
            }
            Element::Document { .. } => {
                self.state = State::End;
                Some(Event::End(Container::Document))
            }
            Element::DynBlock { dyn_block, .. } => {
                self.state = State::End;
                Some(Event::End(Container::DynBlock(dyn_block)))
            }
            Element::Headline { headline, .. } => {
                self.state = State::End;
                Some(Event::End(Container::Headline(headline)))
            }
            Element::Italic { .. } => {
                self.state = State::End;
                Some(Event::End(Container::Italic))
            }
            Element::List { list, .. } => {
                self.state = State::End;
                Some(Event::End(Container::List(list)))
            }
            Element::ListItem { list_item, .. } => {
                self.state = State::End;
                Some(Event::End(Container::ListItem(list_item)))
            }
            Element::Paragraph { .. } => {
                self.state = State::End;
                Some(Event::End(Container::Paragraph))
            }
            Element::Section { .. } => {
                self.state = State::End;
                Some(Event::End(Container::Section))
            }
            Element::Strike { .. } => {
                self.state = State::End;
                Some(Event::End(Container::Strike))
            }
            Element::Underline { .. } => {
                self.state = State::End;
                Some(Event::End(Container::Underline))
            }
            Element::Clock { clock, .. } => {
                self.state = State::End;
                Some(Event::Clock(clock))
            }
            Element::Cookie { cookie, .. } => {
                self.state = State::End;
                Some(Event::Cookie(cookie))
            }
            Element::Drawer { drawer, .. } => {
                self.state = State::End;
                Some(Event::Drawer(drawer))
            }
            Element::FnDef { fn_def, .. } => {
                self.state = State::End;
                Some(Event::FnDef(fn_def))
            }
            Element::FnRef { fn_ref, .. } => {
                self.state = State::End;
                Some(Event::FnRef(fn_ref))
            }
            Element::InlineCall { inline_call, .. } => {
                self.state = State::End;
                Some(Event::InlineCall(inline_call))
            }
            Element::InlineSrc { inline_src, .. } => {
                self.state = State::End;
                Some(Event::InlineSrc(inline_src))
            }
            Element::Keyword { keyword, .. } => {
                self.state = State::End;
                Some(Event::Keyword(keyword))
            }
            Element::Link { link, .. } => {
                self.state = State::End;
                Some(Event::Link(link))
            }
            Element::Macros { macros, .. } => {
                self.state = State::End;
                Some(Event::Macros(macros))
            }
            Element::Planning(planning) => {
                self.state = State::End;
                Some(Event::Planning(planning))
            }
            Element::RadioTarget { radio_target, .. } => {
                self.state = State::End;
                Some(Event::RadioTarget(radio_target))
            }
            Element::Rule { .. } => {
                self.state = State::End;
                Some(Event::Rule)
            }
            Element::Snippet { snippet, .. } => {
                self.state = State::End;
                Some(Event::Snippet(snippet))
            }
            Element::Target { target, .. } => {
                self.state = State::End;
                Some(Event::Target(target))
            }
            Element::Timestamp { timestamp, .. } => {
                self.state = State::End;
                Some(Event::Timestamp(timestamp))
            }
        }
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = Event<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.state {
            State::Finished => None,
            State::End => {
                let node = &self.arena[self.node];
                if let Some(sibling_node) = node.next_sibling() {
                    self.node = sibling_node;
                    self.start_event()
                } else if let Some(parent_node) = node.parent() {
                    self.node = parent_node;
                    self.end_event()
                } else {
                    None
                }
            }
            State::Start => {
                let node = &self.arena[self.node];
                if let Some(child_node) = node.first_child() {
                    self.node = child_node;
                    self.start_event()
                } else if let Some(sibling_node) = node.next_sibling() {
                    self.node = sibling_node;
                    self.start_event()
                } else if let Some(parent_node) = node.parent() {
                    self.node = parent_node;
                    self.end_event()
                } else {
                    None
                }
            }
            State::Empty => self.end_event(),
        }
    }
}
