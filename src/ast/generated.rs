//! generated file, do not modify it directly
#![allow(clippy::all)]
#![allow(unused)]

use crate::syntax::{OrgLanguage, SyntaxKind, SyntaxKind::*, SyntaxNode, SyntaxToken};
use rowan::ast::{support::{self, token}, AstChildren, AstNode};

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
    pub fn post_blank(&self) -> usize {
        super::blank_lines(&self.syntax)
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
    pub fn stars(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, HEADLINE_STARS)
    }
    pub fn keyword(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, HEADLINE_KEYWORD)
    }
    pub fn title(&self) -> Option<HeadlineTitle> {
        support::child(&self.syntax)
    }
    pub fn section(&self) -> Option<Section> {
        support::child(&self.syntax)
    }
    pub fn tags(&self) -> Option<HeadlineTags> {
        support::child(&self.syntax)
    }
    pub fn planning(&self) -> Option<Planning> {
        support::child(&self.syntax)
    }
    pub fn priority(&self) -> Option<HeadlinePriority> {
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
pub struct HeadlineStars {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for HeadlineStars {
    type Language = OrgLanguage;
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == HEADLINE_STARS
    }
    fn cast(node: SyntaxNode) -> Option<HeadlineStars> {
        Self::can_cast(node.kind()).then(|| HeadlineStars { syntax: node })
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl HeadlineStars {
    pub fn headline(&self) -> Option<Headline> {
        self.syntax.parent().and_then(Headline::cast)
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
    pub fn headline(&self) -> Option<Headline> {
        self.syntax.parent().and_then(Headline::cast)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HeadlineKeyword {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for HeadlineKeyword {
    type Language = OrgLanguage;
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == HEADLINE_KEYWORD
    }
    fn cast(node: SyntaxNode) -> Option<HeadlineKeyword> {
        Self::can_cast(node.kind()).then(|| HeadlineKeyword { syntax: node })
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl HeadlineKeyword {
    pub fn headline(&self) -> Option<Headline> {
        self.syntax.parent().and_then(Headline::cast)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HeadlinePriority {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for HeadlinePriority {
    type Language = OrgLanguage;
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == HEADLINE_PRIORITY
    }
    fn cast(node: SyntaxNode) -> Option<HeadlinePriority> {
        Self::can_cast(node.kind()).then(|| HeadlinePriority { syntax: node })
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl HeadlinePriority {
    pub fn text(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, TEXT)
    }
    pub fn headline(&self) -> Option<Headline> {
        self.syntax.parent().and_then(Headline::cast)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HeadlineTags {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for HeadlineTags {
    type Language = OrgLanguage;
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == HEADLINE_TAGS
    }
    fn cast(node: SyntaxNode) -> Option<HeadlineTags> {
        Self::can_cast(node.kind()).then(|| HeadlineTags { syntax: node })
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl HeadlineTags {
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
impl NodeProperty {}

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
    pub fn deadline(&self) -> Option<PlanningDeadline> {
        super::last_child(&self.syntax)
    }
    pub fn scheduled(&self) -> Option<PlanningScheduled> {
        super::last_child(&self.syntax)
    }
    pub fn closed(&self) -> Option<PlanningClosed> {
        super::last_child(&self.syntax)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PlanningDeadline {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for PlanningDeadline {
    type Language = OrgLanguage;
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == PLANNING_DEADLINE
    }
    fn cast(node: SyntaxNode) -> Option<PlanningDeadline> {
        Self::can_cast(node.kind()).then(|| PlanningDeadline { syntax: node })
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl PlanningDeadline {}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PlanningScheduled {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for PlanningScheduled {
    type Language = OrgLanguage;
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == PLANNING_SCHEDULED
    }
    fn cast(node: SyntaxNode) -> Option<PlanningScheduled> {
        Self::can_cast(node.kind()).then(|| PlanningScheduled { syntax: node })
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl PlanningScheduled {}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PlanningClosed {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for PlanningClosed {
    type Language = OrgLanguage;
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == PLANNING_CLOSED
    }
    fn cast(node: SyntaxNode) -> Option<PlanningClosed> {
        Self::can_cast(node.kind()).then(|| PlanningClosed { syntax: node })
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl PlanningClosed {}

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
    pub fn post_blank(&self) -> usize {
        super::blank_lines(&self.syntax)
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
impl OrgTableRow {}

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
impl OrgTableCell {}

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
    pub fn items(&self) -> AstChildren<ListItem> {
        support::children(&self.syntax)
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
    pub fn indent(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, LIST_ITEM_INDENT)
    }
    pub fn bullet(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, LIST_ITEM_BULLET)
    }
    pub fn content(&self) -> Option<ListItemContent> {
        support::child(&self.syntax)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ListItemIndent {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for ListItemIndent {
    type Language = OrgLanguage;
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == LIST_ITEM_INDENT
    }
    fn cast(node: SyntaxNode) -> Option<ListItemIndent> {
        Self::can_cast(node.kind()).then(|| ListItemIndent { syntax: node })
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl ListItemIndent {}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ListItemTag {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for ListItemTag {
    type Language = OrgLanguage;
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == LIST_ITEM_TAG
    }
    fn cast(node: SyntaxNode) -> Option<ListItemTag> {
        Self::can_cast(node.kind()).then(|| ListItemTag { syntax: node })
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl ListItemTag {}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ListItemBullet {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for ListItemBullet {
    type Language = OrgLanguage;
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == LIST_ITEM_BULLET
    }
    fn cast(node: SyntaxNode) -> Option<ListItemBullet> {
        Self::can_cast(node.kind()).then(|| ListItemBullet { syntax: node })
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl ListItemBullet {}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ListItemContent {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for ListItemContent {
    type Language = OrgLanguage;
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == LIST_ITEM_CONTENT
    }
    fn cast(node: SyntaxNode) -> Option<ListItemContent> {
        Self::can_cast(node.kind()).then(|| ListItemContent { syntax: node })
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl ListItemContent {}

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
impl Drawer {}

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
impl DynBlock {}

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
impl Keyword {}

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
impl BabelCall {}

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
    pub fn post_blank(&self) -> usize {
        super::blank_lines(&self.syntax)
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
    pub fn text(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, TEXT)
    }
    pub fn post_blank(&self) -> usize {
        super::blank_lines(&self.syntax)
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
    pub fn text(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, TEXT)
    }
    pub fn post_blank(&self) -> usize {
        super::blank_lines(&self.syntax)
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
impl SpecialBlock {}

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
impl QuoteBlock {}

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
impl CenterBlock {}

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
impl VerseBlock {}

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
impl CommentBlock {}

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
impl ExampleBlock {}

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
impl ExportBlock {}

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
impl SourceBlock {}

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
impl InlineCall {}

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
impl InlineSrc {}

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
    pub fn path(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, LINK_PATH)
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
impl Cookie {}

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
impl RadioTarget {}

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
impl FnRef {}

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
impl LatexEnvironment {}

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
impl Macros {}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MacrosArgument {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for MacrosArgument {
    type Language = OrgLanguage;
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == MACROS_ARGUMENT
    }
    fn cast(node: SyntaxNode) -> Option<MacrosArgument> {
        Self::can_cast(node.kind()).then(|| MacrosArgument { syntax: node })
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl MacrosArgument {}

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
    pub fn name(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, TEXT)
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
impl Target {}

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
impl Bold {}

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
impl Strike {}

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
impl Italic {}

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
impl Underline {}

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
impl Verbatim {}

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
    pub fn text(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, TEXT)
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
    pub fn year_start(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, TIMESTAMP_YEAR)
    }
    pub fn month_start(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, TIMESTAMP_MONTH)
    }
    pub fn day_start(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, TIMESTAMP_DAY)
    }
    pub fn hour_start(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, TIMESTAMP_HOUR)
    }
    pub fn minute_start(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, TIMESTAMP_MINUTE)
    }
    pub fn year_end(&self) -> Option<SyntaxToken> {
        super::last_token(&self.syntax, TIMESTAMP_YEAR)
    }
    pub fn month_end(&self) -> Option<SyntaxToken> {
        super::last_token(&self.syntax, TIMESTAMP_MONTH)
    }
    pub fn day_end(&self) -> Option<SyntaxToken> {
        super::last_token(&self.syntax, TIMESTAMP_DAY)
    }
    pub fn hour_end(&self) -> Option<SyntaxToken> {
        super::last_token(&self.syntax, TIMESTAMP_HOUR)
    }
    pub fn minute_end(&self) -> Option<SyntaxToken> {
        super::last_token(&self.syntax, TIMESTAMP_MINUTE)
    }
}
