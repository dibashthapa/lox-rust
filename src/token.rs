use crate::{token_type::TokenType, value::Value};

#[derive(Clone, PartialEq, Debug)]
pub struct Token {
    pub type_: TokenType,
    pub lexeme: String,
    pub literal: Option<Value>,
    pub line: usize,
}

impl Token {
    pub fn new(type_: TokenType, lexeme: &str, literal: Option<Value>, line: usize) -> Self {
        Self {
            type_,
            lexeme: lexeme.to_string(),
            literal,
            line,
        }
    }

    pub fn is_identifier(&self) -> bool {
        self.type_ == TokenType::Identifier
    }

    pub fn eof(line: usize) -> Token {
        Token {
            type_: TokenType::Eof,
            lexeme: "".to_string(),
            literal: None,
            line,
        }
    }
}
