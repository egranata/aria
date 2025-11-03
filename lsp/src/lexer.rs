// SPDX-License-Identifier: Apache-2.0
use logos::Logos;

#[derive(Logos, Debug, Clone, PartialEq, Hash, Eq, PartialOrd, Ord, Copy)]
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
    StructKwd,
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
    #[token("u-")]
    UnaryMinus,
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
    Pipe,
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

    #[regex(r"[0-9]+(_[0-9]+)*")]
    DecIntLiteral,

    #[regex(r"[0-9]+\.[0-9]+([eE][+-]?[0-9]+)?f?")]
    FloatLiteral,

    #[regex(r#""([^"\\]|\\.)*""#)]
    #[regex(r#"'([^'\\]|\\.)*'"#)]
    StringLiteral,

    #[regex(
        r#"[\p{XID_Start}\p{Emoji_Presentation}_$][\p{XID_Continue}\p{Emoji_Presentation}_$]*"#,
        priority = 1
    )]
    Identifier,

    // trivia
    #[regex(r"[ \t\n\f]+")]
    Whitespace,
    #[regex(r"#[^\n]*")]
    LineComment,

    // Error token for unrecognized input
    Error,

    // compound types
    File,
    ErrorTree,
    Func,
    Lambda,
    Block,
    ParamList,
    Param,
    StmtVal,
    StmtReturn,
    StmtExpr,
    StmtIf,
    StmtFor,
    StmtMatch,
    StmtWhile,
    StmtImport,
    StmtAssert,
    ExprName,
    ExprCall,
    ExprBinary,
    ExprUnary,
    ExprParen,
    ExprLiteral,
    ExprMember,
    ExprIndex,
    ExprTernary,
    ExprAssign,
    ExprType,
    ExprNonNull,
    ExprNullish,
    Mixin,
    MixinInclude,
    MixinEntry,
    Struct,
    StructEntry,
    Enum,
    EnumCase,
    EnumEntry,
    Extension,
    Operator,
    Guard,
    TryBlock,
    MatchRule,
    MatchPattern,
    IdentList,
    QualifiedIdent,
    ImportPath,
    ArgList,
    ListLiteral,
    ModuleFlag,
    Eof,
}

