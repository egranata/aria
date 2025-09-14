use std::cell::Cell;

use crate::{SyntaxKind, lexer};
use rowan::{GreenNode, GreenNodeBuilder};
use SyntaxKind::*;

impl From<SyntaxKind> for rowan::SyntaxKind {
    fn from(kind: SyntaxKind) -> Self {
        Self(kind as u16)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Lang {}
impl rowan::Language for Lang {
    type Kind = SyntaxKind;
    fn kind_from_raw(raw: rowan::SyntaxKind) -> Self::Kind {
        assert!(raw.0 <= Arg as u16);
        unsafe { std::mem::transmute::<u16, SyntaxKind>(raw.0) }
    }
    fn kind_to_raw(kind: Self::Kind) -> rowan::SyntaxKind {
        kind.into()
    }
}

pub struct Parse {
    green_node: GreenNode,
    #[allow(unused)]
    errors: Vec<String>,
}

pub fn parse(text: &str) -> Parse {
    enum Event {
        Open { kind: SyntaxKind }, 
        Close,
        Advance,
    }
    
    struct MarkOpened {
        index: usize,
    }
    
    struct Parser<'a> {
        tokens: Vec<(SyntaxKind, &'a str)>,
        pos: usize,
        fuel: Cell<u32>, 
        events: Vec<Event>,
        errors: Vec<String>
    }
    
    impl Parser<'_> {
        fn file(&mut self) {
            let m: MarkOpened = self.open(); 

            while !self.eof() {
                println!("current token: {:?}", self.tokens[self.pos]);
                if self.at(FuncKwd) {
                    self.func()
                } else {
                    self.advance_with_error("expected a function"); 
                }
            }
            
            self.close(m, File); 
        }

        fn func(&mut self) {
            assert!(self.at(FuncKwd)); 
            let m = self.open(); 
          
            self.expect(FuncKwd);
            self.expect(Identifier);

            if self.at(LeftParen) { 
                self.param_list();
            }
            
            if self.at(LeftBrace) { 
                self.block();
            }
          
            self.close(m, Func);
        }

        fn param_list(&mut self) {
            assert!(self.at(LeftParen));
            let m = self.open();
          
            self.expect(LeftParen); 
            while !self.at(RightParen) && !self.eof() { 
              if self.at(Identifier) { 
                self.param();
              } else {
                break; 
              }
            }
            self.expect(RightParen); 
          
            self.close(m, ParamList);
        }

        fn param(&mut self) {
            assert!(self.at(Identifier));
            let m = self.open();
          
            self.expect(Identifier);
            self.expect(Colon);

            self.expect(Identifier);
            
            if !self.at(RightParen) { 
              self.expect(Comma);
            }
          
            self.close(m, Param);
        }

        fn block(&mut self) {
            assert!(self.at(LeftBrace));
            let m = self.open();
          
            self.expect(LeftBrace);
            while !self.at(RightBrace) && !self.eof() {
                  match self.nth(0) {
                    ValKwd => self.stmt_val(),
                    ReturnKwd => self.stmt_return(),
                    _ => self.stmt_expr(),
                  }
                break
            }
            self.expect(RightBrace);
          
            self.close(m, Block);
        }

        fn stmt_val(&mut self) {
            assert!(self.at(ValKwd));
            let m = self.open();
            
            self.expect(ValKwd);
            self.expect(Identifier);
            self.expect(Equal);
            self.expr();
            self.expect(Semicolon);
            
            self.close(m, StmtVal);
        }
        
        fn stmt_return(&mut self) {
            assert!(self.at(ReturnKwd));
            let m = self.open();
            
            self.expect(ReturnKwd);
            self.expr();
            self.expect(Semicolon);
            
            self.close(m, StmtReturn);
        }
          
        fn stmt_expr(&mut self) {
            let m = self.open();
            
            self.expr();
            self.expect(Semicolon);
            
            self.close(m, StmtExpr);
        }

        fn expr(&mut self) {
            self.expr_delimited()
        }
        
        fn expr_delimited(&mut self) {
            let m = self.open();
            match self.nth(0) {
                HexIntLiteral | OctIntLiteral | BinIntLiteral | DecIntLiteral | 
                FloatLiteral | StringLiteral | TrueKwd | FalseKwd => {
                    self.advance();
                    self.close(m, ExprLiteral)
                }
            
                Identifier => {
                    self.advance();
                    self.close(m, ExprName)
                }
            
                LeftParen => {
                    self.expect(LeftParen);
                    self.expr();
                    self.expect(RightParen);
                    self.close(m, ExprParen)
                }
            
                _ => {
                    if !self.eof() {
                    self.advance();
                    }
                    self.close(m, ErrorTree)
                }
            }
        }
        
        fn open(&mut self) -> MarkOpened { 
            let mark = MarkOpened { index: self.events.len() };
            self.events.push(Event::Open { kind: ErrorTree });
            mark
        }
    
        fn close(&mut self, m: MarkOpened, kind: SyntaxKind) {
            self.events[m.index] = Event::Open { kind };
            self.events.push(Event::Close);
        }
    
        fn advance(&mut self) { 
            assert!(!self.eof());
            self.fuel.set(256); 
            self.events.push(Event::Advance);
            self.pos += 1;
        }
    
        fn eof(&self) -> bool {
            self.pos == self.tokens.len()
        }
    
        fn nth(&self, lookahead: usize) -> SyntaxKind { 
            if self.fuel.get() == 0 { 
                panic!("parser is stuck")
            }
            self.fuel.set(self.fuel.get() - 1);
            self.tokens.get(self.pos + lookahead)
                .map_or(Eof, |it| it.0)
        }
    
        fn at(&self, kind: SyntaxKind) -> bool { 
            self.nth(0) == kind
        }
    
        fn eat(&mut self, kind: SyntaxKind) -> bool { 
            if self.at(kind) {
                self.advance();
                true
            } else {
                false
            }
        }
    
        fn expect(&mut self, kind: SyntaxKind) {
            if self.eat(kind) {
                return;
            }
            // TODO: Error reporting.
            eprintln!("expected {kind:?}");
        }
    
        fn advance_with_error(&mut self, error: &str) {
            let m = self.open();
            // TODO: Error reporting.
            eprintln!("{error}");
            self.advance();
            self.close(m, ErrorTree);
        }

        fn build_tree(self) -> Parse {
            let mut tokens = self.tokens.into_iter();
            let mut builder = GreenNodeBuilder::new();
                
            for event in self.events {
              match event {
                Event::Open { kind } => {
                    builder.start_node(kind.into());
                }
        
                Event::Close => {
                    builder.finish_node();
                }
        
                Event::Advance => {
                    let (token, slice) = tokens.next().unwrap();
                    builder.token(token.into(), slice);
                }
              }
            }
        
            assert!(tokens.next().is_none());

            Parse { green_node: builder.finish(), errors: self.errors }
        }
    }  

    let lex = lexer::lex(text);
    let tokens = lex.into_iter().map(|res| res.unwrap()).collect();
    let mut parser = Parser { tokens, pos: 0, fuel: Cell::new(256), events: Vec::new(), errors: Vec::new() };
    parser.file();
    parser.build_tree()
}

type SyntaxNode = rowan::SyntaxNode<Lang>;

#[allow(unused)]
type SyntaxToken = rowan::SyntaxToken<Lang>;

#[allow(unused)]
type SyntaxElement = rowan::NodeOrToken<SyntaxNode, SyntaxToken>;

impl Parse {
    fn syntax(&self) -> SyntaxNode {
        SyntaxNode::new_root(self.green_node.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    fn tree_to_string(node: SyntaxNode) -> String {
        let mut result = Vec::new();
        tree_to_string_impl(&node, 0, &mut result);
        result.join("\n")
    }

    fn tree_to_string_impl(node: &SyntaxNode, depth: usize, result: &mut Vec<String>) {
        let indent = "  ".repeat(depth);
        result.push(format!("{}{:?}@{:?}", indent, node.kind(), node.text_range()));
        
        for child in node.children_with_tokens() {
            match child {
                rowan::NodeOrToken::Node(child_node) => {
                    tree_to_string_impl(&child_node, depth + 1, result);
                }
                rowan::NodeOrToken::Token(token) => {
                    let token_indent = "  ".repeat(depth + 1);
                    result.push(format!("{}{:?}@{:?} {:?}", 
                        token_indent, 
                        token.kind(), 
                        token.text_range(),
                        token.text()
                    ));
                }
            }
        }
    }

    fn expect_tree(input: &str, lines: &[&str]) {
        let node = parse(input).syntax();
        let tree_str = tree_to_string(node);
        let expected = lines.join("\n");
        assert_eq!(expected, tree_str);
    }

    #[test]
    fn test_empty() {
        expect_tree("", &[
            "File@0..0"
        ])
    }

    #[test]
    fn test_empty_function() {
        expect_tree("func empty_func() {}", &[
            "File@0..18",
            "  Func@0..18",
            "    FuncKwd@0..4 \"func\"",
            "    Identifier@4..14 \"empty_func\"",
            "    ParamList@14..16",
            "      LeftParen@14..15 \"(\"",
            "      RightParen@15..16 \")\"",
            "    Block@16..18",
            "      LeftBrace@16..17 \"{\"",
            "      RightBrace@17..18 \"}\""
        ])
    }

    #[test]
    fn test_params() {
        expect_tree("func empty_func(x, y) {}", &[
            "File@0..21",
            "  Func@0..21",
            "    FuncKwd@0..4 \"func\"",
            "    Identifier@4..14 \"empty_func\"",
            "    ParamList@14..19",
            "      LeftParen@14..15 \"(\"",
            "      Param@15..17",
            "        Identifier@15..16 \"x\"",
            "        Comma@16..17 \",\"",
            "      Param@17..18",
            "        Identifier@17..18 \"y\"",
            "      RightParen@18..19 \")\"",
            "    Block@19..21",
            "      LeftBrace@19..20 \"{\"",
            "      RightBrace@20..21 \"}\""
        ])
    }
}