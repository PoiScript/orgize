use orgize::{
    ast::Rule,
    export::{Container, Event, TraversalContext, Traverser},
    rowan::ast::AstNode,
    Org, SyntaxKind, SyntaxNode,
};

pub fn formatting(org: &Org) -> Vec<(usize, usize, String)> {
    let mut format = FormattingTraverser { edits: vec![] };

    org.traverse(&mut format);

    format.edits
}

struct FormattingTraverser {
    edits: Vec<(usize, usize, String)>,
}

impl Traverser for FormattingTraverser {
    fn event(&mut self, event: Event, _: &mut TraversalContext) {
        match event {
            Event::Rule(rule) => {
                format_rule(&rule, &mut self.edits);
                format_blank_lines(rule.syntax(), &mut self.edits);
            }
            Event::Clock(clock) => {
                format_blank_lines(clock.syntax(), &mut self.edits);
            }

            Event::Enter(Container::Document(document)) => {
                format_blank_lines(document.syntax(), &mut self.edits);
            }
            Event::Enter(Container::Paragraph(paragraph)) => {
                format_blank_lines(paragraph.syntax(), &mut self.edits);
            }
            Event::Enter(Container::List(list)) => {
                format_blank_lines(list.syntax(), &mut self.edits);
            }
            Event::Enter(Container::OrgTable(table)) => {
                format_blank_lines(table.syntax(), &mut self.edits);
            }
            Event::Enter(Container::SpecialBlock(block)) => {
                format_blank_lines(block.syntax(), &mut self.edits);
            }
            Event::Enter(Container::QuoteBlock(block)) => {
                format_blank_lines(block.syntax(), &mut self.edits);
            }
            Event::Enter(Container::CenterBlock(block)) => {
                format_blank_lines(block.syntax(), &mut self.edits);
            }
            Event::Enter(Container::VerseBlock(block)) => {
                format_blank_lines(block.syntax(), &mut self.edits);
            }
            Event::Enter(Container::CommentBlock(block)) => {
                format_blank_lines(block.syntax(), &mut self.edits);
            }
            Event::Enter(Container::ExampleBlock(block)) => {
                format_blank_lines(block.syntax(), &mut self.edits);
            }
            Event::Enter(Container::ExportBlock(block)) => {
                format_blank_lines(block.syntax(), &mut self.edits);
            }

            _ => {}
        }
    }
}

fn format_rule(rule: &Rule, edits: &mut Vec<(usize, usize, String)>) {
    let node = rule.syntax();

    for token in node.children_with_tokens().filter_map(|e| e.into_token()) {
        if token.kind() == SyntaxKind::WHITESPACE && !token.text().is_empty() {
            edits.push((
                token.text_range().start().into(),
                token.text_range().end().into(),
                "".into(),
            ));
        }

        if token.kind() == SyntaxKind::TEXT && token.text().len() != 5 {
            edits.push((
                token.text_range().start().into(),
                token.text_range().end().into(),
                "-----".into(),
            ));
        }

        if token.kind() == SyntaxKind::NEW_LINE && token.text() != "\n" {
            edits.push((
                token.text_range().start().into(),
                token.text_range().end().into(),
                "\n".into(),
            ));
        }
    }
}

fn format_blank_lines(node: &SyntaxNode, edits: &mut Vec<(usize, usize, String)>) {
    let mut blank_lines = node
        .children_with_tokens()
        .filter_map(|e| e.into_token())
        .filter(|n| n.kind() == SyntaxKind::BLANK_LINE);

    if let Some(line) = blank_lines.next() {
        if line.text() != "\n" {
            edits.push((
                line.text_range().start().into(),
                line.text_range().end().into(),
                "\n".into(),
            ));
        }
    }

    match (blank_lines.next(), blank_lines.last()) {
        (Some(first), Some(last)) => {
            edits.push((
                first.text_range().start().into(),
                last.text_range().end().into(),
                "".into(),
            ));
        }
        (Some(first), None) => {
            edits.push((
                first.text_range().start().into(),
                first.text_range().end().into(),
                "".into(),
            ));
        }
        _ => {}
    }
}
