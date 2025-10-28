use std::collections::HashMap;
use std::sync::Arc;

use line_index::LineIndex;
use rowan::TextRange;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};

use lsp::parser::{self, SyntaxNode, SyntaxToken};
use lsp::lexer;

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

// Find token at original source byte offset using the lexer spans
fn lex_token_at_offset(source: &str, offset: usize) -> Option<(lexer::SyntaxKind, logos::Span, String)> {
    let tokens = lexer::lex(source);
    for t in tokens {
        if let Ok((kind, slice, span)) = t {
            if (span.start..span.end).contains(&offset) || span.start == offset {
                return Some((kind, span, slice.to_string()));
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

impl Backend {
    async fn info(&self, msg: String) {
        let _ = self.client.log_message(MessageType::INFO, msg).await;
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

        self.info(format!("file changed {uri}")).await;

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

        let docs = self.documents.lock();
        let Some(doc) = docs.get(&uri) else { return Ok(None) };

        let line_col = line_index::LineCol { line: position.line as u32, col: position.character as u32 };
        let Some(offset) = doc.line_index.offset(line_col) else { return Ok(None) };
        let byte_off: usize = u32::from(offset) as usize;

        if let Some((kind, _span, text)) = lex_token_at_offset(&doc.text, byte_off) {
            if kind == lsp::lexer::SyntaxKind::Identifier {
                let name = text;
                if let Some(ranges) = doc.defs.get(&name) {

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

#[cfg(test)]
mod tests {
    use super::*;
    use rowan::TextSize;

    fn idx_of(src: &str, needle: &str) -> usize {
        src.find(needle).expect("needle not found")
    }

    #[test]
    fn index_collects_val_func_param() {
        let src = "func foo(a) { val x = a; }".to_string();
        let parse = parser::parse(&src);
        let root = parse.syntax();

        let defs = build_index(&root);
        assert!(defs.contains_key("foo"));
        assert!(defs.contains_key("a"));
        assert!(defs.contains_key("x"));
    }

    fn token_text_at(root: &SyntaxNode, range: TextRange) -> Option<String> {
        for el in root.descendants_with_tokens() {
            if let rowan::NodeOrToken::Token(tok) = el {
                if tok.text_range() == range {
                    return Some(tok.text().to_string());
                }
            }
        }
        None
    }

    #[test]
    fn goto_on_val_usage_resolves_definition() {
        let src = "val x = 1; x = 2;".to_string();
        let parse = parser::parse(&src);
        let root = parse.syntax();
        let defs = build_index(&root);

        // Byte offset of the usage (second `x`)
        let usage_start = idx_of(&src, " x = 2");
        let byte_off = usage_start + 1;
        let (kind, _span, text) = lex_token_at_offset(&src, byte_off).expect("token at offset");
        assert_eq!(kind, lsp::lexer::SyntaxKind::Identifier);
        let name = text;
        let def_ranges = defs.get(&name).expect("def exists");
        // The first definition should be the `x` after `val`
        let first_def = def_ranges.first().unwrap();
        // Verify via the tree token text (rowan coordinates)
        let def_text = token_text_at(&root, *first_def).expect("token at def range");
        assert_eq!(def_text, "x");
    }

    #[test]
    fn goto_on_param_usage_resolves_param_def() {
        let src = "func foo(a) { val x = a; }".to_string();
        let parse = parser::parse(&src);
        let root = parse.syntax();
        let defs = build_index(&root);

        // Byte offset of the usage of `a` inside the block
        let usage_start = idx_of(&src, "= a;");
        let byte_off = usage_start + 2;
        let (kind, _span, text) = lex_token_at_offset(&src, byte_off).expect("token at offset");
        assert_eq!(kind, lsp::lexer::SyntaxKind::Identifier);
        let name = text;
        assert_eq!(name, "a");
        let def_ranges = defs.get(&name).expect("param def exists");
        // Ensure at least one definition exists for the param
        assert!(!def_ranges.is_empty());
    }
}