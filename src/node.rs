use indextree::NodeId;

use crate::elements::{Element, Title};
use crate::Org;

#[derive(Copy, Clone)]
pub struct HeadlineNode(pub(crate) NodeId);

impl HeadlineNode {
    pub fn get_title<'a: 'b, 'b>(self, org: &'b Org<'a>) -> &'b Title<'a> {
        let title_node = org.arena[self.0].first_child().unwrap();
        if let Element::Title(title) = org.arena[title_node].get() {
            title
        } else {
            unreachable!()
        }
    }

    pub fn get_title_mut<'a: 'b, 'b>(self, org: &'b mut Org<'a>) -> &'b mut Title<'a> {
        let title_node = org.arena[self.0].first_child().unwrap();
        if let Element::Title(title) = org.arena[title_node].get_mut() {
            title
        } else {
            unreachable!()
        }
    }
}
