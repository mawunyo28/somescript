use super::Token;
use super::Token_Type;
use super::TokenList;

pub struct Lexer {
    pub tokens: TokenList,
    text: String,
    current_char: Option<char>,
    pos: isize,
}

impl Lexer {
    pub fn new(text: String) -> Self {
        Lexer {
            tokens: TokenList::new(),
            text,
            current_char: None,
            pos: -1,
        }
    }
    fn advance(&mut self) {
        self.pos += 1;

        self.current_char = if self.pos < self.text.len() as isize {
            match self.text.chars().nth(self.pos as usize) {
                Some(c) => Some(c),
                None => None,
            }
        } else {
            None
        };
    }

    pub fn make_tokens(&mut self) {
        self.advance();
        while self.current_char.is_some() {
            let current_char = self.current_char.unwrap();
            if current_char == ' ' || current_char == '\n' || current_char == '\t' {
                // break;
            } else if current_char == '+' {
                self.tokens.push(Token::new(Token_Type::TT_PLUS, None));
            } else if current_char == '-' {
                self.tokens.push(Token::new(Token_Type::TT_MINUS, None));
            } else if current_char == '/' {
                self.tokens.push(Token::new(Token_Type::TT_DIV, None));
            } else if current_char == '*' {
                self.tokens.push(Token::new(Token_Type::TT_MUL, None));
            } else if current_char == '%' {
                self.tokens.push(Token::new(Token_Type::TT_MOD, None));
            } else if current_char == '(' {
                self.tokens.push(Token::new(Token_Type::TT_LPAREN, None));
            } else if current_char == ')' {
                self.tokens.push(Token::new(Token_Type::TT_RPAREN, None));
            } else if current_char == '!' {
                self.tokens.push(Token::new(Token_Type::TT_NOT, None));
            } else if current_char == ':' {
                self.tokens.push(Token::new(Token_Type::TT_ASSIGN, None));
            } else if current_char == '"' {
                self.tokens.push(Token::new(Token_Type::TT_LQUOTE, None));
            }
            self.advance();
        }
    }
}
