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

/// Represents the document in `Org` struct.
///
/// Each `Org` struct only has one `Document`.
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

    /// Retuen the ID of the section element of this document, or `None` if it has no section.
    pub fn section_node(self) -> Option<NodeId> {
        self.0.sec_n
    }

    /// Returns an iterator of this document's children.
    ///
    /// ```rust
    /// # use orgize::Org;
    /// #
    /// let mut org = Org::parse(
    ///     r#"** h1
    /// ** h2
    /// *** h2_1
    /// *** h2_2
    /// ** h3
    /// "#,
    ///     );
    ///
    /// let d = org.document();
    ///
    /// let mut iter = d.children(&org);
    ///
    /// assert_eq!(iter.next().unwrap().title(&org).raw, "h1");
    /// assert_eq!(iter.next().unwrap().title(&org).raw, "h2");
    /// assert_eq!(iter.next().unwrap().title(&org).raw, "h3");
    /// assert!(iter.next().is_none());
    /// ```
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

    /// Returns the first child of this document, or None if it has no child.
    ///
    /// ```rust
    /// # use orgize::Org;
    /// #
    /// let mut org = Org::parse(
    ///     r#"** h1
    /// ** h2
    /// *** h2_1
    /// *** h2_2
    /// ** h3
    /// "#,
    ///     );
    ///
    /// let d = org.document();
    ///
    /// assert_eq!(d.first_child(&org).unwrap().title(&org).raw, "h1");
    /// ```
    ///
    /// ```rust
    /// let org = Org::new();
    ///
    /// assert!(org.document().first_child(&org).is_none());
    /// ```
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

    /// Returns the last child of this document, or None if it has no child.
    ///
    /// ```rust
    /// # use orgize::Org;
    /// #
    /// let mut org = Org::parse(
    ///     r#"** h1_1
    /// ** h1_2
    /// *** h1_2_1
    /// *** h1_2_2
    /// ** h1_3
    /// "#,
    ///     );
    ///
    /// let d = org.document();
    ///
    /// assert_eq!(d.last_child(&org).unwrap().title(&org).raw, "h1_3");
    /// ```
    ///
    /// ```rust
    /// let org = Org::new();
    ///
    /// assert!(org.document().last_child(&org).is_none());
    /// ```
    pub fn last_child(self, org: &Org) -> Option<Headline> {
        self.0.last_child(org)
    }

    /// Changes the section content of this document.
    ///
    /// ```rust
    /// # use orgize::Org;
    /// #
    /// let mut org = Org::parse(
    ///     r#"** h1_1
    /// ** h1_2
    /// "#,
    /// );
    ///
    /// let mut d = org.document();
    ///
    /// d.set_section_content("s", &mut org);
    ///
    /// let mut writer = Vec::new();
    /// org.org(&mut writer).unwrap();
    /// assert_eq!(
    ///     String::from_utf8(writer).unwrap(),
    ///     r#"s
    /// ** h1_1
    /// ** h1_2
    /// "#,
    /// );
    /// ```
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

    /// Appends a new child to this document.
    ///
    /// Returns an error if the given new child was already attached,
    /// or the given new child didn't meet the requirements of headline levels.
    ///
    /// ```rust
    /// # use orgize::{elements::Title, Headline, Org};
    /// #
    /// let mut org = Org::parse(
    ///     r#"***** h1
    /// **** h2
    /// *** h3
    /// "#,
    /// );
    ///
    /// let d = org.document();
    ///
    /// let mut h4 = Headline::new(
    ///     Title {
    ///         raw: "h4".into(),
    ///         ..Default::default()
    ///     },
    ///     &mut org,
    /// );
    ///
    /// // level must be smaller than or equal to 3
    /// h4.set_level(4, &mut org).unwrap();
    /// assert!(d.append(h4, &mut org).is_err());
    ///
    /// h4.set_level(2, &mut org).unwrap();
    /// assert!(d.append(h4, &mut org).is_ok());
    ///
    /// let mut writer = Vec::new();
    /// org.org(&mut writer).unwrap();
    /// assert_eq!(
    ///     String::from_utf8(writer).unwrap(),
    ///     r#"***** h1
    /// **** h2
    /// *** h3
    /// ** h4
    /// "#,
    /// );
    ///
    /// // cannot append an attached headline
    /// assert!(d.append(h4, &mut org).is_err());
    /// ```
    pub fn append(self, hdl: Headline, org: &mut Org) -> ValidationResult<()> {
        self.0.append(hdl, org)
    }

    /// Prepends a new child to this document.
    ///
    /// Returns an error if the given new child was already attached,
    /// or the given new child didn't meet the requirements of headline levels.
    ///
    /// ```rust
    /// # use orgize::{elements::Title, Headline, Org};
    /// #
    /// let mut org = Org::parse(
    ///     r#"** h2
    /// ** h3
    /// "#,
    /// );
    ///
    /// let d = org.document();
    ///
    /// let mut h1 = Headline::new(
    ///     Title {
    ///         raw: "h1".into(),
    ///         ..Default::default()
    ///     },
    ///     &mut org,
    /// );
    ///
    /// // level must be greater than 2
    /// h1.set_level(1, &mut org).unwrap();
    /// assert!(d.prepend(h1, &mut org).is_err());
    ///
    /// h1.set_level(4, &mut org).unwrap();
    /// assert!(d.prepend(h1, &mut org).is_ok());
    ///
    /// let mut writer = Vec::new();
    /// org.org(&mut writer).unwrap();
    /// assert_eq!(
    ///     String::from_utf8(writer).unwrap(),
    ///     r#"**** h1
    /// ** h2
    /// ** h3
    /// "#,
    /// );
    ///
    /// // cannot prepend an attached headline
    /// assert!(d.prepend(h1, &mut org).is_err());
    /// ```
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
}

