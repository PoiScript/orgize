mod code_lens;
mod commands;
mod completion;
mod document_link;
mod document_link_resolve;
mod document_symbol;
mod folding_range;
mod formatting;
mod org_document;
mod semantic_token;

use dashmap::DashMap;
use document_symbol::DocumentSymbolTraverser;
use org_document::OrgDocument;
use serde_json::Value;
use tower_lsp::lsp_types::*;
use tower_lsp::{jsonrpc::Result, Client, LanguageServer, LspService, Server};

pub use self::code_lens::*;
pub use self::commands::*;
pub use self::document_link::*;
pub use self::folding_range::*;
pub use self::semantic_token::*;

pub struct Backend {
    client: Client,

    documents: DashMap<String, OrgDocument>,
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            server_info: None,
            offset_encoding: None,
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::INCREMENTAL,
                )),
                execute_command_provider: Some(ExecuteCommandOptions {
                    commands: OrgizeCommand::all(),
                    work_done_progress_options: Default::default(),
                }),
                workspace: Some(WorkspaceServerCapabilities {
                    workspace_folders: Some(WorkspaceFoldersServerCapabilities {
                        supported: Some(true),
                        change_notifications: Some(OneOf::Left(true)),
                    }),
                    file_operations: None,
                }),
                semantic_tokens_provider: Some(
                    SemanticTokensServerCapabilities::SemanticTokensRegistrationOptions(
                        SemanticTokensRegistrationOptions {
                            text_document_registration_options: {
                                TextDocumentRegistrationOptions {
                                    document_selector: Some(vec![DocumentFilter {
                                        language: Some("org".to_string()),
                                        scheme: Some("file".to_string()),
                                        pattern: None,
                                    }]),
                                }
                            },
                            semantic_tokens_options: SemanticTokensOptions {
                                work_done_progress_options: WorkDoneProgressOptions::default(),
                                legend: SemanticTokensLegend {
                                    token_types: LEGEND_TYPE.into(),
                                    token_modifiers: vec![],
                                },
                                range: Some(true),
                                full: Some(SemanticTokensFullOptions::Bool(true)),
                            },
                            static_registration_options: StaticRegistrationOptions::default(),
                        },
                    ),
                ),
                code_lens_provider: Some(CodeLensOptions {
                    resolve_provider: Some(true),
                }),
                folding_range_provider: Some(FoldingRangeProviderCapability::Simple(true)),
                document_link_provider: Some(DocumentLinkOptions {
                    resolve_provider: Some(true),
                    work_done_progress_options: WorkDoneProgressOptions::default(),
                }),
                document_formatting_provider: Some(OneOf::Left(true)),
                document_symbol_provider: Some(OneOf::Left(true)),
                completion_provider: Some(CompletionOptions {
                    resolve_provider: Some(false),
                    trigger_characters: Some(completion::trigger_characters()),
                    ..Default::default()
                }),
                ..ServerCapabilities::default()
            },
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "Orgize LSP initialized")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        self.client
            .log_message(MessageType::INFO, "Orgize LSP shutdown")
            .await;
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let url = params.text_document.uri.to_string();

        self.documents
            .insert(url.clone(), OrgDocument::new(params.text_document.text));
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let url = params.text_document.uri.to_string();

        for change in params.content_changes {
            if let (Some(mut doc), Some(range)) = (self.documents.get_mut(&url), change.range) {
                let start = doc.offset_of(range.start);
                let end = doc.offset_of(range.end);
                doc.update(start, end, &change.text);
            } else {
                self.documents
                    .insert(url.clone(), OrgDocument::new(change.text));
            }
        }
    }

    async fn did_save(&self, _: DidSaveTextDocumentParams) {}

    async fn did_close(&self, _: DidCloseTextDocumentParams) {}

    async fn did_change_configuration(&self, _: DidChangeConfigurationParams) {}

    async fn did_change_workspace_folders(&self, _: DidChangeWorkspaceFoldersParams) {}

    async fn did_change_watched_files(&self, _: DidChangeWatchedFilesParams) {}

    async fn completion(&self, params: CompletionParams) -> Result<Option<CompletionResponse>> {
        Ok(completion::completion(params, &self))
    }

    async fn completion_resolve(&self, params: CompletionItem) -> Result<CompletionItem> {
        Ok(params)
    }

    async fn semantic_tokens_full(
        &self,
        params: SemanticTokensParams,
    ) -> Result<Option<SemanticTokensResult>> {
        let uri = params.text_document.uri.to_string();

        let Some(doc) = self.documents.get(&uri) else {
            return Ok(None);
        };

        let mut traverser = SemanticTokenTraverser::new(&doc);

        doc.traverse(&mut traverser);

        Ok(Some(SemanticTokensResult::Tokens(SemanticTokens {
            result_id: None,
            data: traverser.tokens,
        })))
    }

    async fn semantic_tokens_range(
        &self,
        params: SemanticTokensRangeParams,
    ) -> Result<Option<SemanticTokensRangeResult>> {
        let uri = params.text_document.uri.to_string();

        let Some(doc) = self.documents.get(&uri) else {
            return Ok(None);
        };

        let mut traverser = SemanticTokenTraverser::with_range(&doc, params.range);

        doc.traverse(&mut traverser);

        Ok(Some(SemanticTokensRangeResult::Partial(
            SemanticTokensPartialResult {
                data: traverser.tokens,
            },
        )))
    }

    async fn document_link(&self, params: DocumentLinkParams) -> Result<Option<Vec<DocumentLink>>> {
        let uri = params.text_document.uri.to_string();

        let Some(doc) = self.documents.get(&uri) else {
            return Ok(None);
        };

        let mut traverser =
            DocumentLinkTraverser::new(&doc, params.text_document.uri.to_file_path().ok());

        doc.traverse(&mut traverser);

        Ok(Some(traverser.links))
    }

    async fn document_link_resolve(&self, params: DocumentLink) -> Result<DocumentLink> {
        if let Some(link) = document_link_resolve::document_link_resolve(&params, self) {
            Ok(link)
        } else {
            Ok(params)
        }
    }

    async fn folding_range(&self, params: FoldingRangeParams) -> Result<Option<Vec<FoldingRange>>> {
        let uri = params.text_document.uri.to_string();

        let Some(doc) = self.documents.get(&uri) else {
            return Ok(None);
        };

        let mut traverser = FoldingRangeTraverser::new(&doc);

        doc.traverse(&mut traverser);

        Ok(Some(traverser.ranges))
    }

    async fn code_lens(&self, params: CodeLensParams) -> Result<Option<Vec<CodeLens>>> {
        let uri = params.text_document.uri.to_string();

        let Some(doc) = self.documents.get(&uri) else {
            return Ok(None);
        };

        let mut traverser = CodeLensTraverser::new(params.text_document.uri, &doc);

        doc.traverse(&mut traverser);

        Ok(Some(traverser.lens))
    }

    async fn code_lens_resolve(&self, params: CodeLens) -> Result<CodeLens> {
        Ok(params)
    }

    async fn code_action(&self, _: CodeActionParams) -> Result<Option<CodeActionResponse>> {
        Ok(None)
    }

    async fn formatting(&self, params: DocumentFormattingParams) -> Result<Option<Vec<TextEdit>>> {
        let uri = params.text_document.uri.to_string();

        let Some(doc) = self.documents.get(&uri) else {
            return Ok(None);
        };

        let edits = formatting::formatting(&doc);
        Ok(Some(edits))
    }

    async fn execute_command(&self, params: ExecuteCommandParams) -> Result<Option<Value>> {
        let value = commands::execute(&params, self).await;
        Ok(value)
    }

    async fn document_symbol(
        &self,
        params: DocumentSymbolParams,
    ) -> Result<Option<DocumentSymbolResponse>> {
        let uri = params.text_document.uri.to_string();

        let Some(doc) = self.documents.get(&uri) else {
            return Ok(None);
        };

        let mut t = DocumentSymbolTraverser::new(&doc);
        doc.traverse(&mut t);
        Ok(Some(DocumentSymbolResponse::Nested(t.symbols)))
    }
}

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::build(|client| Backend {
        client,
        documents: DashMap::new(),
    })
    .finish();

    Server::new(stdin, stdout, socket).serve(service).await;
}
