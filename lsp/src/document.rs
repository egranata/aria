use std::collections::HashMap;
use std::sync::Arc;

use line_index::{LineCol, LineIndex};
use rowan::{TextRange, TextSize};
use crate::parser::{self, Parse, SyntaxNode, SyntaxToken};

#[derive(Clone)]
pub struct DocumentState {
    text: Arc<String>,
    text_size: TextSize,
    line_index: Arc<LineIndex>,
    parse: Arc<Parse>,
    defs: HashMap<String, Vec<DefEntry>>,
}

impl DocumentState {
    pub fn new(text: String) -> Self {
        let line_index = LineIndex::new(&text);
        let parse = parser::parse(&text);
        let syntax = parse.syntax();
        let defs = build_index(&syntax);
        Self {
            text_size: TextSize::of(&text),
            text: Arc::new(text),
            parse: Arc::new(parse),
            line_index: Arc::new(line_index),
            defs,
        }
    }

    pub fn update_text(&mut self, text: String) {
        self.text_size = TextSize::of(&text);
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

        if offset > self.text_size {
            return None;
        }

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

    pub fn line_col(&self, offset: rowan::TextSize) -> LineCol {
        self.line_index.line_col(offset)
    }

    pub fn text(&self) -> String {
        self.text.to_string()
    }

    pub fn offset_at_line_col(&self, line: u32, col: u32) -> Option<TextSize> {
        let lc = line_index::LineCol { line, col };
        self.line_index.offset(lc)
    }

    pub fn def_in_scope_at(&self, name: &str, at: TextSize) -> Option<TextRange> {
        let entries = self.defs.get(name)?;
        let mut candidates: Vec<&DefEntry> = entries
            .iter()
            .filter(|e| e.scope_range.contains(at) && (e.hoisted || e.decl_start <= at))
            .collect();

        if candidates.is_empty() {
            candidates = entries.iter().filter(|e| e.hoisted).collect();
        }

        candidates
            .into_iter()
            .min_by(|a, b| {
                use std::cmp::Ordering;
                let len_ord = a.scope_range.len().cmp(&b.scope_range.len());
                if len_ord != Ordering::Equal {
                    return len_ord;
                }
                // Prefer later declaration start (descending)
                b.decl_start.cmp(&a.decl_start)
            })
            .map(|e| e.def_range)
    }

    pub fn definition_at(&self, line: u32, col: u32) -> Option<TextRange> {
        let tok = self.token_at_line_col(line, col)?;
        if tok.kind() == crate::lexer::SyntaxKind::Identifier {
            let name = tok.text();
            let at = tok.text_range().start();
            self.def_in_scope_at(name, at)
        } else {
            None
        }
    }
}


#[derive(Clone, Copy, Debug)]
struct DefEntry {
    def_range: TextRange,
    scope_range: TextRange,
    decl_start: TextSize,
    hoisted: bool,
}

fn build_index(root: &SyntaxNode) -> HashMap<String, Vec<DefEntry>> {
    use crate::lexer::SyntaxKind as K;
    let mut defs: HashMap<String, Vec<DefEntry>> = HashMap::new();

    for node in root.descendants() {
        match node.kind() {
            K::StmtVal | K::Param | K::Func => {
                if let Some(tok) = node
                    .children_with_tokens()
                    .filter_map(|e| e.into_token())
                    .find(|t| t.kind() == K::Identifier)
                {
                    let name = tok.text().to_string();

                    let mut scope_owner = node.parent();
                    while let Some(parent) = scope_owner.clone() {
                        match parent.kind() {
                            K::Func | K::Block => break,
                            _ => scope_owner = parent.parent(),
                        }
                    }

                    let (scope_range, hoisted) = match scope_owner.as_ref() {
                        Some(owner) => (owner.text_range(), false),
                        None => (root.text_range(), true),
                    };

                    let entry = DefEntry {
                        def_range: tok.text_range(),
                        scope_range,
                        decl_start: tok.text_range().start(),
                        hoisted,
                    };

                    defs.entry(name).or_default().push(entry);
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
    fn top_level_val_is_hoisted() {
        let text = "x;\nval x = 1;\n".to_string();
        let doc = DocumentState::new(text);
        let at = doc.offset_at_line_col(0, 0).expect("offset for x use");
        let def = doc.def_in_scope_at("x", at).expect("hoisted top-level def");
        assert_eq!(doc.line_col(def.start()).line, 1);
    }

    #[test]
    fn local_val_is_not_hoisted_within_block() {
        let text = "func f() {\n  y;\n  val y = 2;\n}\n".to_string();
        let doc = DocumentState::new(text);
        let at = doc.offset_at_line_col(1, 2).expect("offset for y use before decl");
        let def = doc.def_in_scope_at("y", at);
        assert!(def.is_none());
    }

    #[test]
    fn shadowing_picks_innermost_definition() {
        let text = "val x = 0;\nfunc f() {\n  val x = 1;\n  x;\n}\n".to_string();
        let doc = DocumentState::new(text);
        let at = doc.offset_at_line_col(3, 2).expect("offset for inner x use");
        let def = doc.def_in_scope_at("x", at).expect("inner x def");
        assert_eq!(doc.line_col(def.start()).line, 2);
    }

    #[test]
    fn params_visible_in_function_body() {
        let text = "func f(a, b) {\n  a;\n}\n".to_string();
        let doc = DocumentState::new(text);
        let at = doc.offset_at_line_col(1, 2).expect("offset for a use in body");
        let def = doc.def_in_scope_at("a", at).expect("param a def");
        assert_eq!(doc.line_col(def.start()).line, 0);
    }

    #[test]
    fn top_level_func_is_hoisted() {
        let text = "g();\nfunc g() {}\n".to_string();
        let doc = DocumentState::new(text);
        let at = doc.offset_at_line_col(0, 0).expect("offset for g() call");
        let def = doc.def_in_scope_at("g", at).expect("hoisted func def");
        assert_eq!(doc.line_col(def.start()).line, 1);
    }

    #[test]
    fn token_at_line_col_out_of_bounds_is_none() {
        let doc = DocumentState::new(sample_text());
        assert!(doc.token_at_line_col(0, 10_000).is_none());
    }

    #[test]
    fn line_col_matches_token_start() {
        let mut doc = DocumentState::new(sample_text());
        let x_tok = doc.token_at_line_col(0, 4).expect("token x");
        assert_eq!(x_tok.text(), "x");
        assert_eq!(x_tok.kind(), SyntaxKind::Identifier);

        doc.update_text(sample_text() + "\nval x = 5;");

        let func_tok = doc.token_at_line_col(1, 0).expect("token func");
        assert_eq!(func_tok.text(), "func");
        assert_eq!(func_tok.kind(), SyntaxKind::FuncKwd);
    }
}