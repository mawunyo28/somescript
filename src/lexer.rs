use super::Token;

pub struct Lexer {
    tokens: Vec<Token>,
    text: String,
    current_char: Option<char>,
    pos: isize,
}

impl Lexer {
    fn new(text: String) -> Self {
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
            self.text.chars().nth(self.pos as usize)
        } else {
            None
        };
    }

    // fn make_token() -> Result<Vec<Token>, String> {}
}
