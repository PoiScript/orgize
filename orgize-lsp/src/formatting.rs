use tower_lsp::lsp_types::TextEdit;

use crate::org_document::OrgDocument;

pub fn formatting(doc: &OrgDocument) -> Vec<TextEdit> {
    orgize_common::formatting(&doc.org)
        .into_iter()
        .map(|(start, end, content)| TextEdit {
            range: doc.range_of(start as u32, end as u32),
            new_text: content,
        })
        .collect::<Vec<_>>()
}
