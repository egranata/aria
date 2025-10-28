use std::collections::HashMap;
use std::sync::Arc;

use line_index::{LineCol, LineIndex};
use rowan::TextRange;
use crate::parser::{self, Parse, SyntaxNode, SyntaxToken};

#[derive(Clone)]
pub struct DocumentState {
    text: Arc<String>,
    line_index: Arc<LineIndex>,
    parse: Arc<Parse>,
    defs: HashMap<String, Vec<TextRange>>,
}

impl DocumentState {
    pub fn new(text: String) -> Self {
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

    pub fn update_text(&mut self, text: String) {
        self.text = Arc::new(text);
        self.line_index = Arc::new(LineIndex::new(&self.text));
        let parse = parser::parse(&self.text);
        let syntax = parse.syntax();

        self.parse = Arc::new(parse);
        self.defs = build_index(&syntax);
    }

    pub fn token_at_line_col(&self, line: u32, col: u32) -> Option<SyntaxToken> {
        let line_col = line_index::LineCol { line, col };
        let Some(offset) = self.line_index.offset(line_col) else { return None };

        for el in self.parse.syntax().descendants_with_tokens() {
            if let rowan::NodeOrToken::Token(tok) = el {
                if tok.text_range().contains(offset) {
                    return Some(tok);
                }
            }
        }
        None
    }

    pub fn def(&self, name: &str) -> Option<&Vec<TextRange>> {
        self.defs.get(name)
    }

    pub fn line_col(&self, offset: rowan::TextSize) -> LineCol {
        self.line_index.line_col(offset)
    }
}


fn build_index(root: &SyntaxNode) -> HashMap<String, Vec<TextRange>> {
    use crate::lexer::SyntaxKind as K;
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