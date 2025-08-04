use super::Token_Type;

use super::Token;

pub struct Lexer {
    pub tokens: Vec<Token>,
    text: String,
    current_char: Option<char>,
    pos: isize,
}

impl Lexer {
    pub fn new(text: String) -> Self {
        Lexer {
            tokens: Vec::new(),
            text,
            current_char: None,
            pos: -1,
        }
    }
    fn advance(&mut self) {
        self.pos += 1;

        self.current_char = if self.pos < self.text.len() as isize {
            Some(self.text.chars().nth(self.pos as usize).unwrap())
        } else {
            None
        };
    }

    pub fn make_tokens(&mut self) {
        self.advance();
        while self.current_char.is_some() {
            let current_char = self.current_char.unwrap();

            if current_char == ' ' || current_char == '\n' {
                self.advance();
            } else if current_char == '+' {
                self.tokens.push(Token::new(Token_Type::TT_PLUS, None));
            }

            self.advance();
        }
    }
}
