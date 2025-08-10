use logos::Logos;

#[derive(Logos, Debug, Clone, PartialEq)]
#[logos(skip r"[ \t\n\f]+")]
#[logos(skip r"#[^\n]*")]
pub enum Token {
    #[token("assert")]
    Assert,
    #[token("break")]
    Break,
    #[token("case")]
    Case,
    #[token("catch")]
    Catch,
    #[token("continue")]
    Continue,
    #[token("else")]
    Else,
    #[token("elsif")]
    Elsif,
    #[token("enum")]
    Enum,
    #[token("extension")]
    Extension,
    #[token("flag")]
    Flag,
    #[token("for")]
    For,
    #[token("from")]
    From,
    #[token("func")]
    Func,
    #[token("guard")]
    Guard,
    #[token("if")]
    If,
    #[token("import")]
    Import,
    #[token("in")]
    In,
    #[token("include")]
    Include,
    #[token("instance")]
    Instance,
    #[token("isa")]
    Isa,
    #[token("match")]
    Match,
    #[token("mixin")]
    Mixin,
    #[token("operator")]
    Operator,
    #[token("return")]
    Return,
    #[token("reverse")]
    Reverse,
    #[token("struct")]
    Struct,
    #[token("throw")]
    Throw,
    #[token("try")]
    Try,
    #[token("type")]
    Type,
    #[token("val")]
    Val,
    #[token("while")]
    While,
    #[token("and")]
    And,

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

    #[regex(r"[a-zA-Z_$][a-zA-Z0-9_$]*")]
    Identifier,

    #[token("u-")]
    UnaryMinus,
    #[token("()")]
    CallOperator,
    #[token("[]=")]
    IndexAssignOperator,
    #[token("[]")]
    IndexOperator,
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keywords() {
        let lexer = Token::lexer("func if else while");
        let tokens: Vec<_> = lexer.collect();
        
        assert_eq!(tokens.len(), 4);
        assert_eq!(tokens[0], Ok(Token::Func));
        assert_eq!(tokens[1], Ok(Token::If));
        assert_eq!(tokens[2], Ok(Token::Else));
        assert_eq!(tokens[3], Ok(Token::While));
    }

    #[test]
    fn test_identifiers() {
        let lexer = Token::lexer("myVar _private $special");
        let tokens: Vec<_> = lexer.collect();
        
        assert_eq!(tokens.len(), 3);
        assert!(matches!(tokens[0], Ok(Token::Identifier)));
        assert!(matches!(tokens[1], Ok(Token::Identifier)));
        assert!(matches!(tokens[2], Ok(Token::Identifier)));
    }

    #[test]
    fn test_literals() {
        let lexer = Token::lexer(r#"42 0x1A 0o77 0b101 3.14 "hello" 'world'"#);
        let tokens: Vec<_> = lexer.collect();
        
        assert_eq!(tokens.len(), 7);
        assert!(matches!(tokens[0], Ok(Token::DecIntLiteral)));
        assert!(matches!(tokens[1], Ok(Token::HexIntLiteral)));
        assert!(matches!(tokens[2], Ok(Token::OctIntLiteral)));
        assert!(matches!(tokens[3], Ok(Token::BinIntLiteral)));
        assert!(matches!(tokens[4], Ok(Token::FloatLiteral)));
        assert!(matches!(tokens[5], Ok(Token::StringLiteral)));
        assert!(matches!(tokens[6], Ok(Token::StringLiteral)));
    }

    #[test]
    fn test_operators() {
        let lexer = Token::lexer("+ - * / % == != <= >= << >> && ||");
        let tokens: Vec<_> = lexer.collect();
        
        assert_eq!(tokens.len(), 13);
        assert_eq!(tokens[0], Ok(Token::Plus));
        assert_eq!(tokens[1], Ok(Token::Minus));
        assert_eq!(tokens[2], Ok(Token::Star));
        assert_eq!(tokens[3], Ok(Token::Slash));
        assert_eq!(tokens[4], Ok(Token::Percent));
        assert_eq!(tokens[5], Ok(Token::Equal));
        assert_eq!(tokens[6], Ok(Token::NotEqual));
        assert_eq!(tokens[7], Ok(Token::LessEqual));
        assert_eq!(tokens[8], Ok(Token::GreaterEqual));
        assert_eq!(tokens[9], Ok(Token::LeftShift));
        assert_eq!(tokens[10], Ok(Token::RightShift));
        assert_eq!(tokens[11], Ok(Token::LogicalAnd));
        assert_eq!(tokens[12], Ok(Token::LogicalOr));
    }

    #[test]
    fn test_comments_and_whitespace() {
        let lexer = Token::lexer("func # this is a comment\n  main");
        let tokens: Vec<_> = lexer.collect();
        
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0], Ok(Token::Func));
        assert!(matches!(tokens[1], Ok(Token::Identifier)));
    }

    #[test]
    fn test_complex_expression() {
        let lexer = Token::lexer("val x = func(a, b) { return a + b; }");
        let tokens: Vec<_> = lexer.collect();
        
        assert!(!tokens.is_empty());
        assert_eq!(tokens[0], Ok(Token::Val));
    }

    #[test]
    fn test_example_files_lex_without_errors() {
        use std::fs;
        use std::path::Path;

        let examples_dir = Path::new("../examples");
        
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
                
                let lexer = Token::lexer(&content);
                let tokens: Vec<_> = lexer.collect();
                
                let errors: Vec<_> = tokens.iter()
                    .filter(|token| token.is_err())
                    .collect();

                assert!(errors.is_empty());
            }
        }
    }
}