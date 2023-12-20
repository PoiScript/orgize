mod headline_toc;
mod src_block_detangle;
mod src_block_execute;
mod src_block_tangle;

use orgize::rowan::ast::AstNode;
use serde_json::{json, Value};
use tower_lsp::lsp_types::{Command, ExecuteCommandParams, MessageType, Url};

use crate::Backend;

pub enum OrgizeCommand {
    SrcBlockExecute { url: Url, block_offset: u32 },

    SrcBlockTangle { url: Url, block_offset: u32 },

    SrcBlockDetangle { url: Url, block_offset: u32 },

    HeadlineToc { url: Url, heading_offset: u32 },
}

impl From<OrgizeCommand> for Command {
    fn from(val: OrgizeCommand) -> Self {
        match val {
            OrgizeCommand::SrcBlockExecute { url, block_offset } => Command {
                command: "orgize.src-block.execute".into(),
                arguments: Some(vec![json!(url), json!(block_offset)]),
                title: "Execute".into(),
            },
            OrgizeCommand::SrcBlockTangle { url, block_offset } => Command {
                command: "orgize.src-block.tangle".into(),
                arguments: Some(vec![json!(url), json!(block_offset)]),
                title: "Tangle".into(),
            },
            OrgizeCommand::SrcBlockDetangle { url, block_offset } => Command {
                command: "orgize.src-block.detangle".into(),
                arguments: Some(vec![json!(url), json!(block_offset)]),
                title: "Detangle".into(),
            },
            OrgizeCommand::HeadlineToc {
                url,
                heading_offset,
            } => Command {
                command: "orgize.headline.toc".into(),
                arguments: Some(vec![json!(url), json!(heading_offset)]),
                title: "Generate TOC".into(),
            },
        }
    }
}

impl OrgizeCommand {
    pub fn all() -> Vec<String> {
        vec![
            "orgize.src-block.execute".into(),
            "orgize.src-block.tangle".into(),
            "orgize.src-block.detangle".into(),
            "orgize.src-block.open-tangle-dest".into(),
            "orgize.headline.toc".into(),
        ]
    }
}

pub async fn execute(params: &ExecuteCommandParams, backend: &Backend) -> Option<Value> {
    let result = match (
        params.command.as_str(),
        params.arguments.get(0).and_then(|x| x.as_str()),
        params.arguments.get(1).and_then(|x| x.as_u64()),
    ) {
        ("orgize.src-block.execute", Some(s), Some(n)) => backend
            .src_block_execute(s.parse().ok()?, n as u32)
            .await
            .map(|_| None),
        ("orgize.src-block.tangle", Some(s), Some(n)) => backend
            .src_block_tangle(s.parse().ok()?, n as u32)
            .await
            .map(|_| None),
        ("orgize.src-block.detangle", Some(s), Some(n)) => backend
            .src_block_detangle(s.parse().ok()?, n as u32)
            .await
            .map(|_| None),
        ("orgize.headline.toc", Some(s), Some(n)) => {
            backend.headline_toc(s.parse().ok()?, n as u32).await;
            Ok(None)
        }
        ("orgize.syntax-tree", Some(s), _) => {
            if let Some(doc) = backend.documents.get(s) {
                Ok(Some(json!(format!("{:#?}", doc.org.document().syntax()))))
            } else {
                Ok(None)
            }
        }
        ("orgize.preview-html", Some(s), _) => {
            if let Some(doc) = backend.documents.get(s) {
                Ok(Some(json!(format!("{}", doc.org.to_html()))))
            } else {
                Ok(None)
            }
        }
        _ => Ok(None),
    };

    match result {
        Ok(value) => value,
        Err(err) => {
            backend
                .client
                .show_message(
                    MessageType::ERROR,
                    format!("Failed to execute {:?}: {}", params.command, err),
                )
                .await;
            None
        }
    }
}
