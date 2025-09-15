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

    struct MarkClosed {
        index: usize,
    }
    
    struct Parser<'a> {
        tokens: Vec<(SyntaxKind, &'a str)>,
        pos: usize,
        fuel: Cell<u32>, 
        events: Vec<Event>,
        errors: Vec<String>
    }

    fn prefix_binding_power(op: SyntaxKind) -> Option<((), u8)> {
        use SyntaxKind::*;
        match op {
            Not | Minus | UnaryMinus => Some(((), 23)), 
            _ => None,
        }
    }

    fn postfix_binding_power(op: SyntaxKind) -> Option<(u8, ())> {
        use SyntaxKind::*;
        match op {
            LeftParen | LeftBracket | Dot | DoubleColon => Some((25, ())),
            _ => None,
        }
    }

    fn infix_binding_power(op: SyntaxKind) -> Option<(u8, u8)> {
        use SyntaxKind::*;
        match op {
            LogicalOr => Some((3, 4)),
            LogicalAnd => Some((5, 6)),
            BitwiseXor => Some((7, 8)),
            BitwiseAnd => Some((9, 10)),
            BitwiseOr => Some((11, 12)),
            
            Equal | NotEqual | IsaKwd => Some((13, 14)),
            
            Less | LessEqual | Greater | GreaterEqual => Some((15, 16)),
            
            LeftShift | RightShift => Some((17, 18)),
            
            Plus | Minus => Some((19, 20)),
            
            Star | Slash | Percent => Some((21, 22)),
            
            _ => None,
        }
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
            self.expect(Assign);
            let _ = self.expr();
            self.expect(Semicolon);
            
            self.close(m, StmtVal);
        }
        
        fn stmt_return(&mut self) {
            assert!(self.at(ReturnKwd));
            let m = self.open();
            
            self.expect(ReturnKwd);
            let _ = self.expr();
            self.expect(Semicolon);
            
            self.close(m, StmtReturn);
        }
          
        fn stmt_expr(&mut self) {
            let m = self.open();
            
            let _ = self.expr();
            self.expect(Semicolon);
            
            self.close(m, StmtExpr);
        }

        fn expr(&mut self) -> MarkClosed {
            self.expr_bp(0)
        }

        fn expr_bp(&mut self, min_bp: u8) -> MarkClosed {
            let mut lhs = self.expr_primary();

            loop {
                let op = self.nth(0);
                
                if let Some((l_bp, ())) = postfix_binding_power(op) {
                    if l_bp < min_bp {
                        break;
                    }
                    
                    lhs = match op {
                        LeftParen => {
                            let m = self.open_before(lhs);
                            self.arg_list();
                            self.close(m, ExprCall)
                        }
                        LeftBracket => {
                            let m = self.open_before(lhs);
                            self.expect(LeftBracket);
                            let _ = self.expr_bp(0);
                            self.expect(RightBracket);
                            self.close(m, ExprIndex)
                        }
                        Dot => {
                            let m = self.open_before(lhs);
                            self.expect(Dot);
                            self.expect(Identifier);
                            self.close(m, ExprMember)
                        }
                        DoubleColon => {
                            let m = self.open_before(lhs);
                            self.expect(DoubleColon);
                            self.expect(Identifier);
                            if self.at(LeftParen) {
                                self.expect(LeftParen);
                                let _ = self.expr_bp(0);
                                self.expect(RightParen);
                            }
                            self.close(m, ExprMember)
                        }
                        _ => break,
                    };
                    continue;
                }

                if op == Question {
                    let (l_bp, r_bp) = (1, 2);
                    if l_bp < min_bp {
                        break;
                    }
                    
                    let m = self.open_before(lhs);
                    self.expect(Question);
                    let _ = self.expr_bp(r_bp);
                    self.expect(Colon);
                    let _ = self.expr_bp(r_bp);
                    lhs = self.close(m, ExprTernary);
                    continue;
                }

                if let Some((l_bp, r_bp)) = infix_binding_power(op) {
                    if l_bp < min_bp {
                        break;
                    }
                    
                    let m = self.open_before(lhs);
                    self.advance(); // consume operator
                    let _ = self.expr_bp(r_bp);
                    lhs = self.close(m, ExprBinary);
                    continue;
                }

                break;
            }

            lhs
        }

        fn expr_primary(&mut self) -> MarkClosed {
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
                    let _ = self.expr_bp(0);
                    self.expect(RightParen);
                    self.close(m, ExprParen)
                }
                
                LeftBracket => {
                    self.expect(LeftBracket);
                    if !self.at(RightBracket) {
                        self.expr_list();
                    }
                    self.expect(RightBracket);
                    self.close(m, ExprList)
                }
                
                op if prefix_binding_power(op).is_some() => {
                    let ((), r_bp) = prefix_binding_power(op).unwrap();
                    self.advance(); 
                    let _ = self.expr_bp(r_bp);
                    self.close(m, ExprUnary)
                }
                
                _ => {
                    if !self.eof() {
                        self.advance();
                    }
                    self.close(m, ErrorTree)
                }
            }
        }

        fn expr_list(&mut self) {
            let m = self.open();
            
            let _ = self.expr_bp(0);
            while self.at(Comma) {
                self.expect(Comma);
                if !self.at(RightBracket) {
                    let _ = self.expr_bp(0);
                }
            }
            
            self.close(m, ArgList); 
        }

        fn arg_list(&mut self) {
            assert!(self.at(LeftParen));
            let m = self.open();
            
            self.expect(LeftParen);
            while !self.at(RightParen) && !self.eof() { 
                self.arg();
            }
            self.expect(RightParen);
            
            self.close(m, ArgList);
        }
            
        fn arg(&mut self) {
            let m = self.open();
            
            let _ = self.expr();
            if !self.at(RightParen) { 
                self.expect(Comma);
            }
            
            self.close(m, Arg);
        }

        
        
        fn open(&mut self) -> MarkOpened { 
            let mark = MarkOpened { index: self.events.len() };
            self.events.push(Event::Open { kind: ErrorTree });
            mark
        }
    
        fn close(&mut self, m: MarkOpened, kind: SyntaxKind) -> MarkClosed {
            self.events[m.index] = Event::Open { kind };
            self.events.push(Event::Close);
            MarkClosed { index: m.index }
        }

        fn open_before(&mut self, m: MarkClosed) -> MarkOpened { 
            let mark = MarkOpened { index: m.index };
            // TODO: do something to avoid element shifting
            self.events.insert(
                m.index,
                Event::Open { kind: ErrorTree },
            );
            mark
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
    fn test_param_list() {
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

    #[test]
    fn test_val() {
        expect_tree("func test() { val x = 5; }", &[
            "File@0..19",
            "  Func@0..19",
            "    FuncKwd@0..4 \"func\"",
            "    Identifier@4..8 \"test\"",
            "    ParamList@8..10",
            "      LeftParen@8..9 \"(\"",
            "      RightParen@9..10 \")\"",
            "    Block@10..19",
            "      LeftBrace@10..11 \"{\"",
            "      StmtVal@11..18",
            "        ValKwd@11..14 \"val\"",
            "        Identifier@14..15 \"x\"",
            "        Assign@15..16 \"=\"",
            "        ExprLiteral@16..17",
            "          DecIntLiteral@16..17 \"5\"",
            "        Semicolon@17..18 \";\"",
            "      RightBrace@18..19 \"}\""
        ])
    }

    #[test]
    fn test_binary_expr() {
        expect_tree("func test() { val x = 1 + 2 * 3; }", &[
            "File@0..23",
            "  Func@0..23",
            "    FuncKwd@0..4 \"func\"",
            "    Identifier@4..8 \"test\"",
            "    ParamList@8..10",
            "      LeftParen@8..9 \"(\"",
            "      RightParen@9..10 \")\"",
            "    Block@10..23",
            "      LeftBrace@10..11 \"{\"",
            "      StmtVal@11..22",
            "        ValKwd@11..14 \"val\"",
            "        Identifier@14..15 \"x\"",
            "        Assign@15..16 \"=\"",
            "        ExprBinary@16..21",
            "          ExprLiteral@16..17",
            "            DecIntLiteral@16..17 \"1\"",
            "          Plus@17..18 \"+\"",
            "          ExprBinary@18..21",
            "            ExprLiteral@18..19",
            "              DecIntLiteral@18..19 \"2\"",
            "            Star@19..20 \"*\"",
            "            ExprLiteral@20..21",
            "              DecIntLiteral@20..21 \"3\"",
            "        Semicolon@21..22 \";\"",
            "      RightBrace@22..23 \"}\""
        ])
    }

    #[test]
    fn test_unary_expr() {
        expect_tree("func test() { val x = -5; }", &[
            "File@0..20",
            "  Func@0..20",
            "    FuncKwd@0..4 \"func\"",
            "    Identifier@4..8 \"test\"",
            "    ParamList@8..10",
            "      LeftParen@8..9 \"(\"",
            "      RightParen@9..10 \")\"",
            "    Block@10..20",
            "      LeftBrace@10..11 \"{\"",
            "      StmtVal@11..19",
            "        ValKwd@11..14 \"val\"",
            "        Identifier@14..15 \"x\"",
            "        Assign@15..16 \"=\"",
            "        ExprLiteral@16..18",
            "          DecIntLiteral@16..18 \"-5\"",
            "        Semicolon@18..19 \";\"",
            "      RightBrace@19..20 \"}\""
        ])
    }

    #[test]
    fn test_member_access() {
        expect_tree("func test() { val x = obj.field; }", &[
            "File@0..27",
            "  Func@0..27",
            "    FuncKwd@0..4 \"func\"",
            "    Identifier@4..8 \"test\"",
            "    ParamList@8..10",
            "      LeftParen@8..9 \"(\"",
            "      RightParen@9..10 \")\"",
            "    Block@10..27",
            "      LeftBrace@10..11 \"{\"",
            "      StmtVal@11..26",
            "        ValKwd@11..14 \"val\"",
            "        Identifier@14..15 \"x\"",
            "        Assign@15..16 \"=\"",
            "        ExprMember@16..25",
            "          ExprName@16..19",
            "            Identifier@16..19 \"obj\"",
            "          Dot@19..20 \".\"",
            "          Identifier@20..25 \"field\"",
            "        Semicolon@25..26 \";\"",
            "      RightBrace@26..27 \"}\""
        ])
    }

    #[test]
    fn test_array_access() {
        expect_tree("func test() { val x = arr[0]; }", &[
            "File@0..24",
            "  Func@0..24",
            "    FuncKwd@0..4 \"func\"",
            "    Identifier@4..8 \"test\"",
            "    ParamList@8..10",
            "      LeftParen@8..9 \"(\"",
            "      RightParen@9..10 \")\"",
            "    Block@10..24",
            "      LeftBrace@10..11 \"{\"",
            "      StmtVal@11..23",
            "        ValKwd@11..14 \"val\"",
            "        Identifier@14..15 \"x\"",
            "        Assign@15..16 \"=\"",
            "        ExprIndex@16..22",
            "          ExprName@16..19",
            "            Identifier@16..19 \"arr\"",
            "          LeftBracket@19..20 \"[\"",
            "          ExprLiteral@20..21",
            "            DecIntLiteral@20..21 \"0\"",
            "          RightBracket@21..22 \"]\"",
            "        Semicolon@22..23 \";\"",
            "      RightBrace@23..24 \"}\""
        ])
    }

    #[test]
    fn test_function_call() {
        expect_tree("func test() { val x = foo(1, 2); }", &[
            "File@0..26",
            "  Func@0..26",
            "    FuncKwd@0..4 \"func\"",
            "    Identifier@4..8 \"test\"",
            "    ParamList@8..10",
            "      LeftParen@8..9 \"(\"",
            "      RightParen@9..10 \")\"",
            "    Block@10..26",
            "      LeftBrace@10..11 \"{\"",
            "      StmtVal@11..25",
            "        ValKwd@11..14 \"val\"",
            "        Identifier@14..15 \"x\"",
            "        Assign@15..16 \"=\"",
            "        ExprCall@16..24",
            "          ExprName@16..19",
            "            Identifier@16..19 \"foo\"",
            "          ArgList@19..24",
            "            LeftParen@19..20 \"(\"",
            "            Arg@20..22",
            "              ExprLiteral@20..21",
            "                DecIntLiteral@20..21 \"1\"",
            "              Comma@21..22 \",\"",
            "            Arg@22..23",
            "              ExprLiteral@22..23",
            "                DecIntLiteral@22..23 \"2\"",
            "            RightParen@23..24 \")\"",
            "        Semicolon@24..25 \";\"",
            "      RightBrace@25..26 \"}\""
        ])
    }

    #[test]
    fn test_chained_postfix() {
        expect_tree("func test() { val x = obj.method().field[0]; }", &[
            "File@0..39",
            "  Func@0..39",
            "    FuncKwd@0..4 \"func\"",
            "    Identifier@4..8 \"test\"",
            "    ParamList@8..10",
            "      LeftParen@8..9 \"(\"",
            "      RightParen@9..10 \")\"",
            "    Block@10..39",
            "      LeftBrace@10..11 \"{\"",
            "      StmtVal@11..38",
            "        ValKwd@11..14 \"val\"",
            "        Identifier@14..15 \"x\"",
            "        Assign@15..16 \"=\"",
            "        ExprIndex@16..37",
            "          ExprMember@16..34",
            "            ExprCall@16..28",
            "              ExprMember@16..26",
            "                ExprName@16..19",
            "                  Identifier@16..19 \"obj\"",
            "                Dot@19..20 \".\"",
            "                Identifier@20..26 \"method\"",
            "              ArgList@26..28",
            "                LeftParen@26..27 \"(\"",
            "                RightParen@27..28 \")\"",
            "            Dot@28..29 \".\"",
            "            Identifier@29..34 \"field\"",
            "          LeftBracket@34..35 \"[\"",
            "          ExprLiteral@35..36",
            "            DecIntLiteral@35..36 \"0\"",
            "          RightBracket@36..37 \"]\"",
            "        Semicolon@37..38 \";\"",
            "      RightBrace@38..39 \"}\""
        ])
    }
}