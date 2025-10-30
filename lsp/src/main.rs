use std::collections::HashMap;
use rowan::TextRange;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};
use lsp::document::{DocumentState};
use tokio::sync::mpsc::{unbounded_channel, UnboundedSender};


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
    documents: parking_lot::Mutex<HashMap<Url, DocumentState>>,
}

impl Backend {
    fn info(&self, msg: String) {
        self.logger.info(msg);
    }
}

fn to_lsp_position(doc: &DocumentState, offset: rowan::TextSize) -> Position {
    let lc = doc.line_col(offset);
    Position::new(lc.line as u32, lc.col as u32)
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
       
        let mut docs = self.documents.lock();
        docs.insert(uri, DocumentState::new(text));
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri;

        let mut docs = self.documents.lock();

        if let Some(doc) = docs.get_mut(&uri) {
            if let Some(change) = params.content_changes.into_iter().last() {
                let text = change.text;
                doc.update_text(text);
            }
        }
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        let uri = params.text_document.uri;
        let mut docs = self.documents.lock();
        docs.remove(&uri);
    }

    async fn goto_definition(&self, params: GotoDefinitionParams) -> Result<Option<GotoDefinitionResponse>> {
        let uri = params.text_document_position_params.text_document.uri;
        let position = params.text_document_position_params.position;

        self.info(format!("goto_definition {} {:?}", uri.clone(), position.clone()));

        let docs = self.documents.lock();
        let Some(doc) = docs.get(&uri) else { return Ok(None) };
        
        if let Some(tok) = doc.token_at_line_col(position.line, position.character) {
            self.info(format!("found token of type {:?}", tok.kind()));

            if tok.kind() == lsp::lexer::SyntaxKind::Identifier {
                let name = tok.text();
                if let Some(ranges) = doc.def(name) {

                    if let Some(def_range) = ranges.first() {
                        let lsp_range = to_lsp_range(&doc, *def_range);
                        let loc = Location::new(uri.clone(), lsp_range);
                    
                        self.info(format!("found a definition for {} at {:?}", name, loc));

                        return Ok(Some(GotoDefinitionResponse::Scalar(loc)));
                    }
                }
            }
        } else {
            self.info("no token found".to_string());
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
            documents: parking_lot::Mutex::new(HashMap::new()),
        }
    })
    .finish();

    Server::new(stdin, stdout, socket).serve(service).await;
}