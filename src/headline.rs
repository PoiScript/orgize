use indextree::NodeId;
use std::borrow::Cow;
use std::ops::RangeInclusive;
use std::usize;

use crate::{
    config::ParseConfig,
    elements::{Element, Title},
    parsers::{parse_container, Container, OwnedArena},
    validate::{ValidationError, ValidationResult},
    Org,
};

#[derive(Copy, Clone, Debug)]
pub struct Document(Headline);

impl Document {
    pub(crate) fn from_org(org: &Org) -> Document {
        let sec_n = org.arena[org.root]
            .first_child()
            .and_then(|n| match org.arena[n].get() {
                Element::Section => Some(n),
                _ => None,
            });
        // document can be treated as zero-level headline without title
        Document(Headline {
            lvl: 0,
            hdl_n: org.root,
            ttl_n: org.root,
            sec_n,
        })
    }

    pub fn section_node(self) -> Option<NodeId> {
        self.0.sec_n
    }

    pub fn children<'a>(self, org: &'a Org) -> impl Iterator<Item = Headline> + 'a {
        self.0
            .hdl_n
            .children(&org.arena)
            // skip sec_n if exists
            .skip(if self.0.sec_n.is_some() { 1 } else { 0 })
            .map(move |n| match *org.arena[n].get() {
                Element::Headline { level } => Headline::from_node(n, level, org),
                _ => unreachable!(),
            })
    }

    pub fn first_child(self, org: &Org) -> Option<Headline> {
        self.0
            .hdl_n
            .children(&org.arena)
            // skip sec_n if exists
            .nth(if self.0.sec_n.is_some() { 1 } else { 0 })
            .map(move |n| match *org.arena[n].get() {
                Element::Headline { level } => Headline::from_node(n, level, org),
                _ => unreachable!(),
            })
    }

    pub fn last_child(self, org: &Org) -> Option<Headline> {
        self.0.last_child(org)
    }

    pub fn set_section_content<'a, S>(&mut self, content: S, org: &mut Org<'a>)
    where
        S: Into<Cow<'a, str>>,
    {
        let sec_n = if let Some(sec_n) = self.0.sec_n {
            let children: Vec<_> = sec_n.children(&org.arena).collect();
            for child in children {
                child.detach(&mut org.arena);
            }
            sec_n
        } else {
            let sec_n = org.arena.new_node(Element::Section);
            self.0.sec_n = Some(sec_n);
            self.0.hdl_n.prepend(sec_n, &mut org.arena);
            sec_n
        };

        match content.into() {
            Cow::Borrowed(content) => parse_container(
                &mut org.arena,
                Container::Block {
                    node: sec_n,
                    content,
                },
                &ParseConfig::default(),
            ),
            Cow::Owned(ref content) => parse_container(
                &mut OwnedArena::new(&mut org.arena),
                Container::Block {
                    node: sec_n,
                    content,
                },
                &ParseConfig::default(),
            ),
        }

        org.debug_validate();
    }

    pub fn append(self, hdl: Headline, org: &mut Org) -> ValidationResult<()> {
        self.0.append(hdl, org)
    }

    pub fn append_title<'a>(self, ttl: Title<'a>, org: &mut Org<'a>) -> ValidationResult<()> {
        self.0.append(Headline::new(ttl, org), org)
    }

    pub fn prepend(self, hdl: Headline, org: &mut Org) -> ValidationResult<()> {
        hdl.check_detached(org)?;

        if let Some(first) = self.first_child(org) {
            hdl.check_level(first.lvl..=usize::MAX)?;
        } else {
            hdl.check_level(self.0.lvl + 1..=usize::MAX)?;
        }

        if let Some(sec_n) = self.0.sec_n {
            sec_n.insert_after(hdl.hdl_n, &mut org.arena);
        } else {
            self.0.hdl_n.prepend(hdl.hdl_n, &mut org.arena);
        }

        org.debug_validate();

        Ok(())
    }

    pub fn prepend_title<'a>(self, ttl: Title<'a>, org: &mut Org<'a>) -> ValidationResult<()> {
        self.prepend(Headline::new(ttl, org), org)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Headline {
    lvl: usize,
    hdl_n: NodeId,
    ttl_n: NodeId,
    sec_n: Option<NodeId>,
}

impl Headline {
    pub fn new<'a>(ttl: Title<'a>, org: &mut Org<'a>) -> Headline {
        let lvl = ttl.level;
        let hdl_n = org.arena.new_node(Element::Headline { level: ttl.level });
        let ttl_n = org.arena.new_node(Element::Document); // placeholder
        hdl_n.append(ttl_n, &mut org.arena);

