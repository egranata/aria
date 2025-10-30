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
        use crate::lexer::SyntaxKind as K;

        match self.parse.syntax().token_at_offset(offset) {
            rowan::TokenAtOffset::Single(tok) => Some(tok),
            rowan::TokenAtOffset::Between(left, right) => {
                // Prefer non-trivia if possible
                let is_trivia = |t: &SyntaxToken| matches!(t.kind(), K::Whitespace | K::LineComment);
                match (is_trivia(&left), is_trivia(&right)) {
                    (false, false) => Some(left),
                    (false, true) => Some(left),
                    (true, false) => Some(right),
                    (true, true) => Some(left),
                }
            }
            rowan::TokenAtOffset::None => None,
        }
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

#[cfg(test)]
mod tests {
    use crate::SyntaxKind;

    use super::*;

    fn sample_text() -> String {
        // 0: "val x = 1;"
        // 1: "func foo(a, b) {}"
        // 2: "func bar() { val y = 2; }"
        "val x = 1;\nfunc foo(a, b) {}\nfunc bar() { val y = 2; }\n".to_string()
    }

    #[test]
    fn indexes_defs_for_val_func_and_params() {
        let doc = DocumentState::new(sample_text());

        let defs_x = doc.def("x").expect("def x present");
        assert!(!defs_x.is_empty());
        assert!(defs_x.iter().any(|r| doc.line_col(r.start()).line == 0));

        let defs_foo = doc.def("foo").expect("def foo present");
        assert!(!defs_foo.is_empty());
        assert!(defs_foo.iter().any(|r| doc.line_col(r.start()).line == 1));

        let defs_a = doc.def("a").expect("def a present");
        assert!(!defs_a.is_empty());
        assert!(defs_a.iter().any(|r| doc.line_col(r.start()).line == 1));

        let defs_b = doc.def("b").expect("def b present");
        assert!(!defs_b.is_empty());
        assert!(defs_b.iter().any(|r| doc.line_col(r.start()).line == 1));

        let defs_bar = doc.def("bar").expect("def bar present");
        assert!(!defs_bar.is_empty());

        let defs_y = doc.def("y").expect("def y present");
        assert!(!defs_y.is_empty());
        assert!(defs_y.iter().any(|r| doc.line_col(r.start()).line == 2));
    }

    #[test]
    fn token_at_line_col_out_of_bounds_is_none() {
        let doc = DocumentState::new(sample_text());
        assert!(doc.token_at_line_col(100, 0).is_none());
        assert!(doc.token_at_line_col(0, 10_000).is_none());
    }

    #[test]
    fn update_text_rebuilds_index() {
        let mut doc = DocumentState::new("val a = 1;\n".to_string());
        assert!(doc.def("a").is_some());
        assert!(doc.def("b").is_none());

        doc.update_text("val b = 2;\n".to_string());
        assert!(doc.def("a").is_none());
        let defs_b = doc.def("b").expect("def b present after update");
        assert!(!defs_b.is_empty());
    }

    #[test]
    fn line_col_matches_token_start() {
        let doc = DocumentState::new(sample_text());
        let x_tok = doc.token_at_line_col(0, 3).expect("token x");
        assert_eq!(x_tok.text(), "x");
        assert_eq!(x_tok.kind(), SyntaxKind::Identifier);

        let func_tok = doc.token_at_line_col(1, 0).expect("token func");
        assert_eq!(func_tok.text(), "func");
        assert_eq!(func_tok.kind(), SyntaxKind::FuncKwd);
    }
}