use orgize::{
    export::{Container, Event, TraversalContext, Traverser},
    rowan::ast::AstNode,
    Org,
};

mod blank_lines;
mod list;
mod rule;

pub fn formatting(org: &Org) -> Vec<(usize, usize, String)> {
    let mut format = FormattingTraverser::default();

    org.traverse(&mut format);

    format.edits
}

#[derive(Default)]
struct FormattingTraverser {
    indent_level: usize,
    edits: Vec<(usize, usize, String)>,
}

impl Traverser for FormattingTraverser {
    fn event(&mut self, event: Event, _: &mut TraversalContext) {
        match event {
            Event::Rule(rule) => {
                rule::format(rule.syntax(), &mut self.edits);
                blank_lines::format(rule.syntax(), &mut self.edits);
            }
            Event::Clock(clock) => {
                blank_lines::format(clock.syntax(), &mut self.edits);
            }

            Event::Enter(Container::Document(document)) => {
                blank_lines::format(document.syntax(), &mut self.edits);
            }
            Event::Enter(Container::Paragraph(paragraph)) => {
                blank_lines::format(paragraph.syntax(), &mut self.edits);
            }
            Event::Enter(Container::List(list)) => {
                list::format(list.syntax(), self.indent_level, &mut self.edits);
                blank_lines::format(list.syntax(), &mut self.edits);
                self.indent_level += 1;
            }
            Event::Leave(Container::List(_)) => {
                self.indent_level -= 1;
            }
            Event::Enter(Container::OrgTable(table)) => {
                blank_lines::format(table.syntax(), &mut self.edits);
            }
            Event::Enter(Container::SpecialBlock(block)) => {
                blank_lines::format(block.syntax(), &mut self.edits);
            }
            Event::Enter(Container::QuoteBlock(block)) => {
                blank_lines::format(block.syntax(), &mut self.edits);
            }
            Event::Enter(Container::CenterBlock(block)) => {
                blank_lines::format(block.syntax(), &mut self.edits);
            }
            Event::Enter(Container::VerseBlock(block)) => {
                blank_lines::format(block.syntax(), &mut self.edits);
            }
            Event::Enter(Container::CommentBlock(block)) => {
                blank_lines::format(block.syntax(), &mut self.edits);
            }
            Event::Enter(Container::ExampleBlock(block)) => {
                blank_lines::format(block.syntax(), &mut self.edits);
            }
            Event::Enter(Container::ExportBlock(block)) => {
                blank_lines::format(block.syntax(), &mut self.edits);
            }
            Event::Enter(Container::SourceBlock(block)) => {
                blank_lines::format(block.syntax(), &mut self.edits);
            }

            _ => {}
        }
    }
}

#[cfg(test)]
#[macro_export]
macro_rules! test_case {
    (
        $n:tt,
        $input:expr,
        $fn:expr,
        $expected:expr
    ) => {{
        use orgize::rowan::ast::AstNode;

        let org = orgize::Org::parse($input);
        let node = org.first_node::<$n>().unwrap();
        let node = node.syntax();

        let mut patches = vec![];

        $fn(&node, &mut patches);

        let input = node.to_string();

        patches.sort_by(|a, b| a.0.cmp(&b.0));

        let mut i = 0;
        let mut output = String::new();
        for (start, end, text) in patches {
            output.push_str(&input[i..start]);
            output.push_str(&text);
            i = end;
        }
        output.push_str(&input[i..]);

        assert_eq!(output, $expected);
    }};
}
