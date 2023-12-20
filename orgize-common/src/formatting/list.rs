use std::iter::once;

use orgize::{ast::ListItem, rowan::ast::AstNode, SyntaxNode};

pub fn format(node: &SyntaxNode, indent_level: usize, edits: &mut Vec<(usize, usize, String)>) {
    let mut items = node.children().filter_map(ListItem::cast);

    let Some(first_item) = items.next() else {
        return;
    };

    match first_item.bullet().trim_end() {
        expected_bullet @ ("-" | "+" | "*") => {
            if first_item.indent() != 3 * indent_level {
                edits.push((
                    first_item.begin() as usize,
                    first_item.begin() as usize + first_item.indent(),
                    " ".repeat(3 * indent_level),
                ));
            }

            for item in items {
                if item.indent() != 3 * indent_level {
                    edits.push((
                        item.begin() as usize,
                        item.begin() as usize + item.indent(),
                        " ".repeat(3 * indent_level),
                    ));
                }

                let bullet = item.bullet();
                let s = bullet.trim_end();
                if s != expected_bullet {
                    edits.push((
                        bullet.start() as usize,
                        bullet.start() as usize + s.len(),
                        expected_bullet.to_string(),
                    ));
                }
            }
        }
        b => {
            let c = if b.ends_with(')') { ')' } else { '.' };

            for (index, item) in once(first_item).chain(items).enumerate() {
                if item.indent() != 3 * indent_level {
                    edits.push((
                        item.begin() as usize,
                        item.begin() as usize + item.indent(),
                        " ".repeat(3 * indent_level),
                    ));
                }

                let expected_bullet = format!("{}{c}", index + 1);
                let bullet = item.bullet();
                let s = bullet.trim_end();
                if s != expected_bullet {
                    edits.push((
                        bullet.start() as usize,
                        bullet.start() as usize + s.len(),
                        expected_bullet,
                    ));
                }
            }
        }
    }
}

#[test]
fn test() {
    use crate::test_case;
    use orgize::ast::List;

    let format0 =
        |node: &SyntaxNode, edits: &mut Vec<(usize, usize, String)>| format(node, 0, edits);

    let format2 =
        |node: &SyntaxNode, edits: &mut Vec<(usize, usize, String)>| format(node, 2, edits);

    test_case!(List, "1.    item", format0, "1.    item");

    test_case!(
        List,
        "0. item\n- item\n+ item",
        format0,
        "1. item\n2. item\n3. item"
    );

    test_case!(
        List,
        " + item\n - item\n 1. item",
        format0,
        "+ item\n+ item\n+ item"
    );

    test_case!(
        List,
        " + item\n - item\n 1. item",
        format2,
        "      + item\n      + item\n      + item"
    );
}
