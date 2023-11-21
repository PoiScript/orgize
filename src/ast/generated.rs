//! generated file, do not modify it directly
#![allow(clippy::all)]
#![allow(unused)]

use crate::syntax::{OrgLanguage, SyntaxKind, SyntaxKind::*, SyntaxNode, SyntaxToken};
use rowan::ast::{support, AstChildren, AstNode};

fn affiliated_keyword(
    node: &SyntaxNode,
    filter: impl Fn(&str) -> bool,
) -> Option<AffiliatedKeyword> {
    node.children()
        .take_while(|n| n.kind() == SyntaxKind::AFFILIATED_KEYWORD)
        .filter_map(AffiliatedKeyword::cast)
        .find(|k| filter(&k.key()))
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Document {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for Document {
    type Language = OrgLanguage;
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == DOCUMENT
    }
    fn cast(node: SyntaxNode) -> Option<Document> {
        Self::can_cast(node.kind()).then(|| Document { syntax: node })
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl Document {
    pub fn begin(&self) -> u32 {
        self.syntax.text_range().start().into()
    }
    pub fn end(&self) -> u32 {
        self.syntax.text_range().end().into()
    }
    pub fn section(&self) -> Option<Section> {
        support::child(&self.syntax)
    }
    pub fn first_headline(&self) -> Option<Headline> {
        support::child(&self.syntax)
    }
    pub fn last_headline(&self) -> Option<Headline> {
        super::last_child(&self.syntax)
    }
    pub fn headlines(&self) -> AstChildren<Headline> {
        support::children(&self.syntax)
    }
    pub fn pre_blank(&self) -> usize {
        super::blank_lines(&self.syntax)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Section {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for Section {
    type Language = OrgLanguage;
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SECTION
    }
    fn cast(node: SyntaxNode) -> Option<Section> {
        Self::can_cast(node.kind()).then(|| Section { syntax: node })
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl Section {
    pub fn begin(&self) -> u32 {
        self.syntax.text_range().start().into()
    }
    pub fn end(&self) -> u32 {
        self.syntax.text_range().end().into()
    }
    pub fn post_blank(&self) -> usize {
        super::blank_lines(&self.syntax)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Paragraph {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for Paragraph {
    type Language = OrgLanguage;
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == PARAGRAPH
    }
    fn cast(node: SyntaxNode) -> Option<Paragraph> {
        Self::can_cast(node.kind()).then(|| Paragraph { syntax: node })
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl Paragraph {
    pub fn begin(&self) -> u32 {
        self.syntax.text_range().start().into()
    }
    pub fn end(&self) -> u32 {
        self.syntax.text_range().end().into()
    }
    pub fn post_blank(&self) -> usize {
        super::blank_lines(&self.syntax)
    }
    pub fn caption(&self) -> Option<AffiliatedKeyword> {
        affiliated_keyword(&self.syntax, |k| k == "CAPTION")
    }
    pub fn header(&self) -> Option<AffiliatedKeyword> {
        affiliated_keyword(&self.syntax, |k| k == "HEADER")
    }
    pub fn name(&self) -> Option<AffiliatedKeyword> {
        affiliated_keyword(&self.syntax, |k| k == "NAME")
    }
    pub fn plot(&self) -> Option<AffiliatedKeyword> {
        affiliated_keyword(&self.syntax, |k| k == "PLOT")
    }
    pub fn results(&self) -> Option<AffiliatedKeyword> {
        affiliated_keyword(&self.syntax, |k| k == "RESULTS")
    }
    pub fn attr(&self, backend: &str) -> Option<AffiliatedKeyword> {
        affiliated_keyword(&self.syntax, |k| {
            k.starts_with("ATTR_") && &k[5..] == backend
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Headline {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for Headline {
    type Language = OrgLanguage;
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == HEADLINE
    }
    fn cast(node: SyntaxNode) -> Option<Headline> {
        Self::can_cast(node.kind()).then(|| Headline { syntax: node })
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl Headline {
    pub fn begin(&self) -> u32 {
        self.syntax.text_range().start().into()
    }
    pub fn end(&self) -> u32 {
        self.syntax.text_range().end().into()
    }
    pub fn section(&self) -> Option<Section> {
        support::child(&self.syntax)
    }
    pub fn planning(&self) -> Option<Planning> {
        support::child(&self.syntax)
    }
    pub fn headlines(&self) -> AstChildren<Headline> {
        support::children(&self.syntax)
    }
    pub fn post_blank(&self) -> usize {
        super::blank_lines(&self.syntax)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HeadlineTitle {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for HeadlineTitle {
    type Language = OrgLanguage;
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == HEADLINE_TITLE
    }
    fn cast(node: SyntaxNode) -> Option<HeadlineTitle> {
        Self::can_cast(node.kind()).then(|| HeadlineTitle { syntax: node })
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl HeadlineTitle {
    pub fn begin(&self) -> u32 {
        self.syntax.text_range().start().into()
    }
    pub fn end(&self) -> u32 {
        self.syntax.text_range().end().into()
    }
    pub fn headline(&self) -> Option<Headline> {
        self.syntax.parent().and_then(Headline::cast)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PropertyDrawer {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for PropertyDrawer {
    type Language = OrgLanguage;
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == PROPERTY_DRAWER
    }
    fn cast(node: SyntaxNode) -> Option<PropertyDrawer> {
        Self::can_cast(node.kind()).then(|| PropertyDrawer { syntax: node })
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl PropertyDrawer {
    pub fn begin(&self) -> u32 {
        self.syntax.text_range().start().into()
    }
    pub fn end(&self) -> u32 {
        self.syntax.text_range().end().into()
    }
    pub fn node_properties(&self) -> AstChildren<NodeProperty> {
        support::children(&self.syntax)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NodeProperty {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for NodeProperty {
    type Language = OrgLanguage;
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == NODE_PROPERTY
    }
    fn cast(node: SyntaxNode) -> Option<NodeProperty> {
        Self::can_cast(node.kind()).then(|| NodeProperty { syntax: node })
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl NodeProperty {
    pub fn begin(&self) -> u32 {
        self.syntax.text_range().start().into()
    }
    pub fn end(&self) -> u32 {
        self.syntax.text_range().end().into()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Planning {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for Planning {
    type Language = OrgLanguage;
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == PLANNING
    }
    fn cast(node: SyntaxNode) -> Option<Planning> {
        Self::can_cast(node.kind()).then(|| Planning { syntax: node })
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl Planning {
    pub fn begin(&self) -> u32 {
        self.syntax.text_range().start().into()
    }
    pub fn end(&self) -> u32 {
        self.syntax.text_range().end().into()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OrgTable {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for OrgTable {
    type Language = OrgLanguage;
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == ORG_TABLE
    }
    fn cast(node: SyntaxNode) -> Option<OrgTable> {
        Self::can_cast(node.kind()).then(|| OrgTable { syntax: node })
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl OrgTable {
    pub fn begin(&self) -> u32 {
        self.syntax.text_range().start().into()
    }
    pub fn end(&self) -> u32 {
        self.syntax.text_range().end().into()
    }
    pub fn post_blank(&self) -> usize {
        super::blank_lines(&self.syntax)
    }
    pub fn caption(&self) -> Option<AffiliatedKeyword> {
        affiliated_keyword(&self.syntax, |k| k == "CAPTION")
    }
    pub fn header(&self) -> Option<AffiliatedKeyword> {
        affiliated_keyword(&self.syntax, |k| k == "HEADER")
    }
    pub fn name(&self) -> Option<AffiliatedKeyword> {
        affiliated_keyword(&self.syntax, |k| k == "NAME")
    }
    pub fn plot(&self) -> Option<AffiliatedKeyword> {
        affiliated_keyword(&self.syntax, |k| k == "PLOT")
    }
    pub fn results(&self) -> Option<AffiliatedKeyword> {
        affiliated_keyword(&self.syntax, |k| k == "RESULTS")
    }
    pub fn attr(&self, backend: &str) -> Option<AffiliatedKeyword> {
        affiliated_keyword(&self.syntax, |k| {
            k.starts_with("ATTR_") && &k[5..] == backend
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OrgTableRow {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for OrgTableRow {
    type Language = OrgLanguage;
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == ORG_TABLE_RULE_ROW || kind == ORG_TABLE_STANDARD_ROW
    }
    fn cast(node: SyntaxNode) -> Option<OrgTableRow> {
        Self::can_cast(node.kind()).then(|| OrgTableRow { syntax: node })
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl OrgTableRow {
    pub fn begin(&self) -> u32 {
        self.syntax.text_range().start().into()
    }
    pub fn end(&self) -> u32 {
        self.syntax.text_range().end().into()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OrgTableCell {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for OrgTableCell {
    type Language = OrgLanguage;
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == ORG_TABLE_CELL
    }
    fn cast(node: SyntaxNode) -> Option<OrgTableCell> {
        Self::can_cast(node.kind()).then(|| OrgTableCell { syntax: node })
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl OrgTableCell {
    pub fn begin(&self) -> u32 {
        self.syntax.text_range().start().into()
    }
    pub fn end(&self) -> u32 {
        self.syntax.text_range().end().into()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct List {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for List {
    type Language = OrgLanguage;
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == LIST
    }
    fn cast(node: SyntaxNode) -> Option<List> {
        Self::can_cast(node.kind()).then(|| List { syntax: node })
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl List {
    pub fn begin(&self) -> u32 {
        self.syntax.text_range().start().into()
    }
    pub fn end(&self) -> u32 {
        self.syntax.text_range().end().into()
    }
    pub fn items(&self) -> AstChildren<ListItem> {
        support::children(&self.syntax)
    }
    pub fn caption(&self) -> Option<AffiliatedKeyword> {
        affiliated_keyword(&self.syntax, |k| k == "CAPTION")
    }
    pub fn header(&self) -> Option<AffiliatedKeyword> {
        affiliated_keyword(&self.syntax, |k| k == "HEADER")
    }
    pub fn name(&self) -> Option<AffiliatedKeyword> {
        affiliated_keyword(&self.syntax, |k| k == "NAME")
    }
    pub fn plot(&self) -> Option<AffiliatedKeyword> {
        affiliated_keyword(&self.syntax, |k| k == "PLOT")
    }
    pub fn results(&self) -> Option<AffiliatedKeyword> {
        affiliated_keyword(&self.syntax, |k| k == "RESULTS")
    }
    pub fn attr(&self, backend: &str) -> Option<AffiliatedKeyword> {
        affiliated_keyword(&self.syntax, |k| {
            k.starts_with("ATTR_") && &k[5..] == backend
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ListItem {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for ListItem {
    type Language = OrgLanguage;
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == LIST_ITEM
    }
    fn cast(node: SyntaxNode) -> Option<ListItem> {
        Self::can_cast(node.kind()).then(|| ListItem { syntax: node })
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl ListItem {
    pub fn begin(&self) -> u32 {
        self.syntax.text_range().start().into()
    }
    pub fn end(&self) -> u32 {
        self.syntax.text_range().end().into()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Drawer {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for Drawer {
    type Language = OrgLanguage;
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == DRAWER
    }
    fn cast(node: SyntaxNode) -> Option<Drawer> {
        Self::can_cast(node.kind()).then(|| Drawer { syntax: node })
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl Drawer {
    pub fn begin(&self) -> u32 {
        self.syntax.text_range().start().into()
    }
    pub fn end(&self) -> u32 {
        self.syntax.text_range().end().into()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DynBlock {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for DynBlock {
    type Language = OrgLanguage;
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == DYN_BLOCK
    }
    fn cast(node: SyntaxNode) -> Option<DynBlock> {
        Self::can_cast(node.kind()).then(|| DynBlock { syntax: node })
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl DynBlock {
    pub fn begin(&self) -> u32 {
        self.syntax.text_range().start().into()
    }
    pub fn end(&self) -> u32 {
        self.syntax.text_range().end().into()
    }
    pub fn caption(&self) -> Option<AffiliatedKeyword> {
        affiliated_keyword(&self.syntax, |k| k == "CAPTION")
    }
    pub fn header(&self) -> Option<AffiliatedKeyword> {
        affiliated_keyword(&self.syntax, |k| k == "HEADER")
    }
    pub fn name(&self) -> Option<AffiliatedKeyword> {
        affiliated_keyword(&self.syntax, |k| k == "NAME")
    }
    pub fn plot(&self) -> Option<AffiliatedKeyword> {
        affiliated_keyword(&self.syntax, |k| k == "PLOT")
    }
    pub fn results(&self) -> Option<AffiliatedKeyword> {
        affiliated_keyword(&self.syntax, |k| k == "RESULTS")
    }
    pub fn attr(&self, backend: &str) -> Option<AffiliatedKeyword> {
        affiliated_keyword(&self.syntax, |k| {
            k.starts_with("ATTR_") && &k[5..] == backend
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Keyword {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for Keyword {
    type Language = OrgLanguage;
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == KEYWORD
    }
    fn cast(node: SyntaxNode) -> Option<Keyword> {
        Self::can_cast(node.kind()).then(|| Keyword { syntax: node })
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl Keyword {
    pub fn begin(&self) -> u32 {
        self.syntax.text_range().start().into()
    }
    pub fn end(&self) -> u32 {
        self.syntax.text_range().end().into()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BabelCall {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for BabelCall {
    type Language = OrgLanguage;
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == BABEL_CALL
    }
    fn cast(node: SyntaxNode) -> Option<BabelCall> {
        Self::can_cast(node.kind()).then(|| BabelCall { syntax: node })
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl BabelCall {
    pub fn begin(&self) -> u32 {
        self.syntax.text_range().start().into()
    }
    pub fn end(&self) -> u32 {
        self.syntax.text_range().end().into()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AffiliatedKeyword {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for AffiliatedKeyword {
    type Language = OrgLanguage;
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == AFFILIATED_KEYWORD
    }
    fn cast(node: SyntaxNode) -> Option<AffiliatedKeyword> {
        Self::can_cast(node.kind()).then(|| AffiliatedKeyword { syntax: node })
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AffiliatedKeyword {
    pub fn begin(&self) -> u32 {
        self.syntax.text_range().start().into()
    }
    pub fn end(&self) -> u32 {
        self.syntax.text_range().end().into()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TableEl {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for TableEl {
    type Language = OrgLanguage;
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == TABLE_EL
    }
    fn cast(node: SyntaxNode) -> Option<TableEl> {
        Self::can_cast(node.kind()).then(|| TableEl { syntax: node })
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl TableEl {
    pub fn begin(&self) -> u32 {
        self.syntax.text_range().start().into()
    }
    pub fn end(&self) -> u32 {
        self.syntax.text_range().end().into()
    }
    pub fn post_blank(&self) -> usize {
        super::blank_lines(&self.syntax)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Clock {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for Clock {
    type Language = OrgLanguage;
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CLOCK
    }
    fn cast(node: SyntaxNode) -> Option<Clock> {
        Self::can_cast(node.kind()).then(|| Clock { syntax: node })
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl Clock {
    pub fn begin(&self) -> u32 {
        self.syntax.text_range().start().into()
    }
    pub fn end(&self) -> u32 {
        self.syntax.text_range().end().into()
    }
    pub fn post_blank(&self) -> usize {
        super::blank_lines(&self.syntax)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FnDef {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for FnDef {
    type Language = OrgLanguage;
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == FN_DEF
    }
    fn cast(node: SyntaxNode) -> Option<FnDef> {
        Self::can_cast(node.kind()).then(|| FnDef { syntax: node })
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl FnDef {
    pub fn begin(&self) -> u32 {
        self.syntax.text_range().start().into()
    }
    pub fn end(&self) -> u32 {
        self.syntax.text_range().end().into()
    }
    pub fn post_blank(&self) -> usize {
        super::blank_lines(&self.syntax)
    }
    pub fn caption(&self) -> Option<AffiliatedKeyword> {
        affiliated_keyword(&self.syntax, |k| k == "CAPTION")
    }
    pub fn header(&self) -> Option<AffiliatedKeyword> {
        affiliated_keyword(&self.syntax, |k| k == "HEADER")
    }
    pub fn name(&self) -> Option<AffiliatedKeyword> {
        affiliated_keyword(&self.syntax, |k| k == "NAME")
    }
    pub fn plot(&self) -> Option<AffiliatedKeyword> {
        affiliated_keyword(&self.syntax, |k| k == "PLOT")
    }
    pub fn results(&self) -> Option<AffiliatedKeyword> {
        affiliated_keyword(&self.syntax, |k| k == "RESULTS")
    }
    pub fn attr(&self, backend: &str) -> Option<AffiliatedKeyword> {
        affiliated_keyword(&self.syntax, |k| {
            k.starts_with("ATTR_") && &k[5..] == backend
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Comment {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for Comment {
    type Language = OrgLanguage;
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == COMMENT
    }
    fn cast(node: SyntaxNode) -> Option<Comment> {
        Self::can_cast(node.kind()).then(|| Comment { syntax: node })
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl Comment {
    pub fn begin(&self) -> u32 {
        self.syntax.text_range().start().into()
    }
    pub fn end(&self) -> u32 {
        self.syntax.text_range().end().into()
    }
    pub fn text(&self) -> Option<super::Token> {
        super::token(&self.syntax, TEXT)
    }
    pub fn post_blank(&self) -> usize {
        super::blank_lines(&self.syntax)
    }
    pub fn caption(&self) -> Option<AffiliatedKeyword> {
        affiliated_keyword(&self.syntax, |k| k == "CAPTION")
    }
    pub fn header(&self) -> Option<AffiliatedKeyword> {
        affiliated_keyword(&self.syntax, |k| k == "HEADER")
    }
    pub fn name(&self) -> Option<AffiliatedKeyword> {
        affiliated_keyword(&self.syntax, |k| k == "NAME")
    }
    pub fn plot(&self) -> Option<AffiliatedKeyword> {
        affiliated_keyword(&self.syntax, |k| k == "PLOT")
    }
    pub fn results(&self) -> Option<AffiliatedKeyword> {
        affiliated_keyword(&self.syntax, |k| k == "RESULTS")
    }
    pub fn attr(&self, backend: &str) -> Option<AffiliatedKeyword> {
        affiliated_keyword(&self.syntax, |k| {
            k.starts_with("ATTR_") && &k[5..] == backend
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Rule {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for Rule {
    type Language = OrgLanguage;
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == RULE
    }
    fn cast(node: SyntaxNode) -> Option<Rule> {
        Self::can_cast(node.kind()).then(|| Rule { syntax: node })
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl Rule {
    pub fn begin(&self) -> u32 {
        self.syntax.text_range().start().into()
    }
    pub fn end(&self) -> u32 {
        self.syntax.text_range().end().into()
    }
    pub fn post_blank(&self) -> usize {
        super::blank_lines(&self.syntax)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FixedWidth {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for FixedWidth {
    type Language = OrgLanguage;
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == FIXED_WIDTH
    }
    fn cast(node: SyntaxNode) -> Option<FixedWidth> {
        Self::can_cast(node.kind()).then(|| FixedWidth { syntax: node })
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl FixedWidth {
    pub fn begin(&self) -> u32 {
        self.syntax.text_range().start().into()
    }
    pub fn end(&self) -> u32 {
        self.syntax.text_range().end().into()
    }
    pub fn text(&self) -> Option<super::Token> {
        super::token(&self.syntax, TEXT)
    }
    pub fn post_blank(&self) -> usize {
        super::blank_lines(&self.syntax)
    }
    pub fn caption(&self) -> Option<AffiliatedKeyword> {
        affiliated_keyword(&self.syntax, |k| k == "CAPTION")
    }
    pub fn header(&self) -> Option<AffiliatedKeyword> {
        affiliated_keyword(&self.syntax, |k| k == "HEADER")
    }
    pub fn name(&self) -> Option<AffiliatedKeyword> {
        affiliated_keyword(&self.syntax, |k| k == "NAME")
    }
    pub fn plot(&self) -> Option<AffiliatedKeyword> {
        affiliated_keyword(&self.syntax, |k| k == "PLOT")
    }
    pub fn results(&self) -> Option<AffiliatedKeyword> {
        affiliated_keyword(&self.syntax, |k| k == "RESULTS")
    }
    pub fn attr(&self, backend: &str) -> Option<AffiliatedKeyword> {
        affiliated_keyword(&self.syntax, |k| {
            k.starts_with("ATTR_") && &k[5..] == backend
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SpecialBlock {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for SpecialBlock {
    type Language = OrgLanguage;
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SPECIAL_BLOCK
    }
    fn cast(node: SyntaxNode) -> Option<SpecialBlock> {
        Self::can_cast(node.kind()).then(|| SpecialBlock { syntax: node })
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl SpecialBlock {
    pub fn begin(&self) -> u32 {
        self.syntax.text_range().start().into()
    }
    pub fn end(&self) -> u32 {
        self.syntax.text_range().end().into()
    }
    pub fn caption(&self) -> Option<AffiliatedKeyword> {
        affiliated_keyword(&self.syntax, |k| k == "CAPTION")
    }
    pub fn header(&self) -> Option<AffiliatedKeyword> {
        affiliated_keyword(&self.syntax, |k| k == "HEADER")
    }
    pub fn name(&self) -> Option<AffiliatedKeyword> {
        affiliated_keyword(&self.syntax, |k| k == "NAME")
    }
    pub fn plot(&self) -> Option<AffiliatedKeyword> {
        affiliated_keyword(&self.syntax, |k| k == "PLOT")
    }
    pub fn results(&self) -> Option<AffiliatedKeyword> {
        affiliated_keyword(&self.syntax, |k| k == "RESULTS")
    }
    pub fn attr(&self, backend: &str) -> Option<AffiliatedKeyword> {
        affiliated_keyword(&self.syntax, |k| {
            k.starts_with("ATTR_") && &k[5..] == backend
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct QuoteBlock {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for QuoteBlock {
    type Language = OrgLanguage;
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == QUOTE_BLOCK
    }
    fn cast(node: SyntaxNode) -> Option<QuoteBlock> {
        Self::can_cast(node.kind()).then(|| QuoteBlock { syntax: node })
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl QuoteBlock {
    pub fn begin(&self) -> u32 {
        self.syntax.text_range().start().into()
    }
    pub fn end(&self) -> u32 {
        self.syntax.text_range().end().into()
    }
    pub fn caption(&self) -> Option<AffiliatedKeyword> {
        affiliated_keyword(&self.syntax, |k| k == "CAPTION")
    }
    pub fn header(&self) -> Option<AffiliatedKeyword> {
        affiliated_keyword(&self.syntax, |k| k == "HEADER")
    }
    pub fn name(&self) -> Option<AffiliatedKeyword> {
        affiliated_keyword(&self.syntax, |k| k == "NAME")
    }
    pub fn plot(&self) -> Option<AffiliatedKeyword> {
        affiliated_keyword(&self.syntax, |k| k == "PLOT")
    }
    pub fn results(&self) -> Option<AffiliatedKeyword> {
        affiliated_keyword(&self.syntax, |k| k == "RESULTS")
    }
    pub fn attr(&self, backend: &str) -> Option<AffiliatedKeyword> {
        affiliated_keyword(&self.syntax, |k| {
            k.starts_with("ATTR_") && &k[5..] == backend
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CenterBlock {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for CenterBlock {
    type Language = OrgLanguage;
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CENTER_BLOCK
    }
    fn cast(node: SyntaxNode) -> Option<CenterBlock> {
        Self::can_cast(node.kind()).then(|| CenterBlock { syntax: node })
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl CenterBlock {
    pub fn begin(&self) -> u32 {
        self.syntax.text_range().start().into()
    }
    pub fn end(&self) -> u32 {
        self.syntax.text_range().end().into()
    }
    pub fn caption(&self) -> Option<AffiliatedKeyword> {
        affiliated_keyword(&self.syntax, |k| k == "CAPTION")
    }
    pub fn header(&self) -> Option<AffiliatedKeyword> {
        affiliated_keyword(&self.syntax, |k| k == "HEADER")
    }
    pub fn name(&self) -> Option<AffiliatedKeyword> {
        affiliated_keyword(&self.syntax, |k| k == "NAME")
    }
    pub fn plot(&self) -> Option<AffiliatedKeyword> {
        affiliated_keyword(&self.syntax, |k| k == "PLOT")
    }
    pub fn results(&self) -> Option<AffiliatedKeyword> {
        affiliated_keyword(&self.syntax, |k| k == "RESULTS")
    }
    pub fn attr(&self, backend: &str) -> Option<AffiliatedKeyword> {
        affiliated_keyword(&self.syntax, |k| {
            k.starts_with("ATTR_") && &k[5..] == backend
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VerseBlock {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for VerseBlock {
    type Language = OrgLanguage;
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == VERSE_BLOCK
    }
    fn cast(node: SyntaxNode) -> Option<VerseBlock> {
        Self::can_cast(node.kind()).then(|| VerseBlock { syntax: node })
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl VerseBlock {
    pub fn begin(&self) -> u32 {
        self.syntax.text_range().start().into()
    }
    pub fn end(&self) -> u32 {
        self.syntax.text_range().end().into()
    }
    pub fn caption(&self) -> Option<AffiliatedKeyword> {
        affiliated_keyword(&self.syntax, |k| k == "CAPTION")
    }
    pub fn header(&self) -> Option<AffiliatedKeyword> {
        affiliated_keyword(&self.syntax, |k| k == "HEADER")
    }
    pub fn name(&self) -> Option<AffiliatedKeyword> {
        affiliated_keyword(&self.syntax, |k| k == "NAME")
    }
    pub fn plot(&self) -> Option<AffiliatedKeyword> {
        affiliated_keyword(&self.syntax, |k| k == "PLOT")
    }
    pub fn results(&self) -> Option<AffiliatedKeyword> {
        affiliated_keyword(&self.syntax, |k| k == "RESULTS")
    }
    pub fn attr(&self, backend: &str) -> Option<AffiliatedKeyword> {
        affiliated_keyword(&self.syntax, |k| {
            k.starts_with("ATTR_") && &k[5..] == backend
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CommentBlock {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for CommentBlock {
    type Language = OrgLanguage;
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == COMMENT_BLOCK
    }
    fn cast(node: SyntaxNode) -> Option<CommentBlock> {
        Self::can_cast(node.kind()).then(|| CommentBlock { syntax: node })
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl CommentBlock {
    pub fn begin(&self) -> u32 {
        self.syntax.text_range().start().into()
    }
    pub fn end(&self) -> u32 {
        self.syntax.text_range().end().into()
    }
    pub fn caption(&self) -> Option<AffiliatedKeyword> {
        affiliated_keyword(&self.syntax, |k| k == "CAPTION")
    }
    pub fn header(&self) -> Option<AffiliatedKeyword> {
        affiliated_keyword(&self.syntax, |k| k == "HEADER")
    }
    pub fn name(&self) -> Option<AffiliatedKeyword> {
        affiliated_keyword(&self.syntax, |k| k == "NAME")
    }
    pub fn plot(&self) -> Option<AffiliatedKeyword> {
        affiliated_keyword(&self.syntax, |k| k == "PLOT")
    }
    pub fn results(&self) -> Option<AffiliatedKeyword> {
        affiliated_keyword(&self.syntax, |k| k == "RESULTS")
    }
    pub fn attr(&self, backend: &str) -> Option<AffiliatedKeyword> {
        affiliated_keyword(&self.syntax, |k| {
            k.starts_with("ATTR_") && &k[5..] == backend
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExampleBlock {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for ExampleBlock {
    type Language = OrgLanguage;
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == EXAMPLE_BLOCK
    }
    fn cast(node: SyntaxNode) -> Option<ExampleBlock> {
        Self::can_cast(node.kind()).then(|| ExampleBlock { syntax: node })
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl ExampleBlock {
    pub fn begin(&self) -> u32 {
        self.syntax.text_range().start().into()
    }
    pub fn end(&self) -> u32 {
        self.syntax.text_range().end().into()
    }
    pub fn caption(&self) -> Option<AffiliatedKeyword> {
        affiliated_keyword(&self.syntax, |k| k == "CAPTION")
    }
    pub fn header(&self) -> Option<AffiliatedKeyword> {
        affiliated_keyword(&self.syntax, |k| k == "HEADER")
    }
    pub fn name(&self) -> Option<AffiliatedKeyword> {
        affiliated_keyword(&self.syntax, |k| k == "NAME")
    }
    pub fn plot(&self) -> Option<AffiliatedKeyword> {
        affiliated_keyword(&self.syntax, |k| k == "PLOT")
    }
    pub fn results(&self) -> Option<AffiliatedKeyword> {
        affiliated_keyword(&self.syntax, |k| k == "RESULTS")
    }
    pub fn attr(&self, backend: &str) -> Option<AffiliatedKeyword> {
        affiliated_keyword(&self.syntax, |k| {
            k.starts_with("ATTR_") && &k[5..] == backend
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExportBlock {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for ExportBlock {
    type Language = OrgLanguage;
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == EXPORT_BLOCK
    }
    fn cast(node: SyntaxNode) -> Option<ExportBlock> {
        Self::can_cast(node.kind()).then(|| ExportBlock { syntax: node })
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl ExportBlock {
    pub fn begin(&self) -> u32 {
        self.syntax.text_range().start().into()
    }
    pub fn end(&self) -> u32 {
        self.syntax.text_range().end().into()
    }
    pub fn caption(&self) -> Option<AffiliatedKeyword> {
        affiliated_keyword(&self.syntax, |k| k == "CAPTION")
    }
    pub fn header(&self) -> Option<AffiliatedKeyword> {
        affiliated_keyword(&self.syntax, |k| k == "HEADER")
    }
    pub fn name(&self) -> Option<AffiliatedKeyword> {
        affiliated_keyword(&self.syntax, |k| k == "NAME")
    }
    pub fn plot(&self) -> Option<AffiliatedKeyword> {
        affiliated_keyword(&self.syntax, |k| k == "PLOT")
    }
    pub fn results(&self) -> Option<AffiliatedKeyword> {
        affiliated_keyword(&self.syntax, |k| k == "RESULTS")
    }
    pub fn attr(&self, backend: &str) -> Option<AffiliatedKeyword> {
        affiliated_keyword(&self.syntax, |k| {
            k.starts_with("ATTR_") && &k[5..] == backend
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SourceBlock {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for SourceBlock {
    type Language = OrgLanguage;
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SOURCE_BLOCK
    }
    fn cast(node: SyntaxNode) -> Option<SourceBlock> {
        Self::can_cast(node.kind()).then(|| SourceBlock { syntax: node })
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl SourceBlock {
    pub fn begin(&self) -> u32 {
        self.syntax.text_range().start().into()
    }
    pub fn end(&self) -> u32 {
        self.syntax.text_range().end().into()
    }
    pub fn caption(&self) -> Option<AffiliatedKeyword> {
        affiliated_keyword(&self.syntax, |k| k == "CAPTION")
    }
    pub fn header(&self) -> Option<AffiliatedKeyword> {
        affiliated_keyword(&self.syntax, |k| k == "HEADER")
    }
    pub fn name(&self) -> Option<AffiliatedKeyword> {
        affiliated_keyword(&self.syntax, |k| k == "NAME")
    }
    pub fn plot(&self) -> Option<AffiliatedKeyword> {
        affiliated_keyword(&self.syntax, |k| k == "PLOT")
    }
    pub fn results(&self) -> Option<AffiliatedKeyword> {
        affiliated_keyword(&self.syntax, |k| k == "RESULTS")
    }
    pub fn attr(&self, backend: &str) -> Option<AffiliatedKeyword> {
        affiliated_keyword(&self.syntax, |k| {
            k.starts_with("ATTR_") && &k[5..] == backend
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct InlineCall {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for InlineCall {
    type Language = OrgLanguage;
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == INLINE_CALL
    }
    fn cast(node: SyntaxNode) -> Option<InlineCall> {
        Self::can_cast(node.kind()).then(|| InlineCall { syntax: node })
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl InlineCall {
    pub fn begin(&self) -> u32 {
        self.syntax.text_range().start().into()
    }
    pub fn end(&self) -> u32 {
        self.syntax.text_range().end().into()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct InlineSrc {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for InlineSrc {
    type Language = OrgLanguage;
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == INLINE_SRC
    }
    fn cast(node: SyntaxNode) -> Option<InlineSrc> {
        Self::can_cast(node.kind()).then(|| InlineSrc { syntax: node })
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl InlineSrc {
    pub fn begin(&self) -> u32 {
        self.syntax.text_range().start().into()
    }
    pub fn end(&self) -> u32 {
        self.syntax.text_range().end().into()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Link {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for Link {
    type Language = OrgLanguage;
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == LINK
    }
    fn cast(node: SyntaxNode) -> Option<Link> {
        Self::can_cast(node.kind()).then(|| Link { syntax: node })
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl Link {
    pub fn begin(&self) -> u32 {
        self.syntax.text_range().start().into()
    }
    pub fn end(&self) -> u32 {
        self.syntax.text_range().end().into()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Cookie {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for Cookie {
    type Language = OrgLanguage;
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == COOKIE
    }
    fn cast(node: SyntaxNode) -> Option<Cookie> {
        Self::can_cast(node.kind()).then(|| Cookie { syntax: node })
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl Cookie {
    pub fn begin(&self) -> u32 {
        self.syntax.text_range().start().into()
    }
    pub fn end(&self) -> u32 {
        self.syntax.text_range().end().into()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RadioTarget {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for RadioTarget {
    type Language = OrgLanguage;
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == RADIO_TARGET
    }
    fn cast(node: SyntaxNode) -> Option<RadioTarget> {
        Self::can_cast(node.kind()).then(|| RadioTarget { syntax: node })
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl RadioTarget {
    pub fn begin(&self) -> u32 {
        self.syntax.text_range().start().into()
    }
    pub fn end(&self) -> u32 {
        self.syntax.text_range().end().into()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FnRef {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for FnRef {
    type Language = OrgLanguage;
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == FN_REF
    }
    fn cast(node: SyntaxNode) -> Option<FnRef> {
        Self::can_cast(node.kind()).then(|| FnRef { syntax: node })
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl FnRef {
    pub fn begin(&self) -> u32 {
        self.syntax.text_range().start().into()
    }
    pub fn end(&self) -> u32 {
        self.syntax.text_range().end().into()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Macros {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for Macros {
    type Language = OrgLanguage;
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == MACROS
    }
    fn cast(node: SyntaxNode) -> Option<Macros> {
        Self::can_cast(node.kind()).then(|| Macros { syntax: node })
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl Macros {
    pub fn begin(&self) -> u32 {
        self.syntax.text_range().start().into()
    }
    pub fn end(&self) -> u32 {
        self.syntax.text_range().end().into()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Snippet {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for Snippet {
    type Language = OrgLanguage;
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SNIPPET
    }
    fn cast(node: SyntaxNode) -> Option<Snippet> {
        Self::can_cast(node.kind()).then(|| Snippet { syntax: node })
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl Snippet {
    pub fn begin(&self) -> u32 {
        self.syntax.text_range().start().into()
    }
    pub fn end(&self) -> u32 {
        self.syntax.text_range().end().into()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Target {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for Target {
    type Language = OrgLanguage;
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == TARGET
    }
    fn cast(node: SyntaxNode) -> Option<Target> {
        Self::can_cast(node.kind()).then(|| Target { syntax: node })
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl Target {
    pub fn begin(&self) -> u32 {
        self.syntax.text_range().start().into()
    }
    pub fn end(&self) -> u32 {
        self.syntax.text_range().end().into()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Bold {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for Bold {
    type Language = OrgLanguage;
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == BOLD
    }
    fn cast(node: SyntaxNode) -> Option<Bold> {
        Self::can_cast(node.kind()).then(|| Bold { syntax: node })
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl Bold {
    pub fn begin(&self) -> u32 {
        self.syntax.text_range().start().into()
    }
    pub fn end(&self) -> u32 {
        self.syntax.text_range().end().into()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Strike {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for Strike {
    type Language = OrgLanguage;
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == STRIKE
    }
    fn cast(node: SyntaxNode) -> Option<Strike> {
        Self::can_cast(node.kind()).then(|| Strike { syntax: node })
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl Strike {
    pub fn begin(&self) -> u32 {
        self.syntax.text_range().start().into()
    }
    pub fn end(&self) -> u32 {
        self.syntax.text_range().end().into()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Italic {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for Italic {
    type Language = OrgLanguage;
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == ITALIC
    }
    fn cast(node: SyntaxNode) -> Option<Italic> {
        Self::can_cast(node.kind()).then(|| Italic { syntax: node })
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl Italic {
    pub fn begin(&self) -> u32 {
        self.syntax.text_range().start().into()
    }
    pub fn end(&self) -> u32 {
        self.syntax.text_range().end().into()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Underline {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for Underline {
    type Language = OrgLanguage;
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == UNDERLINE
    }
    fn cast(node: SyntaxNode) -> Option<Underline> {
        Self::can_cast(node.kind()).then(|| Underline { syntax: node })
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl Underline {
    pub fn begin(&self) -> u32 {
        self.syntax.text_range().start().into()
    }
    pub fn end(&self) -> u32 {
        self.syntax.text_range().end().into()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Verbatim {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for Verbatim {
    type Language = OrgLanguage;
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == VERBATIM
    }
    fn cast(node: SyntaxNode) -> Option<Verbatim> {
        Self::can_cast(node.kind()).then(|| Verbatim { syntax: node })
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl Verbatim {
    pub fn begin(&self) -> u32 {
        self.syntax.text_range().start().into()
    }
    pub fn end(&self) -> u32 {
        self.syntax.text_range().end().into()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Code {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for Code {
    type Language = OrgLanguage;
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CODE
    }
    fn cast(node: SyntaxNode) -> Option<Code> {
        Self::can_cast(node.kind()).then(|| Code { syntax: node })
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl Code {
    pub fn begin(&self) -> u32 {
        self.syntax.text_range().start().into()
    }
    pub fn end(&self) -> u32 {
        self.syntax.text_range().end().into()
    }
    pub fn text(&self) -> Option<super::Token> {
        super::token(&self.syntax, TEXT)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Timestamp {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for Timestamp {
    type Language = OrgLanguage;
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == TIMESTAMP_ACTIVE || kind == TIMESTAMP_INACTIVE || kind == TIMESTAMP_DIARY
    }
    fn cast(node: SyntaxNode) -> Option<Timestamp> {
        Self::can_cast(node.kind()).then(|| Timestamp { syntax: node })
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl Timestamp {
    pub fn begin(&self) -> u32 {
        self.syntax.text_range().start().into()
    }
    pub fn end(&self) -> u32 {
        self.syntax.text_range().end().into()
    }
    pub fn year_start(&self) -> Option<super::Token> {
        super::token(&self.syntax, TIMESTAMP_YEAR)
    }
    pub fn month_start(&self) -> Option<super::Token> {
        super::token(&self.syntax, TIMESTAMP_MONTH)
    }
    pub fn day_start(&self) -> Option<super::Token> {
        super::token(&self.syntax, TIMESTAMP_DAY)
    }
    pub fn hour_start(&self) -> Option<super::Token> {
        super::token(&self.syntax, TIMESTAMP_HOUR)
    }
    pub fn minute_start(&self) -> Option<super::Token> {
        super::token(&self.syntax, TIMESTAMP_MINUTE)
    }
    pub fn year_end(&self) -> Option<super::Token> {
        super::last_token(&self.syntax, TIMESTAMP_YEAR)
    }
    pub fn month_end(&self) -> Option<super::Token> {
        super::last_token(&self.syntax, TIMESTAMP_MONTH)
    }
    pub fn day_end(&self) -> Option<super::Token> {
        super::last_token(&self.syntax, TIMESTAMP_DAY)
    }
    pub fn hour_end(&self) -> Option<super::Token> {
        super::last_token(&self.syntax, TIMESTAMP_HOUR)
    }
    pub fn minute_end(&self) -> Option<super::Token> {
        super::last_token(&self.syntax, TIMESTAMP_MINUTE)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LatexEnvironment {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for LatexEnvironment {
    type Language = OrgLanguage;
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == LATEX_ENVIRONMENT
    }
    fn cast(node: SyntaxNode) -> Option<LatexEnvironment> {
        Self::can_cast(node.kind()).then(|| LatexEnvironment { syntax: node })
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl LatexEnvironment {
    pub fn begin(&self) -> u32 {
        self.syntax.text_range().start().into()
    }
    pub fn end(&self) -> u32 {
        self.syntax.text_range().end().into()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LatexFragment {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for LatexFragment {
    type Language = OrgLanguage;
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == LATEX_FRAGMENT
    }
    fn cast(node: SyntaxNode) -> Option<LatexFragment> {
        Self::can_cast(node.kind()).then(|| LatexFragment { syntax: node })
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl LatexFragment {
    pub fn begin(&self) -> u32 {
        self.syntax.text_range().start().into()
    }
    pub fn end(&self) -> u32 {
        self.syntax.text_range().end().into()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Entity {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for Entity {
    type Language = OrgLanguage;
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == ENTITY
    }
    fn cast(node: SyntaxNode) -> Option<Entity> {
        Self::can_cast(node.kind()).then(|| Entity { syntax: node })
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl Entity {
    pub fn begin(&self) -> u32 {
        self.syntax.text_range().start().into()
    }
    pub fn end(&self) -> u32 {
        self.syntax.text_range().end().into()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LineBreak {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for LineBreak {
    type Language = OrgLanguage;
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == LINE_BREAK
    }
    fn cast(node: SyntaxNode) -> Option<LineBreak> {
        Self::can_cast(node.kind()).then(|| LineBreak { syntax: node })
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl LineBreak {
    pub fn begin(&self) -> u32 {
        self.syntax.text_range().start().into()
    }
    pub fn end(&self) -> u32 {
        self.syntax.text_range().end().into()
    }
}
