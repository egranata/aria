use logos::Logos;

#[derive(Logos, Debug, Clone, PartialEq, Hash, Eq, PartialOrd, Ord, Copy)]
#[logos(skip r"[ \t\n\f]+")]
#[logos(skip r"#[^\n]*")]
#[repr(u16)]
pub enum SyntaxKind {
    #[token("assert")]
    AssertKwd,
    #[token("break")]
    BreakKwd,
    #[token("case")]
    CaseKwd,
    #[token("catch")]
    CatchKwd,
    #[token("continue")]
    ContinueKwd,
    #[token("else")]
    ElseKwd,
    #[token("elsif")]
    ElsifKwd,
    #[token("enum")]
    EnumKwd,
    #[token("extension")]
    ExtensionKwd,
    #[token("flag")]
    FlagKwd,
    #[token("for")]
    ForKwd,
    #[token("from")]
    FromKwd,
    #[token("func")]
    FuncKwd,
    #[token("guard")]
    GuardKwd,
    #[token("if")]
    IfKwd,
    #[token("import")]
    ImportKwd,
    #[token("in")]
    InKwd,
    #[token("include")]
    IncludeKwd,
    #[token("instance")]
    InstanceKwd,
    #[token("isa")]
    IsaKwd,
    #[token("match")]
    MatchKwd,
    #[token("mixin")]
    MixinKwd,
    #[token("operator")]
    OperatorKwd,
    #[token("return")]
    ReturnKwd,
    #[token("reverse")]
    ReverseKwd,
    #[token("struct")]
    StructKwdKwd,
    #[token("throw")]
    ThrowKwd,
    #[token("try")]
    TryKwd,
    #[token("type")]
    TypeKwd,
    #[token("val")]
    ValKwd,
    #[token("while")]
    WhileKwd,
    #[token("and")]
    AndKwd,

    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Star,
    #[token("/")]
    Slash,
    #[token("%")]
    Percent,
    #[token("<<")]
    LeftShift,
    #[token(">>")]
    RightShift,
    #[token("==")]
    Equal,
    #[token("!=")]
    NotEqual,
    #[token("<=")]
    LessEqual,
    #[token(">=")]
    GreaterEqual,
    #[token("<")]
    Less,
    #[token(">")]
    Greater,
    #[token("&&")]
    LogicalAnd,
    #[token("||")]
    LogicalOr,
    #[token("&")]
    BitwiseAnd,
    #[token("|")]
    BitwiseOr,
    #[token("^")]
    BitwiseXor,
    #[token("!")]
    Not,
    #[token("=")]
    Assign,
    #[token("+=")]
    PlusAssign,
    #[token("-=")]
    MinusAssign,
    #[token("*=")]
    StarAssign,
    #[token("/=")]
    SlashAssign,
    #[token("%=")]
    PercentAssign,
    #[token("?")]
    Question,
    #[token(":")]
    Colon,
    #[token("::")]
    DoubleColon,
    #[token("=>")]
    Arrow,
    #[token("...")]
    Ellipsis,

    #[token("(")]
    LeftParen,
    #[token(")")]
    RightParen,
    #[token("[")]
    LeftBracket,
    #[token("]")]
    RightBracket,
    #[token("{")]
    LeftBrace,
    #[token("}")]
    RightBrace,
    #[token(",")]
    Comma,
    #[token(";")]
    Semicolon,
    #[token(".")]
    Dot,

    #[token("true")]
    TrueKwd,
    #[token("false")]
    FalseKwd,

    #[regex(r"0x[0-9a-fA-F]+(_[0-9a-fA-F]+)*")]
    HexIntLiteral,
    
    #[regex(r"0o[0-7]+(_[0-7]+)*")]
    OctIntLiteral,
    
    #[regex(r"0b[01]+(_[01]+)*")]
    BinIntLiteral,
    
    #[regex(r"-?[0-9]+(_[0-9]+)*")]
    DecIntLiteral,
    
