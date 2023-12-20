use orgize::{ast::SourceBlock, rowan::ast::AstNode};
use std::fs;
use tower_lsp::lsp_types::{MessageType, Url};

use crate::Backend;

impl Backend {
    pub async fn src_block_tangle(&self, url: Url, block_offset: u32) -> anyhow::Result<()> {
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

        let Ok(file_path) = url.to_file_path() else {
            return Ok(());
        };

        if let Some((dest, _permission, contents, _mkdir)) =
            orgize_common::tangle(block, &file_path)?
        {
            fs::write(&dest, contents)?;

            self.client
                .show_message(
                    MessageType::INFO,
                    format!("Wrote to {}", dest.to_string_lossy()),
                )
                .await;
        } else {
            self.client
                .show_message(MessageType::WARNING, "Code block can't be tangled.")
                .await;
        }

        Ok(())
    }
}
