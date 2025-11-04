// SPDX-License-Identifier: Apache-2.0
use line_index::{LineCol, LineIndex};
use lsp::document::DocumentState;
use rowan::TextRange;
use std::collections::HashMap;
use tokio::sync::mpsc::{UnboundedSender, unbounded_channel};
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};

#[derive(Clone)]
struct Logger {
    tx: UnboundedSender<String>,
}

impl Logger {
    fn new(client: Client) -> Self {
        let (tx, mut rx) = unbounded_channel::<String>();
        tokio::spawn(async move {
            while let Some(msg) = rx.recv().await {
                let _ = client.log_message(MessageType::INFO, msg).await;
            }
        });
        Self { tx }
    }

    fn info(&self, msg: impl Into<String>) {
        let _ = self.tx.send(msg.into());
    }
}

struct Backend {
    logger: Logger,
    client: Client,
    documents: parking_lot::Mutex<HashMap<Url, DocumentState>>,
}

impl Backend {
    fn info(&self, msg: String) {
        self.logger.info(msg);
    }
}

fn to_lsp_position(doc: &DocumentState, offset: rowan::TextSize) -> Position {
    let lc = doc.line_col(offset);
    Position::new(lc.line, lc.col)
}

fn to_lsp_range(doc: &DocumentState, range: TextRange) -> Range {
    Range::new(
        to_lsp_position(doc, range.start()),
        to_lsp_position(doc, range.end()),
    )
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        self.info("Initializing Aria LSP".to_string());
        Ok(InitializeResult {
            server_info: None,
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::INCREMENTAL,
                )),
                definition_provider: Some(OneOf::Left(true)),
                ..ServerCapabilities::default()
            },
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.info("Aria LSP initialized".to_string());
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let uri = params.text_document.uri;
        let text = params.text_document.text;

        self.info(format!("opened file {uri}"));

        let (diags, uri_clone) = {
            let mut docs = self.documents.lock();
            let doc = DocumentState::new(text);
            let diags = {
                let mut v = Vec::new();
                for (range, msg) in doc.parse_error_ranges() {
                    v.push(Diagnostic {
                        range: to_lsp_range(&doc, range),
                        severity: Some(DiagnosticSeverity::ERROR),
                        code: None,
                        code_description: None,
                        source: Some("aria-parser".into()),
                        message: msg,
                        related_information: None,
                        tags: None,
                        data: None,
                    });
                }
                v
            };
            docs.insert(uri.clone(), doc);
            (diags, uri.clone())
        };
        let _ = self
            .client
            .publish_diagnostics(uri_clone, diags, None)
            .await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri;
        let (diags_opt, uri_clone) = {
            let mut docs = self.documents.lock();

            if let Some(doc) = docs.get_mut(&uri) {
                let mut text = doc.text();
                let mut index = LineIndex::new(&text);

                for change in params.content_changes {
                    if let Some(range) = change.range {
                        let Some(start_ts) = index.offset(LineCol {
                            line: range.start.line,
                            col: range.start.character,
                        }) else {
                            continue;
                        };
                        let Some(end_ts) = index.offset(LineCol {
                            line: range.end.line,
                            col: range.end.character,
                        }) else {
                            continue;
                        };

                        let start: usize = u32::from(start_ts) as usize;
                        let end: usize = u32::from(end_ts) as usize;

                        if start <= end && end <= text.len() {
                            let mut new_text = String::with_capacity(
                                text.len() - (end - start) + change.text.len(),
                            );
                            new_text.push_str(&text[..start]);
                            new_text.push_str(&change.text);
                            new_text.push_str(&text[end..]);
                            text = new_text;
                            index = LineIndex::new(&text);
                        } else {
                            self.info(format!(
                                "skipping invalid change range for {uri}: {range:?}"
                            ));
                        }
                    } else {
                        // Full text replacement
                        text = change.text;
                        index = LineIndex::new(&text);
                    }
                }
                doc.update_text(text);
                let diags = {
                    let mut v = Vec::new();
                    for (range, msg) in doc.parse_error_ranges() {
                        v.push(Diagnostic {
                            range: to_lsp_range(doc, range),
                            severity: Some(DiagnosticSeverity::ERROR),
                            code: None,
                            code_description: None,
                            source: Some("aria-parser".into()),
                            message: msg,
                            related_information: None,
                            tags: None,
                            data: None,
                        });
                    }
                    v
                };
                (Some(diags), Some(uri.clone()))
            } else {
                (None, None)
            }
        };
        if let (Some(diags), Some(uri2)) = (diags_opt, uri_clone) {
            let _ = self.client.publish_diagnostics(uri2, diags, None).await;
        }
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        let uri = params.text_document.uri;
        let mut docs = self.documents.lock();
        docs.remove(&uri);
    }

    async fn goto_definition(
        &self,
        params: GotoDefinitionParams,
    ) -> Result<Option<GotoDefinitionResponse>> {
        let uri = params.text_document_position_params.text_document.uri;
        let position = params.text_document_position_params.position;

        self.info(format!(
            "goto_definition {} {:?}",
            uri.clone(),
            position.clone()
        ));

        let docs = self.documents.lock();
        let Some(doc) = docs.get(&uri) else {
            return Ok(None);
        };

        if let Some(def_range) = doc.def(position.line, position.character) {
            let lsp_range = to_lsp_range(doc, def_range);
            let loc = Location::new(uri.clone(), lsp_range);
            self.info(format!("found a definition at {loc:?}"));
            return Ok(Some(GotoDefinitionResponse::Scalar(loc)));
        }

        self.info("no definitions found".to_string());

        Ok(None)
    }
}

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::build(|client| {
        let logger = Logger::new(client.clone());
        Backend {
            logger,
            client: client.clone(),
            documents: parking_lot::Mutex::new(HashMap::new()),
        }
    })
    .finish();

    Server::new(stdin, stdout, socket).serve(service).await;
}
