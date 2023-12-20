use tower_lsp::lsp_types::{
    CompletionItem, CompletionItemKind, CompletionParams, CompletionResponse, CompletionTextEdit,
    InsertTextFormat, Position, Range, TextEdit,
};

use crate::Backend;

pub fn completion(params: CompletionParams, backend: &Backend) -> Option<CompletionResponse> {
    let uri = params.text_document_position.text_document.uri.to_string();

    let Some(doc) = backend.documents.get(&uri) else {
        return None;
    };

    let offset = doc.offset_of(params.text_document_position.position) as usize;

    if offset < 2 {
        return None;
    }

    let filter_text = doc.text.get((offset - 2)..offset)?;

    let (label, new_text) = match filter_text {
        "<a" => (
            "ASCI export block",
            "#+BEGIN_EXPORT ascii\n${0}\n#+END_EXPORT\n",
        ),
        "<c" => ("Center block", "#+BEGIN_CENTER\n${0}\n#+END_CENTER\n"),
        "<C" => ("Comment block", "#+BEGIN_COMMENT\n${0}\n#+END_COMMENT\n"),
        "<e" => ("Example block", "#+BEGIN_EXAMPLE\n${0}\n#+END_EXAMPLE\n"),
        "<E" => ("Export block", "#+BEGIN_EXPORT\n${0}\n#+END_EXPORT\n"),
        "<h" => (
            "HTML export block",
            "#+BEGIN_EXPORT html\n${0}\n#+END_EXPORT\n",
        ),
        "<l" => (
            "LaTeX export block",
            "#+BEGIN_EXPORT latex\n${0}\n#+END_EXPORT\n",
        ),
        "<q" => ("Quote block", "#+BEGIN_QUOTE\n${0}\n#+END_QUOTE\n"),
        "<s" => ("Source block", "#+BEGIN_SRC ${1}\n${0}\n#+END_SRC\n"),
        "<v" => ("Verse block", "#+BEGIN_VERSE\n${0}\n#+END_VERSE\n"),
        _ => return None,
    };

    let end = params.text_document_position.position;

    Some(CompletionResponse::Array(vec![CompletionItem {
        label: label.into(),
        kind: Some(CompletionItemKind::SNIPPET),
        insert_text: Some(new_text.into()),
        insert_text_format: Some(InsertTextFormat::SNIPPET),
        filter_text: Some(filter_text.into()),
        text_edit: Some(CompletionTextEdit::Edit(TextEdit {
            new_text: new_text.into(),
            range: Range {
                start: Position::new(end.line, end.character - 2),
                end,
            },
        })),
        ..Default::default()
    }]))
}

pub fn trigger_characters() -> Vec<String> {
    vec![
        "<a".into(),
        "<c".into(),
        "<C".into(),
        "<e".into(),
        "<E".into(),
        "<h".into(),
        "<l".into(),
        "<q".into(),
        "<s".into(),
        "<v".into(),
        "<I".into(),
    ]
}
