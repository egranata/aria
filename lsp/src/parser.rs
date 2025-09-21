use std::cell::Cell;
use line_index::LineIndex;
use crate::{SyntaxKind, lexer};
use rowan::{GreenNode, GreenNodeBuilder, TextSize};
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
        tokens: Vec<(SyntaxKind, &'a str, logos::Span)>,
        pos: usize,
        fuel: Cell<u32>, 
        events: Vec<Event>,
        errors: Vec<String>,
        line_index: LineIndex
    }

    fn prefix_binding_power(op: SyntaxKind) -> Option<((), u8)> {
        use SyntaxKind::*;
        match op {
            Not | Minus => Some(((), 23)), 
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
            // Assignment operators (right-associative, lowest precedence)
            Assign | PlusAssign | MinusAssign | StarAssign | SlashAssign | PercentAssign => Some((2, 1)),
            
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
                match self.nth(0) {
                    ImportKwd => self.stmt_import(),
                    ValKwd => self.decl_val(),
                    Identifier => {self.expr();},
                    StructKwd => self.decl_struct(),
                    MixinKwd => self.decl_mixin(),
                    EnumKwd => self.decl_enum(),
                    ExtensionKwd => self.decl_extension(),
                    FuncKwd => self.decl_func(),
                    AssertKwd => self.stmt_assert(),
                    _ => self.advance_with_error("expected a function") 
                }
            }
            
            self.close(m, File); 
        }

        fn decl_struct(&mut self) {
            assert!(self.at(StructKwd));
            let m = self.open();
            
            self.expect(StructKwd);
            self.expect(Identifier);
            self.expect(LeftBrace);
            
            // Parse struct entries (method_decl | operator_decl | "type" ~ val_decl_stmt | mixin_include_decl | struct_decl | enum_decl)
            while !self.at(RightBrace) && !self.eof() {
                match self.nth(0) {
                    FuncKwd => self.decl_func(),
                    StructKwd => self.decl_struct(),
                    EnumKwd => self.decl_enum(),
                    IncludeKwd => self.mixin_include(),
                    TypeKwd => {
                        self.expect(TypeKwd);
                        if self.at(ValKwd) {
                            self.decl_val();
                        } else {
                            self.decl_func();
                        }
                    }
                    _ => self.advance_with_error("expected struct entry")
                }
            }
            
            self.expect(RightBrace);
            self.close(m, Struct);
        }

        fn mixin_include(&mut self) {
            assert!(self.at(IncludeKwd));
            let m = self.open();
            
            self.expect(IncludeKwd);
            let _ = self.expr();
            
            self.close(m, MixinInclude);
        }

        fn decl_enum(&mut self) {
            assert!(self.at(EnumKwd));
            let m = self.open();
            
            self.expect(EnumKwd);
            self.expect(Identifier);
            self.expect(LeftBrace);
            
            // Parse enum entries (enum_case_decl | struct_entry)
            while !self.at(RightBrace) && !self.eof() {
                match self.nth(0) {
                    CaseKwd => self.enum_case(),
                    FuncKwd => self.decl_func(),
                    StructKwd => self.decl_struct(),
                    EnumKwd => self.decl_enum(),
                    IncludeKwd => self.mixin_include(),
                    TypeKwd => {
                        self.expect(TypeKwd);
                        self.decl_val();
                    }
                    _ => self.advance_with_error("expected enum entry")
                }
                
                // Optional comma
                if self.at(Comma) {
                    self.expect(Comma);
                }
            }
            
            self.expect(RightBrace);
            self.close(m, Enum);
        }

        fn enum_case(&mut self) {
            assert!(self.at(CaseKwd));
            let m = self.open();
            
            self.expect(CaseKwd);
            self.expect(Identifier);
            
            // Optional parameter: ("(" ~ expression ~ ")")?
            if self.at(LeftParen) {
                self.expect(LeftParen);
                let _ = self.expr();
                self.expect(RightParen);
            }
            
            self.close(m, EnumCase);
        }

        fn decl_mixin(&mut self) {
            assert!(self.at(MixinKwd));
            let m = self.open();
            
            self.expect(MixinKwd);
            self.expect(Identifier);
            self.expect(LeftBrace);
            
            // Parse mixin entries (method_decl | operator_decl | mixin_include_decl)
            while !self.at(RightBrace) && !self.eof() {
                match self.nth(0) {
                    FuncKwd => self.decl_func(),
                    OperatorKwd => self.decl_operator(),
                    IncludeKwd => self.mixin_include(),
                    _ => self.advance_with_error("expected mixin entry")
                }
            }
            
            self.expect(RightBrace);
            self.close(m, Mixin);
        }

        fn decl_operator(&mut self) {
            let m = self.open();
            
            // Optional reverse direction
            if self.at(ReverseKwd) {
                self.expect(ReverseKwd);
            }
            
            self.expect(OperatorKwd);
            
            // Operator symbol - need to handle various operator tokens
            match self.nth(0) {
                Plus | Minus | Star | Slash | Percent | LeftShift | RightShift |
                Equal | LessEqual | GreaterEqual | Less | Greater |
                BitwiseAnd | BitwiseOr | BitwiseXor |
                LeftParen | LeftBracket => {
                    self.advance();
                }
                _ => self.advance_with_error("expected operator symbol")
            }
            
            if self.at(LeftParen) {
                self.param_list();
            }
            
            if self.at(LeftBrace) {
                self.block();
            }
            
            self.close(m, Operator);
        }

        fn decl_extension(&mut self) {
            assert!(self.at(ExtensionKwd));
            let m = self.open();
            
            self.expect(ExtensionKwd);
            let _ = self.expr(); // extension target expression
            self.expect(LeftBrace);
            
            // Parse struct entries (same as struct_decl)
            while !self.at(RightBrace) && !self.eof() {
                match self.nth(0) {
                    FuncKwd => self.decl_func(),
                    OperatorKwd => self.decl_operator(),
                    StructKwd => self.decl_struct(),
                    EnumKwd => self.decl_enum(),
                    IncludeKwd => self.mixin_include(),
                    TypeKwd => {
                        self.expect(TypeKwd);
                        self.decl_val();
                    }
                    _ => self.advance_with_error("expected extension entry")
                }
            }
            
            self.expect(RightBrace);
            self.close(m, Extension);
        }

        fn decl_func(&mut self) {
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

            if self.at(Colon) {
                self.expect(Colon);
                self.expect(Identifier);
            }
            
            if !self.at(RightParen) { 
              self.expect(Comma);
            }
          
            self.close(m, Param);
        }

        fn block(&mut self) {
            self.assert_tok(LeftBrace);
            let m = self.open();
          
            self.expect(LeftBrace);
            while !self.at(RightBrace) && !self.eof() {
                  match self.nth(0) {
                    AssertKwd => self.stmt_assert(),
                    BreakKwd => self.stmt_single_token(BreakKwd),
                    ContinueKwd => self.stmt_single_token(ContinueKwd),
                    ValKwd => self.decl_val(),
                    IfKwd => self.stmt_if(),
                    MatchKwd => self.stmt_match(),
                    WhileKwd => self.stmt_while(),
                    ForKwd => self.stmt_for(),
                    ReturnKwd => self.stmt_return(),
                    LeftBrace => self.block(),
                    GuardKwd => self.guard(),
                    TryKwd => self.try_catch(),
                    StructKwd => self.decl_struct(),
                    EnumKwd => self.decl_enum(),
                    _ => self.stmt_expr(),
                  }
            }
            self.expect(RightBrace);
          
            self.close(m, Block);
        }

        fn guard(&mut self) {
            assert!(self.at(GuardKwd));
            let m = self.open();
            
            self.expect(GuardKwd);
            self.expect(Identifier);
            self.expect(Assign);
            let _ = self.expr();
            self.block();
            
            self.close(m, Guard);
        }

        fn stmt_if(&mut self) {
            assert!(self.at(IfKwd));
            let m = self.open();
            
            // if piece
            self.expect(IfKwd);
            let _ = self.expr();
            self.block();
            
            // elsif pieces
            while self.at(ElsifKwd) {
                self.expect(ElsifKwd);
                let _ = self.expr();
                self.block();
            }
            
            // optional else piece
            if self.at(ElseKwd) {
                self.expect(ElseKwd);
                self.block();
            }
            
            self.close(m, StmtIf);
        }

        fn stmt_for(&mut self) {
            assert!(self.at(ForKwd));
            let m = self.open();
            
            self.expect(ForKwd);
            self.expect(Identifier);
            self.expect(InKwd);
            let _ = self.expr();
            self.block();
            
            // optional else piece
            if self.at(ElseKwd) {
                self.expect(ElseKwd);
                self.block();
            }
            
            self.close(m, StmtFor);
        }

        fn stmt_match(&mut self) {
            assert!(self.at(MatchKwd));
            let m = self.open();
            
            self.expect(MatchKwd);
            let _ = self.expr();
            self.expect(LeftBrace);
            
            // Parse match rules
            if !self.at(RightBrace) {
                self.match_rule();
                
                while (self.at(Comma) || !self.at(RightBrace)) && !self.eof() {
                    if self.at(Comma) {
                        self.expect(Comma);
                    }
                    if !self.at(RightBrace) {
                        self.match_rule();
                    }
                }
                
                if self.at(Comma) {
                    self.expect(Comma);
                }
            }
            
            self.expect(RightBrace);
            
            // optional else piece
            if self.at(ElseKwd) {
                self.expect(ElseKwd);
                self.block();
            }
            
            self.close(m, StmtMatch);
        }

        fn match_rule(&mut self) {
            let m = self.open();
            
            self.match_pattern();
            
            // Handle "and" patterns
            while self.at(AndKwd) {
                self.expect(AndKwd);
                self.match_pattern();
            }
            
            self.expect(Arrow); // "=>"
            self.block();
            
            self.close(m, MatchRule);
        }

        fn match_pattern(&mut self) {
            let m = self.open();
            
            match self.nth(0) {
                CaseKwd => {
                    self.expect(CaseKwd);
                    self.expect(Identifier);
                    if self.at(LeftParen) {
                        self.expect(LeftParen);
                        self.expect(Identifier);
                        if self.at(Colon) {
                            self.expect(Colon);
                            let _ = self.expr();
                        }
                        self.expect(RightParen);
                    }
                }
                Equal | NotEqual | IsaKwd => {
                    self.advance(); // comparison operator
                    let _ = self.expr();
                }
                Less | LessEqual | Greater | GreaterEqual => {
                    self.advance(); // relational operator
                    let _ = self.expr();
                }
                _ => self.advance_with_error("expected match pattern")
            }
            
            self.close(m, MatchPattern);
        }

        fn stmt_while(&mut self) {
            assert!(self.at(WhileKwd));
            let m = self.open();
            
            self.expect(WhileKwd);
            let _ = self.expr();
            self.block();
            
            // optional else piece
            if self.at(ElseKwd) {
                self.expect(ElseKwd);
                self.block();
            }
            
            self.close(m, StmtWhile);
        }

        fn try_catch(&mut self) {
            assert!(self.at(TryKwd));
            let m = self.open();
            
            self.expect(TryKwd);
            self.block();
            self.expect(CatchKwd);
            self.expect(Identifier);
            self.block();
            
            self.close(m, TryBlock);
        }

        fn stmt_import(&mut self) {
            assert!(self.at(ImportKwd));
            let m = self.open();
            
            self.expect(ImportKwd);
            
            let mut is_from_import = false;
            let mut temp_pos = self.pos;
            
            while temp_pos < self.tokens.len() && 
                  self.tokens[temp_pos].0 != FromKwd && 
                  self.tokens[temp_pos].0 != Semicolon {
                temp_pos += 1;
            }
            
            if temp_pos < self.tokens.len() && self.tokens[temp_pos].0 == FromKwd {
                is_from_import = true;
            }
            
            if is_from_import {
                if self.at(Star) {
                    self.expect(Star);
                } else {
                    self.ident_list();
                }
                
                self.expect(FromKwd);
            }
            
            self.import_path();
            self.expect(Semicolon);
            
            self.close(m, StmtImport);
        }

        fn decl_val(&mut self) {
            assert!(self.at(ValKwd));
            let m = self.open();
            
            self.expect(ValKwd);
            self.expect(Identifier);
            self.expect(Assign);
            let _ = self.expr();

            if self.at(LeftBrace) {
                self.init_block();
            }

            self.expect(Semicolon);

            self.close(m, StmtVal);
        }

        fn stmt_single_token(&mut self, kind: SyntaxKind) {
            assert!(self.at(kind));
            let m = self.open();
            
            self.expect(kind);
            self.expect(Semicolon);
            
            self.close(m, StmtReturn);
        }

        fn stmt_return(&mut self) {
            assert!(self.at(ReturnKwd));
            let m = self.open();
            
            self.expect(ReturnKwd);

            if !self.at(Semicolon) {
                let _ = self.expr();
            }

            if self.at(LeftBrace) {
                self.init_block();
            }

            self.expect(Semicolon);
            self.close(m, StmtReturn);
        }

        fn init_block(&mut self) {
            self.assert_tok(LeftBrace);
            self.expect(LeftBrace);

            while self.at(Dot) {
                self.expect(Dot);
                self.expect(Identifier);
                self.expect(Assign);
                let _ = self.expr();

                if !self.at(RightBrace) {
                    self.expect(Comma);
                }
            }

            self.expect(RightBrace);
        }

        fn ident_list(&mut self) {
            let m = self.open();
            
            self.expect(Identifier);
            while self.at(Comma) {
                self.expect(Comma);
                if self.at(Identifier) {
                    self.expect(Identifier);
                }
            }
            
            self.close(m, IdentList);
        }

        fn import_path(&mut self) {
            let m = self.open();
            
            self.expect(Identifier);
            while self.at(Dot) {
                self.expect(Dot);
                self.expect(Identifier);
            }
            
            self.close(m, ImportPath);
        }

        fn stmt_assert(&mut self) {
            assert!(self.at(AssertKwd));
            let m = self.open();
            
            self.expect(AssertKwd);
            let _ = self.expr();
            self.expect(Semicolon);

            self.close(m, StmtAssert);
        }
          
        fn stmt_expr(&mut self) {
            let m = self.open();
            
            let _ = self.expr();

            if self.at(LeftBrace) {
                self.init_block();
            }

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
                    
                    // Use ExprAssign for assignment operators, ExprBinary for others
                    let node_kind = match op {
                        Assign | PlusAssign | MinusAssign | StarAssign | SlashAssign | PercentAssign => ExprAssign,
                        _ => ExprBinary,
                    };
                    
                    lhs = self.close(m, node_kind);
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
            let _ = self.expr_bp(0);
            while self.at(Comma) {
                self.expect(Comma);
                if !self.at(RightBracket) {
                    let _ = self.expr_bp(0);
                }
            }
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

        fn nth_token(&self, lookahead: usize) -> Option<&(SyntaxKind, &str, logos::Span)> {
            self.tokens.get(self.pos + lookahead)
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
            // TODO: replace this with error message instead of assert
            self.assert_tok(kind);
        }
    
        fn advance_with_error(&mut self, error: &str) {
            let m = self.open();
            self.report_error(error);
            self.advance();
            self.close(m, ErrorTree);
        }

        fn report_error(&mut self, error: &str) {            
            let msg = if let Some(tok) = self.nth_token(0) {                
                let pos = &tok.2;
                let line_col = self.line_index.line_col(TextSize::new(pos.start.try_into().unwrap()));

                format!("{error} at line {}, column {}", line_col.line + 1, line_col.col + 1)
            } else {
                format!("{error}")
            };
            
            self.errors.push(msg);
        }

        fn get_context_msg(&mut self) -> String {
            if let Some(tok) = self.nth_token(0) {                
                let pos = &tok.2;
                let line_col = self.line_index.line_col(TextSize::new(pos.start.try_into().unwrap()));

                format!("token: {:?}, line: {}, col: {}", tok.0, line_col.line + 1, line_col.col + 1)
            } else {
                format!("no token available")
            }
        }

        fn assert_tok(&mut self, kind: SyntaxKind) {
            let msg = self.get_context_msg();
            assert!(self.at(kind), "expected token {:?} instead of {}", kind, msg);
        }

        fn build_tree(self) -> Parse {
            let mut tokens = self.tokens.into_iter();

            // TODO: add a shared node cache
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
                    let (token, slice, _) = tokens.next().unwrap();
                    builder.token(token.into(), slice);
                }
              }
            }
        
            assert!(tokens.next().is_none());

            Parse { green_node: builder.finish(), errors: self.errors }
        }
    }  


    let line_index = LineIndex::new(text);
    let lex = lexer::lex(text);
    let tokens = lex.into_iter().map(|res| res.unwrap()).collect();
    let mut parser = Parser { tokens, pos: 0, line_index, fuel: Cell::new(256), events: Vec::new(), errors: Vec::new() };
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
    fn test_list_literal_empty() {
        expect_tree("func test() { val x = []; }", &[
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
            "        ExprList@16..18",
            "          LeftBracket@16..17 \"[\"",
            "          RightBracket@17..18 \"]\"",
            "        Semicolon@18..19 \";\"",
            "      RightBrace@19..20 \"}\""
        ])
    }

    #[test]
    fn test_list_literal_with_elements() {
        expect_tree("func test() { val x = [1, 2, 3]; }", &[
            "File@0..25",
            "  Func@0..25",
            "    FuncKwd@0..4 \"func\"",
            "    Identifier@4..8 \"test\"",
            "    ParamList@8..10",
            "      LeftParen@8..9 \"(\"",
            "      RightParen@9..10 \")\"",
            "    Block@10..25",
            "      LeftBrace@10..11 \"{\"",
            "      StmtVal@11..24",
            "        ValKwd@11..14 \"val\"",
            "        Identifier@14..15 \"x\"",
            "        Assign@15..16 \"=\"",
            "        ExprList@16..23",
            "          LeftBracket@16..17 \"[\"",
            "          ExprLiteral@17..18",
            "            DecIntLiteral@17..18 \"1\"",
            "          Comma@18..19 \",\"",
            "          ExprLiteral@19..20",
            "            DecIntLiteral@19..20 \"2\"",
            "          Comma@20..21 \",\"",
            "          ExprLiteral@21..22",
            "            DecIntLiteral@21..22 \"3\"",
            "          RightBracket@22..23 \"]\"",
            "        Semicolon@23..24 \";\"",
            "      RightBrace@24..25 \"}\""
        ])
    }

    #[test]
    fn test_list_literal_nested() {
        expect_tree("func test() { val x = [[1, 2], [3]]; }", &[
            "File@0..29",
            "  Func@0..29",
            "    FuncKwd@0..4 \"func\"",
            "    Identifier@4..8 \"test\"",
            "    ParamList@8..10",
            "      LeftParen@8..9 \"(\"",
            "      RightParen@9..10 \")\"",
            "    Block@10..29",
            "      LeftBrace@10..11 \"{\"",
            "      StmtVal@11..28",
            "        ValKwd@11..14 \"val\"",
            "        Identifier@14..15 \"x\"",
            "        Assign@15..16 \"=\"",
            "        ExprList@16..27",
            "          LeftBracket@16..17 \"[\"",
            "          ExprList@17..22",
            "            LeftBracket@17..18 \"[\"",
            "            ExprLiteral@18..19",
            "              DecIntLiteral@18..19 \"1\"",
            "            Comma@19..20 \",\"",
            "            ExprLiteral@20..21",
            "              DecIntLiteral@20..21 \"2\"",
            "            RightBracket@21..22 \"]\"",
            "          Comma@22..23 \",\"",
            "          ExprList@23..26",
            "            LeftBracket@23..24 \"[\"",
            "            ExprLiteral@24..25",
            "              DecIntLiteral@24..25 \"3\"",
            "            RightBracket@25..26 \"]\"",
            "          RightBracket@26..27 \"]\"",
            "        Semicolon@27..28 \";\"",
            "      RightBrace@28..29 \"}\""
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
            "        ExprUnary@16..18",
            "          Minus@16..17 \"-\"",
            "          ExprLiteral@17..18",
            "            DecIntLiteral@17..18 \"5\"",
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

    fn test_files_in_directory_parse(dir: &str) {
        use std::fs;
        use std::path::Path;

        let test_dir = Path::new(dir);
        
        if !test_dir.exists() {
            println!("Directory not found, skipping test: {}", dir);
            return;
        }

        let entries = fs::read_dir(test_dir)
            .expect(&format!("Failed to read directory: {}", dir));

        for entry in entries {
            let entry = entry.expect("Failed to read directory entry");
            let path = entry.path();
            
            if path.extension().and_then(|s| s.to_str()) == Some("aria") {
                let filename = path.file_name().unwrap().to_str().unwrap();

                println!("Parsing {}", filename);
                            
                let content = fs::read_to_string(&path)
                    .expect(&format!("Failed to read file: {}", filename));
                
                let parse_result = parse(&content);
                
                if !parse_result.errors.is_empty() {
                    println!("\n{} has parse errors:", filename);
                    for error in &parse_result.errors {
                        println!("  {}", error);
                    }
                    panic!("Parse errors found in {}", filename);
                }
            }
        }
    }

    #[test]
    fn test_assignment_basic() {
        expect_tree("func test() { x = 5; }", &[
            "File@0..16",
            "  Func@0..16",
            "    FuncKwd@0..4 \"func\"",
            "    Identifier@4..8 \"test\"",
            "    ParamList@8..10",
            "      LeftParen@8..9 \"(\"",
            "      RightParen@9..10 \")\"",
            "    Block@10..16",
            "      LeftBrace@10..11 \"{\"",
            "      StmtExpr@11..15",
            "        ExprAssign@11..14",
            "          ExprName@11..12",
            "            Identifier@11..12 \"x\"",
            "          Assign@12..13 \"=\"",
            "          ExprLiteral@13..14",
            "            DecIntLiteral@13..14 \"5\"",
            "        Semicolon@14..15 \";\"",
            "      RightBrace@15..16 \"}\""
        ])
    }

    #[test]
    fn test_assignment_compound() {
        expect_tree("func test() { x += 5; }", &[
            "File@0..17",
            "  Func@0..17",
            "    FuncKwd@0..4 \"func\"",
            "    Identifier@4..8 \"test\"",
            "    ParamList@8..10",
            "      LeftParen@8..9 \"(\"",
            "      RightParen@9..10 \")\"",
            "    Block@10..17",
            "      LeftBrace@10..11 \"{\"",
            "      StmtExpr@11..16",
            "        ExprAssign@11..15",
            "          ExprName@11..12",
            "            Identifier@11..12 \"x\"",
            "          PlusAssign@12..14 \"+=\"",
            "          ExprLiteral@14..15",
            "            DecIntLiteral@14..15 \"5\"",
            "        Semicolon@15..16 \";\"",
            "      RightBrace@16..17 \"}\""
        ])
    }

    #[test]
    fn test_assignment_all_compound_operators() {
        expect_tree("func test() { x -= 1; y *= 2; z /= 3; w %= 4; }", &[
            "File@0..32",
            "  Func@0..32",
            "    FuncKwd@0..4 \"func\"",
            "    Identifier@4..8 \"test\"",
            "    ParamList@8..10",
            "      LeftParen@8..9 \"(\"",
            "      RightParen@9..10 \")\"",
            "    Block@10..32",
            "      LeftBrace@10..11 \"{\"",
            "      StmtExpr@11..16",
            "        ExprAssign@11..15",
            "          ExprName@11..12",
            "            Identifier@11..12 \"x\"",
            "          MinusAssign@12..14 \"-=\"",
            "          ExprLiteral@14..15",
            "            DecIntLiteral@14..15 \"1\"",
            "        Semicolon@15..16 \";\"",
            "      StmtExpr@16..21",
            "        ExprAssign@16..20",
            "          ExprName@16..17",
            "            Identifier@16..17 \"y\"",
            "          StarAssign@17..19 \"*=\"",
            "          ExprLiteral@19..20",
            "            DecIntLiteral@19..20 \"2\"",
            "        Semicolon@20..21 \";\"",
            "      StmtExpr@21..26",
            "        ExprAssign@21..25",
            "          ExprName@21..22",
            "            Identifier@21..22 \"z\"",
            "          SlashAssign@22..24 \"/=\"",
            "          ExprLiteral@24..25",
            "            DecIntLiteral@24..25 \"3\"",
            "        Semicolon@25..26 \";\"",
            "      StmtExpr@26..31",
            "        ExprAssign@26..30",
            "          ExprName@26..27",
            "            Identifier@26..27 \"w\"",
            "          PercentAssign@27..29 \"%=\"",
            "          ExprLiteral@29..30",
            "            DecIntLiteral@29..30 \"4\"",
            "        Semicolon@30..31 \";\"",
            "      RightBrace@31..32 \"}\""
        ])
    }

    #[test]
    fn test_assignment_precedence() {
        expect_tree("func test() { x = y + z; }", &[
            "File@0..18",
            "  Func@0..18",
            "    FuncKwd@0..4 \"func\"",
            "    Identifier@4..8 \"test\"",
            "    ParamList@8..10",
            "      LeftParen@8..9 \"(\"",
            "      RightParen@9..10 \")\"",
            "    Block@10..18",
            "      LeftBrace@10..11 \"{\"",
            "      StmtExpr@11..17",
            "        ExprAssign@11..16",
            "          ExprName@11..12",
            "            Identifier@11..12 \"x\"",
            "          Assign@12..13 \"=\"",
            "          ExprBinary@13..16",
            "            ExprName@13..14",
            "              Identifier@13..14 \"y\"",
            "            Plus@14..15 \"+\"",
            "            ExprName@15..16",
            "              Identifier@15..16 \"z\"",
            "        Semicolon@16..17 \";\"",
            "      RightBrace@17..18 \"}\""
        ])
    }

    #[test]
    fn test_assignment_right_associative() {
        expect_tree("func test() { x = y = z; }", &[
            "File@0..18",
            "  Func@0..18",
            "    FuncKwd@0..4 \"func\"",
            "    Identifier@4..8 \"test\"",
            "    ParamList@8..10",
            "      LeftParen@8..9 \"(\"",
            "      RightParen@9..10 \")\"",
            "    Block@10..18",
            "      LeftBrace@10..11 \"{\"",
            "      StmtExpr@11..17",
            "        ExprAssign@11..16",
            "          ExprName@11..12",
            "            Identifier@11..12 \"x\"",
            "          Assign@12..13 \"=\"",
            "          ExprAssign@13..16",
            "            ExprName@13..14",
            "              Identifier@13..14 \"y\"",
            "            Assign@14..15 \"=\"",
            "            ExprName@15..16",
            "              Identifier@15..16 \"z\"",
            "        Semicolon@16..17 \";\"",
            "      RightBrace@17..18 \"}\""
        ])
    }

    #[test]
    fn test_simple_binary_expr() {
        expect_tree("func test() { val x = y-1; }", &[
            "File@0..21",
            "  Func@0..21",
            "    FuncKwd@0..4 \"func\"",
            "    Identifier@4..8 \"test\"",
            "    ParamList@8..10",
            "      LeftParen@8..9 \"(\"",
            "      RightParen@9..10 \")\"",
            "    Block@10..21",
            "      LeftBrace@10..11 \"{\"",
            "      StmtVal@11..20",
            "        ValKwd@11..14 \"val\"",
            "        Identifier@14..15 \"x\"",
            "        Assign@15..16 \"=\"",
            "        ExprBinary@16..19",
            "          ExprName@16..17",
            "            Identifier@16..17 \"y\"",
            "          Minus@17..18 \"-\"",
            "          ExprLiteral@18..19",
            "            DecIntLiteral@18..19 \"1\"",
            "        Semicolon@19..20 \";\"",
            "      RightBrace@20..21 \"}\""
        ])
    }

    #[test]
    fn test_example_files_parse_without_errors() {
        test_files_in_directory_parse("../examples");
    }

    // #[test]
    // fn test_files_parse_without_errors() {
    //     test_files_in_directory_parse("../tests");
    // }
}