/*
* TOKEN
*/

#[allow(non_camel_case_types)]
#[derive(Debug)]
enum Token_Types {
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

impl Token_Types {
    fn as_string(&self) -> &'static str {
        match self {
            Token_Types::TT_INT_TYPE => "INT_TYPE",
            Token_Types::TT_FLOAT_TYPE => "FLOAT_TYPE",
            Token_Types::TT_BOOL_TYPE => "BOOL_TYPE",
            Token_Types::TT_INT => "INT",
            Token_Types::TT_FLOAT => "FLOAT",
            Token_Types::TT_BOOL => "BOOL",
            Token_Types::TT_LQUOTE => "LQUOTE",
            Token_Types::TT_RQUOTE => "RQUOTE",
            Token_Types::TT_PLUS => "PLUS",
            Token_Types::TT_MINUS => "MINUS",
            Token_Types::TT_DIV => "DIV",
            Token_Types::TT_MOD => "MOD",
            Token_Types::TT_MUL => "MUL",
            Token_Types::TT_EXPO => "EXPO",
            Token_Types::TT_AND => "AND",
            Token_Types::TT_OR => "OR",
            Token_Types::TT_NOT => "NOT",
            Token_Types::TT_LPAREN => "LPAREN",
            Token_Types::TT_RPAREN => "RPAREN",
            Token_Types::TT_ASSIGN => "ASSIGN",
            Token_Types::TT_LESS => "LESS",
            Token_Types::TT_GREAT => "GREAT",
            Token_Types::TT_LESS_EQUAL => "LESS_EQUAL",
            Token_Types::TT_GREAT_EQUAL => "GREAT_EQUAL",
            Token_Types::TT_EQUAL => "EQUAL",
            Token_Types::TT_SEMI => "SEMI",
            Token_Types::TT_NAME => "NAME",
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
struct Token {
    token_type: Token_Types,
    value: Option<ValueType>,
}

pub mod lexer;
