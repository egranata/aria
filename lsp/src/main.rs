use std::collections::HashMap;
use std::sync::Arc;

use line_index::LineIndex;
use rowan::TextRange;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};

use lsp::parser::{self, SyntaxNode, SyntaxToken};

#[derive(Clone)]
struct DocumentState {
    text: Arc<String>,
    line_index: Arc<LineIndex>,
    // map from identifier to list of definition ranges in this file
    defs: HashMap<String, Vec<TextRange>>,
}

impl DocumentState {
    fn new(text: String) -> Self {
        let line_index = Arc::new(LineIndex::new(&text));
        let parse = parser::parse(&text);
        let syntax = parse.syntax();
        let defs = build_index(&syntax);
        Self {
            text: Arc::new(text),
            line_index,
            defs,
        }
    }

    fn update_text(&mut self, text: String) {
        self.text = Arc::new(text);
        self.line_index = Arc::new(LineIndex::new(&self.text));
        let parse = parser::parse(&self.text);
        let syntax = parse.syntax();
        self.defs = build_index(&syntax);
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

fn token_at_offset(root: &SyntaxNode, offset: rowan::TextSize) -> Option<SyntaxToken> {
    for el in root.descendants_with_tokens() {
        if let rowan::NodeOrToken::Token(tok) = el {
            if tok.text_range().contains(offset) {
                return Some(tok);
            }
        }
    }
    None
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

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
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
        let _ = self.client.log_message(MessageType::INFO, "Aria LSP initialized").await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let uri = params.text_document.uri;
        let text = params.text_document.text;
        let mut docs = self.documents.lock();
        docs.insert(uri, DocumentState::new(text));
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri;
        let mut docs = self.documents.lock();
        if let Some(doc) = docs.get_mut(&uri) {
            // For simplicity, rebuild from full text if provided, otherwise apply incremental naive
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

        let docs = self.documents.lock();
        let Some(doc) = docs.get(&uri) else { return Ok(None) };
        // Parse a fresh syntax tree for current text
        let tree = parser::parse(&doc.text).syntax();

        // Convert LSP position to byte offset using LineIndex
        let line_col = line_index::LineCol { line: position.line as u32, col: position.character as u32 };
        let offset = doc.line_index.offset(line_col);

        // Find identifier token under cursor
        if let Some(tok) = token_at_offset(&tree, offset.unwrap()) {
            if tok.kind() == lsp::lexer::SyntaxKind::Identifier {
                let name = tok.text().to_string();
                if let Some(ranges) = doc.defs.get(&name) {
                    // Prefer the first definition for now
                    if let Some(def_range) = ranges.first() {
                        let lsp_range = to_lsp_range(&doc.line_index, *def_range);
                        let loc = Location::new(uri.clone(), lsp_range);
                        return Ok(Some(GotoDefinitionResponse::Scalar(loc)));
                    }
                }
            }
        }

        Ok(None)
    }
}

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::build(|client| Backend {
        client,
        documents: parking_lot::Mutex::new(HashMap::new()),
    })
    .finish();

    Server::new(stdin, stdout, socket).serve(service).await;
}