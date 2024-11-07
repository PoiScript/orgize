use crate::ast::*;

#[non_exhaustive]
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Container {
    Document(Document),
    Section(Section),
    Paragraph(Paragraph),
    Headline(Headline),

    OrgTable(OrgTable),
    OrgTableRow(OrgTableRow),
    OrgTableCell(OrgTableCell),
    TableEl(TableEl),

    List(List),
    ListItem(ListItem),
    Drawer(Drawer),
    DynBlock(DynBlock),

    FnDef(FnDef),
    FnContent(FnContent),
    Comment(Comment),
    FixedWidth(FixedWidth),
    SpecialBlock(SpecialBlock),
    QuoteBlock(QuoteBlock),
    CenterBlock(CenterBlock),
    VerseBlock(VerseBlock),
    CommentBlock(CommentBlock),
    ExampleBlock(ExampleBlock),
    ExportBlock(ExportBlock),
    SourceBlock(SourceBlock),

    Link(Link),
    RadioTarget(RadioTarget),
    FnRef(FnRef),
    Target(Target),
    Bold(Bold),
    Strike(Strike),
    Italic(Italic),
    Underline(Underline),
    Verbatim(Verbatim),
    Code(Code),
    Superscript(Superscript),
    Subscript(Subscript),
    BabelCall(BabelCall),
    PropertyDrawer(PropertyDrawer),
    AffiliatedKeyword(AffiliatedKeyword),
    Keyword(Keyword),
}

#[non_exhaustive]
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Event {
    Enter(Container),
    Leave(Container),

    Text(Token),
    Macros(Macros),
    Cookie(Cookie),
    FnLabel(Token),
    InlineCall(InlineCall),
    InlineSrc(InlineSrc),
    Clock(Clock),
    LineBreak(LineBreak),
    Snippet(Snippet),
    Rule(Rule),
    Timestamp(Timestamp),
    LatexFragment(LatexFragment),
    LatexEnvironment(LatexEnvironment),
    Entity(Entity),

    #[cfg(feature = "syntax-org-fc")]
    Cloze(Cloze),
}
