use std::collections::HashMap;

use orgize::{ast::SourceBlock, rowan::ast::AstNode};
use tower_lsp::lsp_types::{MessageType, TextEdit, Url, WorkspaceEdit};

use crate::Backend;

impl Backend {
    pub async fn src_block_execute(&self, url: Url, block_offset: u32) -> anyhow::Result<()> {
        let uri = url.to_string();

        let Some(doc) = self.documents.get(&uri) else {
            return Ok(());
        };

        let Some(block) = doc
            .org
            .document()
            .syntax()
            .descendants()
            .filter_map(SourceBlock::cast)
            .find(|n| n.begin() == block_offset)
        else {
            return Ok(());
        };

        let dir = tempfile::tempdir().unwrap();

        if let Some((start, end, new_text)) = orgize_common::execute(block, dir.path())? {
            let mut changes = HashMap::new();

            let range = doc.range_of(start as u32, end as u32);

            changes.insert(url, vec![TextEdit { new_text, range }]);

            let _ = self
                .client
                .apply_edit(WorkspaceEdit {
                    changes: Some(changes),
                    ..Default::default()
                })
                .await;
        } else {
            self.client
                .show_message(MessageType::WARNING, "Code block can't be executed.")
                .await;
        }

        Ok(())
    }
}
