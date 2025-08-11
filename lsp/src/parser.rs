use std::cell::Cell;

use crate::{SyntaxKind, lexer};
use rowan::GreenNode;
use rowan::GreenNodeBuilder;

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
        assert!(raw.0 <= SyntaxKind::Arg as u16);
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
        fn parse(&mut self) {
            self.file();
        }

        fn file(&mut self) {
            let m: MarkOpened = self.open(); 

            while !self.eof() { 
              if self.at(SyntaxKind::FuncKwd) {
                self.func()
              } else {
                self.advance_with_error("expected a function"); 
              }
            }
          
            self.close(m, SyntaxKind::File); 
        }

        fn func(&mut self) {
            
        }

        fn open(&mut self) -> MarkOpened { 
            let mark = MarkOpened { index: self.events.len() };
            self.events.push(Event::Open { kind: SyntaxKind::ErrorTree });
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
                .map_or(SyntaxKind::Eof, |it| it.0)
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
            self.close(m, SyntaxKind::ErrorTree);
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

    let mut tokens = lexer::lex(text);
    tokens.reverse();
    
    let mut parser = Parser { tokens, pos: 0, fuel: Cell::new(0), events: Vec::new(), errors: Vec::new() };
    parser.parse();
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

    #[test]
    fn test_basic() {
        let input = "";
        let parse_result = parse(input);
    }
}