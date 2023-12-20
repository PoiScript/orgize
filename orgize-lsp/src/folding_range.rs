use orgize::{
    export::{Container, Event, TraversalContext, Traverser},
    rowan::ast::AstNode,
    SyntaxKind, SyntaxNode,
};
use tower_lsp::lsp_types::{FoldingRange, FoldingRangeKind};

use crate::org_document::OrgDocument;

pub struct FoldingRangeTraverser<'a> {
    pub doc: &'a OrgDocument,
    pub ranges: Vec<FoldingRange>,
}

impl<'a> Traverser for FoldingRangeTraverser<'a> {
    fn event(&mut self, event: Event, _: &mut TraversalContext) {
        let syntax = match &event {
            Event::Enter(Container::Headline(i)) => i.syntax(),
            Event::Enter(Container::OrgTable(i)) => i.syntax(),
            Event::Enter(Container::TableEl(i)) => i.syntax(),
            Event::Enter(Container::List(i)) => i.syntax(),
            Event::Enter(Container::Drawer(i)) => i.syntax(),
            Event::Enter(Container::DynBlock(i)) => i.syntax(),
            Event::Enter(Container::SpecialBlock(i)) => i.syntax(),
            Event::Enter(Container::QuoteBlock(i)) => i.syntax(),
            Event::Enter(Container::CenterBlock(i)) => i.syntax(),
            Event::Enter(Container::VerseBlock(i)) => i.syntax(),
            Event::Enter(Container::CommentBlock(i)) => i.syntax(),
            Event::Enter(Container::ExampleBlock(i)) => i.syntax(),
            Event::Enter(Container::ExportBlock(i)) => i.syntax(),
            Event::Enter(Container::SourceBlock(i)) => i.syntax(),
            _ => return,
        };

        let (start, end) = if syntax.kind() == SyntaxKind::HEADLINE {
            let range = syntax.text_range();
            (range.start().into(), range.end().into())
        } else {
            get_block_folding_range(syntax)
        };

        let start_line = self.doc.line_of(start);
        let end_line = self.doc.line_of(end - 1);

        if start_line != end_line {
            self.ranges.push(FoldingRange {
                start_line,
                end_line,
                kind: Some(FoldingRangeKind::Region),
                ..Default::default()
            });
        }
    }
}

fn get_block_folding_range(syntax: &SyntaxNode) -> (u32, u32) {
    let start: u32 = syntax.text_range().start().into();

    // don't include blank lines in folding range
    let end = syntax
        .children()
        .take_while(|n| n.kind() != SyntaxKind::BLANK_LINE)
        .last();

    let end: u32 = end.map(|n| n.text_range().end().into()).unwrap_or(start);

    (start, end)
}

impl<'a> FoldingRangeTraverser<'a> {
    pub fn new(doc: &'a OrgDocument) -> Self {
        FoldingRangeTraverser {
            ranges: vec![],
            doc,
        }
    }
}

#[test]
fn test() {
    let doc = OrgDocument::new("\n* a\n\n* b\n\n");
    let mut t = FoldingRangeTraverser::new(&doc);
    doc.traverse(&mut t);
    assert_eq!(t.ranges[0].start_line, 1);
    assert_eq!(t.ranges[0].end_line, 2);
    assert_eq!(t.ranges[1].start_line, 3);
    assert_eq!(t.ranges[1].end_line, 4);

    let doc = OrgDocument::new("\n\r\n#+begin_src\n#+end_src\n\r\r");
    let mut t = FoldingRangeTraverser::new(&doc);
    doc.traverse(&mut t);
    assert_eq!(t.ranges[0].start_line, 2);
    assert_eq!(t.ranges[0].end_line, 3);
}
