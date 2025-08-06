/*
* TOKEN
*/

use std::fmt::Display;

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum Token_Type {
    TT_INT_TYPE,
    TT_FLOAT_TYPE,
    TT_BOOL_TYPE,
    TT_INT,
    TT_FLOAT,
    TT_BOOL,
    // TT_LQUOTE,
    // TT_RQUOTE,
    TT_QUOTE,
    TT_PLUS,
    TT_MINUS,
    TT_DIV,
    TT_MOD,
    TT_MUL,
    TT_EXPO,
    TT_AND,
    TT_OR,
    TT_NOT,
    TT_LPAREN,
    TT_RPAREN,
    TT_ASSIGN,
    TT_LESS,
    TT_GREAT,
    TT_LESS_EQUAL,
    TT_GREAT_EQUAL,
    TT_EQUAL,
    TT_SEMI,
    TT_NAME,
}

impl Token_Type {
    fn as_string(&self) -> &'static str {
        match self {
            Token_Type::TT_INT_TYPE => "INT_TYPE",
            Token_Type::TT_FLOAT_TYPE => "FLOAT_TYPE",
            Token_Type::TT_BOOL_TYPE => "BOOL_TYPE",
            Token_Type::TT_INT => "INT",
            Token_Type::TT_FLOAT => "FLOAT",
            Token_Type::TT_BOOL => "BOOL",
            // Token_Type::TT_LQUOTE => "LQUOTE",
            // Token_Type::TT_RQUOTE => "RQUOTE",
            Token_Type::TT_QUOTE => "QOUTE",
            Token_Type::TT_PLUS => "PLUS",
            Token_Type::TT_MINUS => "MINUS",
            Token_Type::TT_DIV => "DIV",
            Token_Type::TT_MOD => "MOD",
            Token_Type::TT_MUL => "MUL",
            Token_Type::TT_EXPO => "EXPO",
            Token_Type::TT_AND => "AND",
            Token_Type::TT_OR => "OR",
            Token_Type::TT_NOT => "NOT",
            Token_Type::TT_LPAREN => "LPAREN",
            Token_Type::TT_RPAREN => "RPAREN",
            Token_Type::TT_ASSIGN => "ASSIGN",
            Token_Type::TT_LESS => "LESS",
            Token_Type::TT_GREAT => "GREAT",
            Token_Type::TT_LESS_EQUAL => "LESS_EQUAL",
            Token_Type::TT_GREAT_EQUAL => "GREAT_EQUAL",
            Token_Type::TT_EQUAL => "EQUAL",
            Token_Type::TT_SEMI => "SEMI",
            Token_Type::TT_NAME => "NAME",
        }
    }
}

#[derive(Debug)]
pub enum ValueType {
    Int(i32),
    Float(f64),
    String(String),
    Bool(bool),
}

impl Display for ValueType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValueType::Int(i) => write!(f, "{i}"),
            ValueType::Float(d) => write!(f, "{d}"),
            ValueType::String(s) => write!(f, "{s}"),
            ValueType::Bool(b) => write!(f, "{b}"),
        }
    }
}

#[derive(Debug)]
pub struct Token {
    token_type: Token_Type,
    value: Option<ValueType>,
}

impl Token {
    pub fn new(token_type: Token_Type, value: Option<ValueType>) -> Token {
        Self { token_type, value }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.value {
            Some(a) => write!(f, "{}:{}", self.token_type.as_string(), a),
            None => write!(f, "{}", self.token_type.as_string()),
        }
    }
}

pub struct TokenList(pub Vec<Token>);

impl TokenList {
    pub fn push(&mut self, token: Token) {
        self.0.push(token);
    }
    pub fn new() -> Self {
        TokenList(Vec::new())
    }
}

impl Default for TokenList {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for TokenList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for token in &self.0 {
            write!(f, "[{token}] ")?;
        }
        Ok(())
    }
}

pub mod lexer;

pub fn build(text: String) -> TokenList {
    let mut lexer = lexer::Lexer::new(text);
    lexer.make_tokens();
    lexer.tokens
}
