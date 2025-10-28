pub mod lexer;
pub mod parser;
pub mod document;
pub use lexer::SyntaxKind;
pub use parser::parse;