    #[regex(r"-?[0-9]+\.[0-9]+([eE][+-]?[0-9]+)?f?")]
    FloatLiteral,
    
    #[regex(r#""([^"\\]|\\.)*""#)]
    #[regex(r#"'([^'\\]|\\.)*'"#)]
    StringLiteral,

    #[regex(r#"[\p{XID_Start}_$][\p{XID_Continue}_$]*|\p{Emoji}[\p{Emoji_Modifier}\p{Emoji_Component}]*"#, priority = 1)]
    Identifier,

    #[token("u-")]
    UnaryMinus,

    // Error token for unrecognized input
    Error,

    // compound types
    File,
    ErrorTree,
    Func,
    Block,
    ParamList,
    Param,
    StmtVal,
    StmtReturn,
    StmtExpr,
    ExprName,
    ExprCall,
    ExprBinary,
    ExprUnary,
    ExprParen,
    ExprLiteral,
    ExprMember,
    ExprIndex,
    ExprTernary,
    ExprList,
    Mixin,
    Struct,
    Ext,
    ArgList,
    Arg,
    Eof
}

pub fn lex(s: &str) -> Vec<Result<(SyntaxKind, &str), LexError>> {
    let mut lexer = SyntaxKind::lexer(s);
    let mut tokens = Vec::new();
    
    while let Some(token_result) = lexer.next() {
        let slice = lexer.slice();
        match token_result {
            Ok(token) => tokens.push(Ok((token, slice))),
            Err(_) => {
                let span = lexer.span();
                tokens.push(Err(LexError {
                    message: format!("Unexpected character(s): '{}'", slice),
                    span,
                    text: slice.to_string(),
                }));
            }
        }
    }
    
    tokens
}

#[derive(Debug, Clone, PartialEq)]
pub struct LexError {
    pub message: String,
    pub span: std::ops::Range<usize>,
    pub text: String,
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keywords() {
        let lexer = SyntaxKind::lexer("func if else while");
        let tokens: Vec<_> = lexer.collect();
        
        assert_eq!(tokens.len(), 4);
        assert_eq!(tokens[0], Ok(SyntaxKind::FuncKwd));
        assert_eq!(tokens[1], Ok(SyntaxKind::IfKwd));
        assert_eq!(tokens[2], Ok(SyntaxKind::ElseKwd));
        assert_eq!(tokens[3], Ok(SyntaxKind::WhileKwd));
    }

    #[test]
    fn test_identifiers() {
        let lexer = SyntaxKind::lexer("myVar _private $special");
        let tokens: Vec<_> = lexer.collect();
        
        assert_eq!(tokens.len(), 3);
        assert!(matches!(tokens[0], Ok(SyntaxKind::Identifier)));
        assert!(matches!(tokens[1], Ok(SyntaxKind::Identifier)));
        assert!(matches!(tokens[2], Ok(SyntaxKind::Identifier)));
    }