        match ttl.raw {
            Cow::Borrowed(content) => parse_container(
                &mut org.arena,
                Container::Inline {
                    node: ttl_n,
                    content,
                },
                &ParseConfig::default(),
            ),
            Cow::Owned(ref content) => parse_container(
                &mut OwnedArena::new(&mut org.arena),
                Container::Inline {
                    node: ttl_n,
                    content,
                },
                &ParseConfig::default(),
            ),
        }

        *org.arena[ttl_n].get_mut() = Element::Title(ttl);

        Headline {
            lvl,
            hdl_n,
            ttl_n,
            sec_n: None,
        }
    }

    pub(crate) fn from_node(hdl_n: NodeId, lvl: usize, org: &Org) -> Headline {
        let ttl_n = org.arena[hdl_n].first_child().unwrap();
        let sec_n = org.arena[ttl_n]
            .next_sibling()
            .and_then(|n| match org.arena[n].get() {
                Element::Section => Some(n),
                _ => None,
            });

        Headline {
            lvl,
            hdl_n,
            ttl_n,
            sec_n,
        }
    }

    pub fn level(self) -> usize {
        self.lvl
    }

    pub fn headline_node(self) -> NodeId {
        self.hdl_n
    }

    pub fn title_node(self) -> NodeId {
        self.ttl_n
    }

    pub fn section_node(self) -> Option<NodeId> {
        self.sec_n
    }

    pub fn title<'a: 'b, 'b>(self, org: &'b Org<'a>) -> &'b Title<'a> {
        match org.arena[self.ttl_n].get() {
            Element::Title(title) => title,
            _ => unreachable!(),
        }
    }

    pub fn title_mut<'a: 'b, 'b>(self, org: &'b mut Org<'a>) -> &'b mut Title<'a> {
        match org.arena[self.ttl_n].get_mut() {
            Element::Title(title) => title,
            _ => unreachable!(),
        }
    }

    pub fn set_title_content<'a, S>(self, content: S, org: &mut Org<'a>)
    where
        S: Into<Cow<'a, str>>,
    {
        let content = content.into();

        let children: Vec<_> = self.ttl_n.children(&org.arena).collect();
        for child in children {
            child.detach(&mut org.arena);
        }

        match &content {
            Cow::Borrowed(content) => parse_container(
                &mut org.arena,
                Container::Inline {
                    node: self.ttl_n,
                    content,
                },
                &ParseConfig::default(),
            ),
            Cow::Owned(ref content) => parse_container(
                &mut OwnedArena::new(&mut org.arena),
                Container::Inline {
                    node: self.ttl_n,
                    content,
                },
                &ParseConfig::default(),
            ),
        }

        self.title_mut(org).raw = content;

        org.debug_validate();
    }

    pub fn set_section_content<'a, S>(&mut self, content: S, org: &mut Org<'a>)
    where
        S: Into<Cow<'a, str>>,
    {
        let sec_n = if let Some(sec_n) = self.sec_n {
            let children: Vec<_> = sec_n.children(&org.arena).collect();
            for child in children {
                child.detach(&mut org.arena);
            }
            sec_n
        } else {
            let sec_n = org.arena.new_node(Element::Section);
            self.sec_n = Some(sec_n);
            self.ttl_n.insert_after(sec_n, &mut org.arena);
            sec_n
        };

        match content.into() {
            Cow::Borrowed(content) => parse_container(
                &mut org.arena,
                Container::Block {
                    node: sec_n,
                    content,
                },
                &ParseConfig::default(),
            ),
            Cow::Owned(ref content) => parse_container(
                &mut OwnedArena::new(&mut org.arena),
                Container::Block {
                    node: sec_n,
                    content,
                },
                &ParseConfig::default(),
            ),
        }

        org.debug_validate();
    }

    pub fn parent(self, org: &Org) -> Option<Headline> {
        org.arena[self.hdl_n]
            .parent()
            .and_then(|n| match *org.arena[n].get() {
                Element::Headline { level } => Some(Headline::from_node(n, level, org)),
                Element::Document => None,
                _ => unreachable!(),
            })
    }

    pub fn children<'a>(self, org: &'a Org) -> impl Iterator<Item = Headline> + 'a {
        self.hdl_n
            .children(&org.arena)
            .skip(if self.sec_n.is_some() { 2 } else { 1 })
            .filter_map(move |n| match *org.arena[n].get() {
                Element::Headline { level } => Some(Headline::from_node(n, level, org)),
                _ => unreachable!(),
            })
    }

    pub fn first_child(self, org: &Org) -> Option<Headline> {
        self.hdl_n
            .children(&org.arena)
            .nth(if self.sec_n.is_some() { 2 } else { 1 })
            .map(|n| match *org.arena[n].get() {
                Element::Headline { level } => Headline::from_node(n, level, org),
                _ => unreachable!(),
            })
    }

    pub fn last_child(self, org: &Org) -> Option<Headline> {
        org.arena[self.hdl_n]
            .last_child()
            .and_then(|n| match *org.arena[n].get() {
                Element::Headline { level } => Some(Headline::from_node(n, level, org)),
                Element::Section | Element::Title(_) => None,
                _ => unreachable!(),
            })
    }

    pub fn previous(self, org: &Org) -> Option<Headline> {
        org.arena[self.hdl_n]
            .previous_sibling()
            .map(|n| match *org.arena[n].get() {
                Element::Headline { level } => Headline::from_node(n, level, org),
                _ => unreachable!(),
            })
    }

    pub fn next(self, org: &Org) -> Option<Headline> {
        org.arena[self.hdl_n]
            .next_sibling()
            .map(|n| match *org.arena[n].get() {
                Element::Headline { level } => Headline::from_node(n, level, org),
                _ => unreachable!(),
            })
    }

    pub fn detach(self, org: &mut Org) {
        self.hdl_n.detach(&mut org.arena);
    }

    pub fn is_detached(self, org: &Org) -> bool {
        self.parent(&org).is_none()
    }

    pub fn append(self, hdl: Headline, org: &mut Org) -> ValidationResult<()> {
        hdl.check_detached(org)?;

        if let Some(last) = self.last_child(org) {
            hdl.check_level(self.lvl + 1..=last.lvl)?;
        } else {
            hdl.check_level(self.lvl + 1..=usize::MAX)?;
        }

        self.hdl_n.append(hdl.hdl_n, &mut org.arena);

        org.debug_validate();

        Ok(())
    }

    pub fn append_title<'a>(self, ttl: Title<'a>, org: &mut Org<'a>) -> ValidationResult<()> {
        self.append(Headline::new(ttl, org), org)
    }

    pub fn prepend(self, hdl: Headline, org: &mut Org) -> ValidationResult<()> {
        hdl.check_detached(org)?;

        if let Some(first) = self.first_child(org) {
            hdl.check_level(first.lvl..=usize::MAX)?;
        } else {
            hdl.check_level(self.lvl + 1..=usize::MAX)?;
        }

        self.sec_n
            .unwrap_or(self.ttl_n)
            .insert_after(hdl.hdl_n, &mut org.arena);

        org.debug_validate();

        Ok(())
    }

    pub fn prepend_title<'a>(self, ttl: Title<'a>, org: &mut Org<'a>) -> ValidationResult<()> {
        self.prepend(Headline::new(ttl, org), org)
    }

    pub fn insert_before(self, hdl: Headline, org: &mut Org) -> ValidationResult<()> {
        hdl.check_detached(org)?;

        if let Some(previous) = self.previous(org) {
            hdl.check_level(self.lvl..=previous.lvl)?;
        } else {
            hdl.check_level(self.lvl..=usize::MAX)?;
        }

        self.hdl_n.insert_before(hdl.hdl_n, &mut org.arena);

        org.debug_validate();

        Ok(())
    }

    pub fn insert_title_before<'a>(
        self,
        ttl: Title<'a>,
        org: &mut Org<'a>,
    ) -> ValidationResult<()> {
        self.insert_before(Headline::new(ttl, org), org)
    }

    pub fn insert_after(self, hdl: Headline, org: &mut Org) -> ValidationResult<()> {
        hdl.check_detached(org)?;

        if let Some(next) = self.next(org) {
            hdl.check_level(next.lvl..=self.lvl)?;
        } else if let Some(parent) = self.parent(org) {
            hdl.check_level(parent.lvl + 1..=self.lvl)?;
        } else {
            hdl.check_level(1..=self.lvl)?;
        }

        self.hdl_n.insert_after(hdl.hdl_n, &mut org.arena);

        org.debug_validate();

        Ok(())
    }

    pub fn insert_title_after<'a>(self, ttl: Title<'a>, org: &mut Org<'a>) -> ValidationResult<()> {
        self.insert_after(Headline::new(ttl, org), org)
    }

    fn check_detached(self, org: &Org) -> ValidationResult<()> {
        if !self.is_detached(org) {
            Err(ValidationError::ExpectedDetached { at: self.hdl_n })
        } else {
            Ok(())
        }
    }

    fn check_level(self, range: RangeInclusive<usize>) -> ValidationResult<()> {
        if !range.contains(&self.lvl) {
            Err(ValidationError::HeadlineLevelMismatch {
                range,
                at: self.hdl_n,
            })
        } else {
            Ok(())
        }
    }
}

impl Org<'_> {
    /// Return a `Document`
    pub fn document(&self) -> Document {
        Document::from_org(self)
    }

    /// Return an iterator of `Headline`
    pub fn headlines(&self) -> impl Iterator<Item = Headline> + '_ {
        self.root
            .descendants(&self.arena)
            .skip(1)
            .filter_map(move |node| match &self.arena[node].get() {
                Element::Headline { level } => Some(Headline::from_node(node, *level, self)),
                _ => None,
            })
    }
}
