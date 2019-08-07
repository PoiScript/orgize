use indextree::NodeId;

use crate::elements::{Element, Title};
use crate::Org;

#[derive(Copy, Clone)]
pub struct HeadlineNode(pub(crate) NodeId);

impl HeadlineNode {
    pub fn get_title<'a>(self, org: &'a Org<'a>) -> &'a Title<'a> {
        let title_node = org.arena[self.0].first_child().unwrap();
        if let Element::Title(title) = org.arena[title_node].get() {
            title
        } else {
            unreachable!()
        }
    }

    pub fn get_title_mut<'a>(self, org: &'a mut Org<'a>) -> &'a mut Title<'a> {
        let title_node = org.arena[self.0].first_child().unwrap();
        if let Element::Title(title) = org.arena[title_node].get_mut() {
            title
        } else {
            unreachable!()
        }
    }
}
