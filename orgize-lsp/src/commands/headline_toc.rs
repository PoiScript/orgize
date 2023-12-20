use orgize::{
    export::{Container, Event, TraversalContext, Traverser},
    rowan::ast::AstNode,
    SyntaxKind,
};
use std::collections::HashMap;
use std::fmt::Write;
use tower_lsp::lsp_types::{TextEdit, Url, WorkspaceEdit};

use crate::Backend;

impl Backend {
    pub async fn headline_toc(&self, url: Url, headline_offset: u32) {
        let uri = url.to_string();

        let Some(doc) = self.documents.get(&uri) else {
            return;
        };

        let mut toc = Toc {
            indent: 0,
            output: String::new(),

            headline_offset,
            edit_range: None,
        };

        doc.traverse(&mut toc);

        if let Some((start, end)) = toc.edit_range {
            let mut changes = HashMap::new();

            let range = doc.range_of(start, end);

            changes.insert(
                url,
                vec![TextEdit {
                    new_text: toc.output,
                    range,
                }],
            );

            let _ = self
                .client
                .apply_edit(WorkspaceEdit {
                    changes: Some(changes),
                    ..Default::default()
                })
                .await;
        }
    }
}

pub struct Toc {
    output: String,
    indent: usize,

    headline_offset: u32,

    edit_range: Option<(u32, u32)>,
}

impl Traverser for Toc {
    fn event(&mut self, event: Event, ctx: &mut TraversalContext) {
        match event {
            Event::Enter(Container::Headline(headline)) => {
                if headline.begin() == self.headline_offset {
                    let start = headline
                        .syntax()
                        .children_with_tokens()
                        .find(|n| n.kind() == SyntaxKind::NEW_LINE)
                        .map(|n| n.text_range().end().into())
                        .unwrap_or(headline.end());

                    let end = headline.end();

                    self.edit_range = Some((start, end));
                } else {
                    let title = headline.title().map(|e| e.to_string()).collect::<String>();

                    let slug = orgize_common::headline_slug(&headline);

                    let _ = writeln!(
                        &mut self.output,
                        "{: >i$}- [[#{slug}][{title}]]",
                        "",
                        i = self.indent
                    );
                }

                self.indent += 2;
            }
            Event::Leave(Container::Headline(_)) => self.indent -= 2,

            Event::Enter(Container::Section(_)) => ctx.skip(),
            Event::Enter(Container::Document(_)) => self.output += "#+begin_quote\n",
            Event::Leave(Container::Document(_)) => self.output += "#+end_quote\n\n",
            _ => {}
        }
    }
}