/// Represents a headline in `Org` struct.
///
/// Each `Org` has zero or more `Headline`s.
#[derive(Copy, Clone, Debug)]
pub struct Headline {
    lvl: usize,
    hdl_n: NodeId,
    ttl_n: NodeId,
    sec_n: Option<NodeId>,
}

impl Headline {
    /// Creates a new detaced Headline.
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

    /// Returns the level of this headline.
    pub fn level(self) -> usize {
        self.lvl
    }

    /// Retuen the ID of the headline element of this headline.
    pub fn headline_node(self) -> NodeId {
        self.hdl_n
    }

    /// Retuen the ID of the title element of this headline.
    pub fn title_node(self) -> NodeId {
        self.ttl_n
    }

    /// Retuen the ID of the section element of this headline, or `None` if it has no section..
    pub fn section_node(self) -> Option<NodeId> {
        self.sec_n
    }

    /// Returns a reference to the title element of this headline.
    pub fn title<'a: 'b, 'b>(self, org: &'b Org<'a>) -> &'b Title<'a> {
        match org.arena[self.ttl_n].get() {
            Element::Title(title) => title,
            _ => unreachable!(),
        }
    }

    /// Returns a mutual reference to the title element of this headline.
    ///
    /// Don't change the level and content of the `&mut Titile` directly.
    /// Alternatively, uses [`Headline::set_level`] and [`Headline::set_title_content`].
    ///
    /// [`Headline::set_level`]: #method.set_level
    /// [`Headline::set_title_content`]: #method.set_title_content
    ///
    /// ```rust
    /// # use orgize::Org;
    /// #
    /// let mut org = Org::parse("* h1");
    ///
    /// // ..
    /// # let headlines = org.headlines().collect::<Vec<_>>();
    /// # let h1 = headlines[0];
    ///
    /// h1.title_mut(&mut org).priority = Some('A');
    ///
    /// let mut writer = Vec::new();
    /// org.org(&mut writer).unwrap();
    /// assert_eq!(
    ///     String::from_utf8(writer).unwrap(),
    ///     "* [#A] h1\n",
    /// );
    /// ```
    pub fn title_mut<'a: 'b, 'b>(self, org: &'b mut Org<'a>) -> &'b mut Title<'a> {
        match org.arena[self.ttl_n].get_mut() {
            Element::Title(title) => title,
            _ => unreachable!(),
        }
    }

    /// Changes the level of this headline.
    ///
    /// ```rust
    /// # use orgize::{elements::Title, Headline, Org};
    /// #
    /// let mut org = Org::parse(
    ///     r#"* h1
    /// ** h1_1
    /// ** h1_2
    /// ** h1_3
    /// "#,
    ///     );
    ///
    /// // ..
    /// # let headlines = org.headlines().collect::<Vec<_>>();
    /// # let h1 = headlines[0];
    /// # let h1_1 = headlines[1];
    /// # let h1_2 = headlines[2];
    /// # let h1_3 = headlines[3];
    ///
    ///
    /// // detached headline's levels can be changed freely
    /// let mut new_headline = Headline::new(
    ///     Title {
    ///         raw: "new".into(),
    ///         ..Default::default()
    ///     },
    ///     &mut org,
    /// );
    /// new_headline.set_level(42, &mut org).unwrap();
    /// ```
    pub fn set_level(&mut self, lvl: usize, org: &mut Org) -> ValidationResult<()> {
        if !self.is_detached(org) {
            unimplemented!();
        }
        self.lvl = lvl;
        self.title_mut(org).level = lvl;
        if let Element::Headline { level } = org.arena[self.hdl_n].get_mut() {
            *level = lvl;
        }
        Ok(())
    }

    /// Changes the title content of this headline.
    ///
    /// ```rust
    /// # use orgize::Org;
    /// #
    /// let mut org = Org::parse(
    ///     r#"* h1
    /// ** h1_1
    /// "#,
    ///     );
    ///
    /// // ..
    /// # let headlines = org.headlines().collect::<Vec<_>>();
    /// # let h1 = headlines[0];
    /// # let h1_1 = headlines[1];
    ///
    /// h1.set_title_content("H1", &mut org);
    /// h1_1.set_title_content(String::from("*H1_1*"), &mut org);
    ///
    /// assert!(h1.parent(&org).is_none());
    /// let mut writer = Vec::new();
    /// org.org(&mut writer).unwrap();
    /// assert_eq!(
    ///     String::from_utf8(writer).unwrap(),
    ///     r#"* H1
    /// ** *H1_1*
    /// "#,
    /// );
    /// ```
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

    /// Changes the section content of this headline.
    ///
    /// ```rust
    /// # use orgize::Org;
    /// #
    /// let mut org = Org::parse(
    ///     r#"* h1
    /// ** h1_1
    /// s1_1
    /// "#,
    ///     );
    ///
    /// // ..
    /// # let headlines = org.headlines().collect::<Vec<_>>();
    /// # let mut h1 = headlines[0];
    /// # let mut h1_1 = headlines[1];
    ///
    /// h1.set_section_content("s1", &mut org);
    /// h1_1.set_section_content(String::from("*s1_1*"), &mut org);
    ///
    /// assert!(h1.parent(&org).is_none());
    /// let mut writer = Vec::new();
    /// org.org(&mut writer).unwrap();
    /// assert_eq!(
    ///     String::from_utf8(writer).unwrap(),
    ///     r#"* h1
    /// s1
    /// ** h1_1
    /// *s1_1*
    /// "#,
    /// );
    /// ```
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

    /// Returns the parent of this headline, or `None` if it is detached or attached to the document.
    ///
    /// ```rust
    /// # use orgize::{elements::Title, Headline, Org};
    /// #
    /// let mut org = Org::parse(
    ///     r#"* h1
    /// ** h1_1
    /// ** h1_2
    /// *** h1_2_1
    /// *** h1_2_2
    /// ** h1_3
    /// "#,
    ///     );
    ///
    /// // ..
    /// # let headlines = org.headlines().collect::<Vec<_>>();
    /// # let h1 = headlines[0];
    /// # let h1_1 = headlines[1];
    /// # let h1_2_1 = headlines[3];
    ///
    /// assert_eq!(h1_1.parent(&org).unwrap().title(&org).raw, "h1");
    /// assert_eq!(h1_2_1.parent(&org).unwrap().title(&org).raw, "h1_2");
    ///
    /// assert!(h1.parent(&org).is_none());
    ///
    /// // detached headline have no parent
    /// assert!(Headline::new(Title::default(), &mut org).parent(&org).is_none());
    /// ```
    pub fn parent(self, org: &Org) -> Option<Headline> {
        org.arena[self.hdl_n]
            .parent()
            .and_then(|n| match *org.arena[n].get() {
                Element::Headline { level } => Some(Headline::from_node(n, level, org)),
                Element::Document => None,
                _ => unreachable!(),
            })
    }

    /// Returns an iterator of this headline's children.
    ///
    /// ```rust
    /// # use orgize::Org;
    /// #
    /// let mut org = Org::parse(
    ///     r#"* h1
    /// ** h1_1
    /// ** h1_2
    /// *** h1_2_1
    /// *** h1_2_2
    /// ** h1_3
    /// "#,
    ///     );
    ///
    /// // ..
    /// # let headlines = org.headlines().collect::<Vec<_>>();
    /// # let h1 = headlines[0];
    ///
    /// let mut iter = h1.children(&org);
    ///
    /// assert_eq!(iter.next().unwrap().title(&org).raw, "h1_1");
    /// assert_eq!(iter.next().unwrap().title(&org).raw, "h1_2");
    /// assert_eq!(iter.next().unwrap().title(&org).raw, "h1_3");
    /// assert!(iter.next().is_none());
    /// ```
    pub fn children<'a>(self, org: &'a Org) -> impl Iterator<Item = Headline> + 'a {
        self.hdl_n
            .children(&org.arena)
            .skip(if self.sec_n.is_some() { 2 } else { 1 })
            .filter_map(move |n| match *org.arena[n].get() {
                Element::Headline { level } => Some(Headline::from_node(n, level, org)),
                _ => unreachable!(),
            })
    }

    /// Returns the first child of this headline, or `None` if it has no child.
    ///
    /// ```rust
    /// # use orgize::Org;
    /// #
    /// let mut org = Org::parse(
    ///     r#"* h1
    /// ** h1_1
    /// ** h1_2
    /// *** h1_2_1
    /// *** h1_2_2
    /// ** h1_3
    /// "#,
    ///     );
    ///
    /// // ..
    /// # let headlines = org.headlines().collect::<Vec<_>>();
    /// # let h1 = headlines[0];
    /// # let h1_1 = headlines[1];
    /// # let h1_2 = headlines[2];
    /// # let h1_2_1 = headlines[3];
    /// # let h1_2_2 = headlines[4];
    /// # let h1_3 = headlines[5];
    ///
    /// assert_eq!(h1_2.first_child(&org).unwrap().title(&org).raw, "h1_2_1");
    ///
    /// assert!(h1_1.first_child(&org).is_none());
    /// assert!(h1_3.first_child(&org).is_none());
    /// ```
    pub fn first_child(self, org: &Org) -> Option<Headline> {
        self.hdl_n
            .children(&org.arena)
            .nth(if self.sec_n.is_some() { 2 } else { 1 })
            .map(|n| match *org.arena[n].get() {
                Element::Headline { level } => Headline::from_node(n, level, org),
                _ => unreachable!(),
            })
    }

    /// Returns the last child of this headline, or `None` if it has no child.
    ///
    /// ```rust
    /// # use orgize::Org;
    /// #
    /// let mut org = Org::parse(
    ///     r#"* h1
    /// ** h1_1
    /// ** h1_2
    /// *** h1_2_1
    /// *** h1_2_2
    /// ** h1_3
    /// "#,
    ///     );
    ///
    /// // ..
    /// # let headlines = org.headlines().collect::<Vec<_>>();
    /// # let h1 = headlines[0];
    /// # let h1_1 = headlines[1];
    /// # let h1_2 = headlines[2];
    /// # let h1_2_1 = headlines[3];
    /// # let h1_2_2 = headlines[4];
    /// # let h1_3 = headlines[5];
    ///
    /// assert_eq!(h1_2.last_child(&org).unwrap().title(&org).raw, "h1_2_2");
    ///
    /// assert!(h1_1.last_child(&org).is_none());
    /// assert!(h1_3.last_child(&org).is_none());
    /// ```
    pub fn last_child(self, org: &Org) -> Option<Headline> {
        org.arena[self.hdl_n]
            .last_child()
            .and_then(|n| match *org.arena[n].get() {
                Element::Headline { level } => Some(Headline::from_node(n, level, org)),
                Element::Section | Element::Title(_) => None,
                _ => unreachable!(),
            })
    }

    /// Returns the previous sibling of this headline, or `None` if it is a first child.
    ///
    /// ```rust
    /// # use orgize::Org;
    /// #
    /// let mut org = Org::parse(
    ///     r#"* h1
    /// ** h1_1
    /// ** h1_2
    /// *** h1_2_1
    /// *** h1_2_2
    /// ** h1_3
    /// "#,
    ///     );
    ///
    /// // ..
    /// # let headlines = org.headlines().collect::<Vec<_>>();
    /// # let h1 = headlines[0];
    /// # let h1_1 = headlines[1];
    /// # let h1_2 = headlines[2];
    /// # let h1_2_1 = headlines[3];
    /// # let h1_2_2 = headlines[4];
    /// # let h1_3 = headlines[5];
    ///
    /// assert_eq!(h1_2.previous(&org).unwrap().title(&org).raw, "h1_1");
    ///
    /// assert!(h1_1.previous(&org).is_none());
    /// assert!(h1_2_1.previous(&org).is_none());
    /// ```
    pub fn previous(self, org: &Org) -> Option<Headline> {
        org.arena[self.hdl_n]
            .previous_sibling()
            .and_then(|n| match *org.arena[n].get() {
                Element::Headline { level } => Some(Headline::from_node(n, level, org)),
                Element::Title(_) | Element::Section => None,
                _ => unreachable!(),
            })
    }

    /// Returns the next sibling of this headline, or `None` if it is a last child.
    ///
    /// ```rust
    /// # use orgize::Org;
    /// #
    /// let mut org = Org::parse(
    ///     r#"* h1
    /// ** h1_1
    /// ** h1_2
    /// *** h1_2_1
    /// *** h1_2_2
    /// ** h1_3
    /// "#,
    ///     );
    ///
    /// // ..
    /// # let headlines = org.headlines().collect::<Vec<_>>();
    /// # let h1 = headlines[0];
    /// # let h1_1 = headlines[1];
    /// # let h1_2 = headlines[2];
    /// # let h1_2_1 = headlines[3];
    /// # let h1_2_2 = headlines[4];
    /// # let h1_3 = headlines[5];
    ///
    /// assert_eq!(h1_2.next(&org).unwrap().title(&org).raw, "h1_3");
    ///
    /// assert!(h1_3.next(&org).is_none());
    /// assert!(h1_2_2.next(&org).is_none());
    /// ```
    pub fn next(self, org: &Org) -> Option<Headline> {
        org.arena[self.hdl_n]
            .next_sibling()
            .map(|n| match *org.arena[n].get() {
                Element::Headline { level } => Headline::from_node(n, level, org),
                _ => unreachable!(),
            })
    }

    /// Detaches this headline from arena.
    ///
    /// ```rust
    /// # use orgize::Org;
    /// #
    /// let mut org = Org::parse(
    ///     r#"* h1
    /// ** h1_1
    /// ** h1_2
    /// *** h1_2_1
    /// *** h1_2_2
    /// ** h1_3
    /// "#,
    ///     );
    ///
    /// // ..
    /// # let headlines = org.headlines().collect::<Vec<_>>();
    /// # let h1 = headlines[0];
    /// # let h1_1 = headlines[1];
    /// # let h1_2 = headlines[2];
    ///
    /// h1_2.detach(&mut org);
    ///
    /// let mut writer = Vec::new();
    /// org.org(&mut writer).unwrap();
    /// assert_eq!(
    ///     String::from_utf8(writer).unwrap(),
    ///     r#"* h1
    /// ** h1_1
    /// ** h1_3
    /// "#,
    /// );
    /// ```
    pub fn detach(self, org: &mut Org) {
        self.hdl_n.detach(&mut org.arena);
    }

    /// Returns `true` if this headline is detached.
    pub fn is_detached(self, org: &Org) -> bool {
        org.arena[self.hdl_n].parent().is_none()
    }

    /// Appends a new child to this headline.
    ///
    /// Returns an error if the given new child was already attached, or
    /// the given new child didn't meet the requirements of headline levels.
    ///
    /// ```rust
    /// # use orgize::{elements::Title, Headline, Org};
    /// #
    /// let mut org = Org::parse(
    ///     r#"* h1
    /// ** h1_1
    /// ***** h1_1_1
    /// "#,
    ///     );
    ///
    /// // ..
    /// # let headlines = org.headlines().collect::<Vec<_>>();
    /// # let h1 = headlines[0];
    /// # let h1_1 = headlines[1];
    ///
    /// let mut h1_1_2 = Headline::new(
    ///     Title {
    ///         raw: "h1_1_2".into(),
    ///         ..Default::default()
    ///     },
    ///     &mut org,
    /// );
    ///
    /// // level must be greater than 2, and smaller than or equal to 5
    /// h1_1_2.set_level(2, &mut org).unwrap();
    /// assert!(h1_1.append(h1_1_2, &mut org).is_err());
    /// h1_1_2.set_level(6, &mut org).unwrap();
    /// assert!(h1_1.append(h1_1_2, &mut org).is_err());
    ///
    /// h1_1_2.set_level(4, &mut org).unwrap();
    /// assert!(h1_1.append(h1_1_2, &mut org).is_ok());
    ///
    /// let mut writer = Vec::new();
    /// org.org(&mut writer).unwrap();
    /// assert_eq!(
    ///     String::from_utf8(writer).unwrap(),
    ///     r#"* h1
    /// ** h1_1
    /// ***** h1_1_1
    /// **** h1_1_2
    /// "#,
    /// );
    ///
    /// // cannot append an attached headline
    /// assert!(h1_1.append(h1_1_2, &mut org).is_err());
    /// ```
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

    /// Prepends a new child to this headline.
    ///
    /// Returns an error if the given new child was already attached, or
    /// the given new child didn't meet the requirements of headline levels.
    ///
    /// ```rust
    /// # use orgize::{elements::Title, Headline, Org};
    /// #
    /// let mut org = Org::parse(
    ///     r#"* h1
    /// ** h1_1
    /// ***** h1_1_1
    /// "#,
    ///     );
    ///
    /// // ..
    /// # let headlines = org.headlines().collect::<Vec<_>>();
    /// # let h1 = headlines[0];
    /// # let h1_1 = headlines[1];
    ///
    /// let mut h1_1_2 = Headline::new(
    ///     Title {
    ///         raw: "h1_1_2".into(),
    ///         ..Default::default()
    ///     },
    ///     &mut org,
    /// );
    ///
    /// // level must be greater than or equal to 5
    /// h1_1_2.set_level(2, &mut org).unwrap();
    /// assert!(h1_1.prepend(h1_1_2, &mut org).is_err());
    ///
    /// h1_1_2.set_level(5, &mut org).unwrap();
    /// assert!(h1_1.prepend(h1_1_2, &mut org).is_ok());
    ///
    /// // cannot prepend an attached headline
    /// assert!(h1_1.prepend(h1_1_2, &mut org).is_err());
    /// ```
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

    /// Inserts a new sibling before this headline.
    ///
    /// Returns an error if the given new child was already attached, or
    /// the given new child didn't meet the requirements of headline levels.
    ///
    /// ```rust
    /// # use orgize::{elements::Title, Headline, Org};
    /// #
    /// let mut org = Org::parse(
    ///     r#"* h1
    /// ** h1_1
    /// **** h1_1_1
    /// *** h1_1_3
    /// "#,
    ///     );
    ///
    /// // ..
    /// # let headlines = org.headlines().collect::<Vec<_>>();
    /// # let h1 = headlines[0];
    /// # let h1_1 = headlines[1];
    /// # let h1_1_1 = headlines[2];
    /// # let h1_1_3 = headlines[3];
    ///
    /// let mut h1_1_2 = Headline::new(
    ///     Title {
    ///         raw: "h1_1_2".into(),
    ///         ..Default::default()
    ///     },
    ///     &mut org,
    /// );
    ///
    /// // level must be greater than or equal to 3, but smaller than or equal to 4
    /// h1_1_2.set_level(2, &mut org).unwrap();
    /// assert!(h1_1_3.insert_before(h1_1_2, &mut org).is_err());
    /// h1_1_2.set_level(5, &mut org).unwrap();
    /// assert!(h1_1_3.insert_before(h1_1_2, &mut org).is_err());
    ///
    /// h1_1_2.set_level(4, &mut org).unwrap();
    /// assert!(h1_1_3.insert_before(h1_1_2, &mut org).is_ok());
    ///
    /// let mut writer = Vec::new();
    /// org.org(&mut writer).unwrap();
    /// assert_eq!(
    ///     String::from_utf8(writer).unwrap(),
    ///     r#"* h1
    /// ** h1_1
    /// **** h1_1_1
    /// **** h1_1_2
    /// *** h1_1_3
    /// "#,
    /// );
    ///
    /// // cannot insert an attached headline
    /// assert!(h1_1_3.insert_before(h1_1_2, &mut org).is_err());
    /// ```
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

    /// Inserts a new sibling after this headline.
    ///
    /// Returns an error if the given new child was already attached, or
    /// the given new child didn't meet the requirements of headline levels.
    ///
    /// ```rust
    /// # use orgize::{elements::Title, Headline, Org};
    /// #
    /// let mut org = Org::parse(
    ///     r#"* h1
    /// ** h1_1
    /// **** h1_1_1
    /// *** h1_1_3
    /// "#,
    ///     );
    ///
    /// // ..
    /// # let headlines = org.headlines().collect::<Vec<_>>();
    /// # let h1 = headlines[0];
    /// # let h1_1 = headlines[1];
    /// # let h1_1_1 = headlines[2];
    /// # let h1_1_3 = headlines[3];
    ///
    /// let mut h1_1_2 = Headline::new(
    ///     Title {
    ///         raw: "h1_1_2".into(),
    ///         ..Default::default()
    ///     },
    ///     &mut org,
    /// );
    ///
    /// // level must be greater than or equal to 3, but smaller than or equal to 4
    /// h1_1_2.set_level(2, &mut org).unwrap();
    /// assert!(h1_1_1.insert_after(h1_1_2, &mut org).is_err());
    /// h1_1_2.set_level(5, &mut org).unwrap();
    /// assert!(h1_1_1.insert_after(h1_1_2, &mut org).is_err());
    ///
    /// h1_1_2.set_level(4, &mut org).unwrap();
    /// assert!(h1_1_1.insert_after(h1_1_2, &mut org).is_ok());
    ///
    /// let mut writer = Vec::new();
    /// org.org(&mut writer).unwrap();
    /// assert_eq!(
    ///     String::from_utf8(writer).unwrap(),
    ///     r#"* h1
    /// ** h1_1
    /// **** h1_1_1
    /// **** h1_1_2
    /// *** h1_1_3
    /// "#,
    /// );
    ///
    /// // cannot insert an attached headline
    /// assert!(h1_1_1.insert_after(h1_1_2, &mut org).is_err());
    /// ```
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
    /// Returns the `Document`.
    pub fn document(&self) -> Document {
        Document::from_org(self)
    }

    /// Returns an iterator of `Headline`s.
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
