/*
* TOKEN
*/

#[allow(non_camel_case_types)]
#[derive(Debug)]
enum Token_Type {
    TT_INT_TYPE,
    TT_FLOAT_TYPE,
    TT_BOOL_TYPE,
    TT_INT,
    TT_FLOAT,
    TT_BOOL,
    TT_LQUOTE,
    TT_RQUOTE,
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
            Token_Type::TT_LQUOTE => "LQUOTE",
            Token_Type::TT_RQUOTE => "RQUOTE",
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
enum ValueType {
    Int(i32),
    Float(f64),
    String(String),
    Bool(bool),
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

pub mod lexer;

pub fn build(text: String) -> Vec<Token> {
    let mut lexer = lexer::Lexer::new(text);
    lexer.make_tokens();
    return lexer.tokens;
}