pub fn lex(s: &str) -> Vec<Result<(SyntaxKind, &str, logos::Span), LexError>> {
    let mut lexer = SyntaxKind::lexer(s);
    let mut tokens = Vec::new();

    while let Some(token_result) = lexer.next() {
        let slice = lexer.slice();
        match token_result {
            Ok(token) => tokens.push(Ok((token, slice, lexer.span()))),
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

    fn non_trivia_tokens(input: &str) -> Vec<SyntaxKind> {
        use SyntaxKind::*;
        lex(input)
            .into_iter()
            .filter_map(|r| r.ok())
            .map(|(k, _, _)| k)
            .filter(|k| !matches!(k, Whitespace | LineComment))
            .collect()
    }

    #[test]
    fn test_keywords() {
        let tokens = non_trivia_tokens("func if else while");
        assert_eq!(tokens.len(), 4);
        assert_eq!(tokens[0], SyntaxKind::FuncKwd);
        assert_eq!(tokens[1], SyntaxKind::IfKwd);
        assert_eq!(tokens[2], SyntaxKind::ElseKwd);
        assert_eq!(tokens[3], SyntaxKind::WhileKwd);
    }

    #[test]
    fn test_identifiers() {
        let tokens = non_trivia_tokens("myVar _private $special");
        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0], SyntaxKind::Identifier);
        assert_eq!(tokens[1], SyntaxKind::Identifier);
        assert_eq!(tokens[2], SyntaxKind::Identifier);
    }

    #[test]
    fn test_literals() {
        let tokens = non_trivia_tokens(r#"42 0x1A 0o77 0b101 3.14 "hello" 'world'"#);
        assert_eq!(tokens.len(), 7);
        assert_eq!(tokens[0], SyntaxKind::DecIntLiteral);
        assert_eq!(tokens[1], SyntaxKind::HexIntLiteral);
        assert_eq!(tokens[2], SyntaxKind::OctIntLiteral);
        assert_eq!(tokens[3], SyntaxKind::BinIntLiteral);
        assert_eq!(tokens[4], SyntaxKind::FloatLiteral);
        assert_eq!(tokens[5], SyntaxKind::StringLiteral);
        assert_eq!(tokens[6], SyntaxKind::StringLiteral);
    }

    #[test]
    fn test_operators() {
        let tokens = non_trivia_tokens("+ - * / % == != <= >= << >> && ||");
        assert_eq!(tokens.len(), 13);
        assert_eq!(tokens[0], SyntaxKind::Plus);
        assert_eq!(tokens[1], SyntaxKind::Minus);
        assert_eq!(tokens[2], SyntaxKind::Star);
        assert_eq!(tokens[3], SyntaxKind::Slash);
        assert_eq!(tokens[4], SyntaxKind::Percent);
        assert_eq!(tokens[5], SyntaxKind::Equal);
        assert_eq!(tokens[6], SyntaxKind::NotEqual);
        assert_eq!(tokens[7], SyntaxKind::LessEqual);
        assert_eq!(tokens[8], SyntaxKind::GreaterEqual);
        assert_eq!(tokens[9], SyntaxKind::LeftShift);
        assert_eq!(tokens[10], SyntaxKind::RightShift);
        assert_eq!(tokens[11], SyntaxKind::LogicalAnd);
        assert_eq!(tokens[12], SyntaxKind::LogicalOr);
    }

    #[test]
    fn test_comments_and_whitespace() {
        let tokens = non_trivia_tokens("func # this is a comment\n  main");
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0], SyntaxKind::FuncKwd);
        assert_eq!(tokens[1], SyntaxKind::Identifier);
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

        println!("reading inside {dir}");

        let dir = Path::new(dir);

        if !dir.exists() {
            println!("Examples directory not found, skipping test");
            return;
        }

        let entries = fs::read_dir(dir).expect("Failed to read examples directory");

        for entry in entries {
            let entry = entry.expect("Failed to read directory entry");
            let path = entry.path();

            if path.extension().and_then(|s| s.to_str()) == Some("aria") {
                let filename = path.file_name().unwrap().to_str().unwrap();

                let content =
                    fs::read_to_string(&path).expect(&format!("Failed to read file: {}", filename));

                let tokens = lex(&content);

                let errors: Vec<_> = tokens
                    .iter()
                    .filter_map(|token| token.as_ref().err())
                    .collect();

                if !errors.is_empty() {
                    println!("\n{} has lexer errors:", filename);
                    for error in &errors {
                        println!(
                            "  {} at position {}..{}",
                            error.message, error.span.start, error.span.end
                        );
                    }
                }

                assert!(errors.is_empty());
            }

            if path.is_dir() {
                test_files_in_directory(path.to_path_buf().to_str().unwrap());
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
    fn test_std_lib_lex_without_errors() {
        test_files_in_directory("../lib");
    }

    #[test]
    fn test_std_lib_test_lex_without_errors() {
        test_files_in_directory("../lib-test");
    }

    #[test]
    fn test_error_reporting() {
        let tokens = lex("func @ invalid # comment");

        let errors: Vec<_> = tokens.iter().filter_map(|t| t.as_ref().err()).collect();

        if !errors.is_empty() {
            println!("Lexer errors found:");
            for error in &errors {
                println!(
                    "  {} at position {}..{}",
                    error.message, error.span.start, error.span.end
                );
            }
        }

        let successful_tokens: Vec<_> = tokens.iter().filter_map(|t| t.as_ref().ok()).collect();
        assert!(!successful_tokens.is_empty());
        assert_eq!(successful_tokens[0].0, SyntaxKind::FuncKwd);
    }

    #[test]
    fn test_star_number_separation() {
        let tokens = lex("*2");
        let successful_tokens: Vec<_> = tokens.iter().filter_map(|t| t.as_ref().ok()).collect();

        println!("Tokens for '*2': {:?}", successful_tokens);

        // Should be two tokens: Star and DecIntLiteral
        assert_eq!(successful_tokens.len(), 2);
        assert_eq!(successful_tokens[0].0, SyntaxKind::Star);
        assert_eq!(successful_tokens[1].0, SyntaxKind::DecIntLiteral);
        assert_eq!(successful_tokens[0].1, "*");
        assert_eq!(successful_tokens[1].1, "2");
    }

    #[test]
    fn test_operator_number_separation() {
        // Test various operator-number combinations
        let test_cases = vec![
            (
                "*2",
                vec![(SyntaxKind::Star, "*"), (SyntaxKind::DecIntLiteral, "2")],
            ),
            (
                "+3",
                vec![(SyntaxKind::Plus, "+"), (SyntaxKind::DecIntLiteral, "3")],
            ),
            (
                "-4",
                vec![(SyntaxKind::Minus, "-"), (SyntaxKind::DecIntLiteral, "4")],
            ),
            (
                "/5",
                vec![(SyntaxKind::Slash, "/"), (SyntaxKind::DecIntLiteral, "5")],
            ),
            (
                "%6",
                vec![(SyntaxKind::Percent, "%"), (SyntaxKind::DecIntLiteral, "6")],
            ),
        ];

        for (input, expected) in test_cases {
            let tokens = lex(input);
            let successful_tokens: Vec<_> = tokens.iter().filter_map(|t| t.as_ref().ok()).collect();

            assert_eq!(
                successful_tokens.len(),
                expected.len(),
                "Failed for input: {}",
                input
            );
            for (i, (expected_kind, expected_text)) in expected.iter().enumerate() {
                assert_eq!(
                    successful_tokens[i].0, *expected_kind,
                    "Failed kind for input: {} at position {}",
                    input, i
                );
                assert_eq!(
                    successful_tokens[i].1, *expected_text,
                    "Failed text for input: {} at position {}",
                    input, i
                );
            }
        }
    }

    #[test]
    fn test_emoji_identifiers() {
        // Test emoji identifiers still work
        let tokens = lex("😎 💯");
        let successful_tokens: Vec<_> = tokens
            .into_iter()
            .filter_map(|t| t.ok())
            .filter(|(k, _, _)| !matches!(k, SyntaxKind::Whitespace | SyntaxKind::LineComment))
            .collect();

        assert_eq!(successful_tokens.len(), 2);
        assert_eq!(successful_tokens[0].0, SyntaxKind::Identifier);
        assert_eq!(successful_tokens[1].0, SyntaxKind::Identifier);
        assert_eq!(successful_tokens[0].1, "😎");
        assert_eq!(successful_tokens[1].1, "💯");
    }
}
