use std::collections::HashMap;
use std::sync::Arc;
use std::sync::OnceLock;

use line_index::LineIndex;
use rowan::TextRange;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};

use lsp::parser::{self, Parse, SyntaxNode, SyntaxToken};

#[derive(Clone)]
struct DocumentState {
    text: Arc<String>,
    line_index: Arc<LineIndex>,
    parse: Arc<Parse>,
    defs: HashMap<String, Vec<TextRange>>,
}

impl DocumentState {
    fn new(text: String) -> Self {
        let line_index = LineIndex::new(&text);
        let parse = parser::parse(&text);
        let syntax = parse.syntax();
        let defs = build_index(&syntax);
        Self {
            text: Arc::new(text),
            parse: Arc::new(parse),
            line_index: Arc::new(line_index),
            defs,
        }
    }

    fn update_text(&mut self, text: String) {
        self.text = Arc::new(text);
        self.line_index = Arc::new(LineIndex::new(&self.text));
        let parse = parser::parse(&self.text);
        let syntax = parse.syntax();

        self.parse = Arc::new(parse);
        self.defs = build_index(&syntax);
    }

    fn token_at_offset(&self, offset: rowan::TextSize) -> Option<SyntaxToken> {
        for el in self.parse.syntax().descendants_with_tokens() {
            if let rowan::NodeOrToken::Token(tok) = el {
                if tok.text_range().contains(offset) {
                    return Some(tok);
                }
            }
        }
        None
    }
}

static LOGGER_CLIENT: OnceLock<Client> = OnceLock::new();

fn init_logger(client: Client) {
    let _ = LOGGER_CLIENT.set(client);
}

pub async fn info(msg: String) {
    if let Some(client) = LOGGER_CLIENT.get() {
        let _ = client.log_message(MessageType::INFO, msg).await;
    }
}

struct Backend {
    client: Client,
    documents: parking_lot::Mutex<HashMap<Url, DocumentState>>,
}

fn build_index(root: &SyntaxNode) -> HashMap<String, Vec<TextRange>> {
    use lsp::lexer::SyntaxKind as K;
    let mut defs: HashMap<String, Vec<TextRange>> = HashMap::new();

    for node in root.descendants() {
        match node.kind() {
            K::StmtVal | K::Param | K::Func => {
                // First Identifier token child is the declared name
                if let Some(tok) = node
                    .children_with_tokens()
                    .filter_map(|e| e.into_token())
                    .find(|t| t.kind() == K::Identifier)
                {
                    let name = tok.text().to_string();
                    defs.entry(name).or_default().push(tok.text_range());
                }
            }
            _ => {}
        }
    }

    defs
}

fn to_lsp_position(li: &LineIndex, offset: rowan::TextSize) -> Position {
    let lc = li.line_col(offset);
    Position::new(lc.line as u32, lc.col as u32)
}

fn to_lsp_range(li: &LineIndex, range: TextRange) -> Range {
    Range::new(
        to_lsp_position(li, range.start()),
        to_lsp_position(li, range.end()),
    )
}

impl Backend {
    async fn info(&self, msg: String) {
        info(msg).await;
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        self.info("Initializing Aria LSP".to_string()).await;
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
        self.info("Aria LSP initialized".to_string()).await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let uri = params.text_document.uri;
        let text = params.text_document.text;
       
        self.info(format!("opened file {uri}")).await;
       
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

        self.info(format!("goto_definition {} {:?}", uri.clone(), position.clone())).await;

        {
            let docs = self.documents.lock();
            let Some(doc) = docs.get(&uri) else { return Ok(None) };

            let line_col = line_index::LineCol { line: position.line as u32, col: position.character as u32 };
            let Some(offset) = doc.line_index.offset(line_col) else { return Ok(None) };

            if let Some(tok) = doc.token_at_offset(offset) {
                if tok.kind() == lsp::lexer::SyntaxKind::Identifier {
                    let name = tok.text();
                    if let Some(ranges) = doc.defs.get(name) {

                        if let Some(def_range) = ranges.first() {
                            let lsp_range = to_lsp_range(&doc.line_index, *def_range);
                            let loc = Location::new(uri.clone(), lsp_range);
                        
                            
                            return Ok(Some(GotoDefinitionResponse::Scalar(loc)));
                        }
                    }
                }
            }
        }

        self.info("no definitions found".to_string()).await;

        Ok(None)
    }
}

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::build(|client| {
        init_logger(client.clone());
        Backend {
            client,
            documents: parking_lot::Mutex::new(HashMap::new()),
        }
    })
    .finish();

    Server::new(stdin, stdout, socket).serve(service).await;
}