    #[test]
    fn test_literals() {
        let lexer = SyntaxKind::lexer(r#"42 0x1A 0o77 0b101 3.14 "hello" 'world'"#);
        let tokens: Vec<_> = lexer.collect();
        
        assert_eq!(tokens.len(), 7);
        assert!(matches!(tokens[0], Ok(SyntaxKind::DecIntLiteral)));
        assert!(matches!(tokens[1], Ok(SyntaxKind::HexIntLiteral)));
        assert!(matches!(tokens[2], Ok(SyntaxKind::OctIntLiteral)));
        assert!(matches!(tokens[3], Ok(SyntaxKind::BinIntLiteral)));
        assert!(matches!(tokens[4], Ok(SyntaxKind::FloatLiteral)));
        assert!(matches!(tokens[5], Ok(SyntaxKind::StringLiteral)));
        assert!(matches!(tokens[6], Ok(SyntaxKind::StringLiteral)));
    }

    #[test]
    fn test_operators() {
        let lexer = SyntaxKind::lexer("+ - * / % == != <= >= << >> && ||");
        let tokens: Vec<_> = lexer.collect();
        
        assert_eq!(tokens.len(), 13);
        assert_eq!(tokens[0], Ok(SyntaxKind::Plus));
        assert_eq!(tokens[1], Ok(SyntaxKind::Minus));
        assert_eq!(tokens[2], Ok(SyntaxKind::Star));
        assert_eq!(tokens[3], Ok(SyntaxKind::Slash));
        assert_eq!(tokens[4], Ok(SyntaxKind::Percent));
        assert_eq!(tokens[5], Ok(SyntaxKind::Equal));
        assert_eq!(tokens[6], Ok(SyntaxKind::NotEqual));
        assert_eq!(tokens[7], Ok(SyntaxKind::LessEqual));
        assert_eq!(tokens[8], Ok(SyntaxKind::GreaterEqual));
        assert_eq!(tokens[9], Ok(SyntaxKind::LeftShift));
        assert_eq!(tokens[10], Ok(SyntaxKind::RightShift));
        assert_eq!(tokens[11], Ok(SyntaxKind::LogicalAnd));
        assert_eq!(tokens[12], Ok(SyntaxKind::LogicalOr));
    }

    #[test]
    fn test_comments_and_whitespace() {
        let lexer = SyntaxKind::lexer("func # this is a comment\n  main");
        let tokens: Vec<_> = lexer.collect();
        
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0], Ok(SyntaxKind::FuncKwd));
        assert!(matches!(tokens[1], Ok(SyntaxKind::Identifier)));
    }

    #[test]
    fn test_complex_expression() {
        let lexer = SyntaxKind::lexer("val x = func(a, b) { return a + b; }");
        let tokens: Vec<_> = lexer.collect();
        
        assert!(!tokens.is_empty());
        assert_eq!(tokens[0], Ok(SyntaxKind::ValKwd));
    }

    fn test_files_in_directory(dir: &str) {
        use std::fs;
        use std::path::Path;

        let examples_dir = Path::new(dir);
        
        if !examples_dir.exists() {
            println!("Examples directory not found, skipping test");
            return;
        }

        let entries = fs::read_dir(examples_dir)
            .expect("Failed to read examples directory");

        for entry in entries {
            let entry = entry.expect("Failed to read directory entry");
            let path = entry.path();
            
            if path.extension().and_then(|s| s.to_str()) == Some("aria") {
                let filename = path.file_name().unwrap().to_str().unwrap();
                            
                let content = fs::read_to_string(&path)
                    .expect(&format!("Failed to read file: {}", filename));
                
                let tokens = lex(&content);
                
                let errors: Vec<_> = tokens.iter()
                    .filter_map(|token| token.as_ref().err())
                    .collect();
                
                if !errors.is_empty() {
                    println!("\n{} has lexer errors:", filename);
                    for error in &errors {
                        println!("  {} at position {}..{}", error.message, error.span.start, error.span.end);
                    }
                }
                
                assert!(errors.is_empty());
            }
        }
    }

    #[test]
    fn test_example_files_lex_without_errors() {
        test_files_in_directory("../examples");
    }

    #[test]
    fn test_files_lex_without_errors() {
        test_files_in_directory("../tests");
    }

    #[test]
    fn test_error_reporting() {
        let tokens = lex("func @ invalid # comment");
        
        let errors: Vec<_> = tokens.iter().filter_map(|t| t.as_ref().err()).collect();
        
        if !errors.is_empty() {
            println!("Lexer errors found:");
            for error in &errors {
                println!("  {} at position {}..{}", error.message, error.span.start, error.span.end);
            }
        }
        
        let successful_tokens: Vec<_> = tokens.iter().filter_map(|t| t.as_ref().ok()).collect();
        assert!(!successful_tokens.is_empty());
        assert_eq!(successful_tokens[0].0, SyntaxKind::FuncKwd);
    }
}