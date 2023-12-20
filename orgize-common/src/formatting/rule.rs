use orgize::{SyntaxKind, SyntaxNode};

pub fn format(node: &SyntaxNode, edits: &mut Vec<(usize, usize, String)>) {
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

#[test]
fn test() {
    use crate::test_case;
    use orgize::ast::Rule;

    test_case!(Rule, "    ------------\r\n", format, "-----\n");
}
