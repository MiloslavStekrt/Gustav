use crate::st::{SyntaxToken, SyntaxKind};

#[derive(Clone)]
pub struct Lexer {
    text: String,
    pos: usize
}
impl Lexer {
    pub fn new(text: &str) -> Self {
        Self {text: text.to_string(), pos: 0}
    }
    fn current(&self) -> char {
        if self.pos >= self.text.len() {
            return '\0';
        }
        return self.text.chars().nth(self.pos).unwrap();
    }
    pub fn next(&mut self) {
        self.pos+=1;
    }
    pub fn next_token(&mut self) -> SyntaxToken {
        let curr = self.current();

        if self.pos >= self.text.len() {
            return SyntaxToken::new(SyntaxKind::FileEnd, self.pos, "\0");
        }


        if curr.is_digit(10) {
            let start = self.pos;
            while self.current().is_digit(10) {
                self.next();
            }
            let text: &str = &self.text[start..self.pos];
            return SyntaxToken::new(SyntaxKind::Number, start, text);
        }
        
        if curr.is_whitespace() {
            let start = self.pos;
            while self.current().is_whitespace() {
                self.next();
            }
            let text: &str = &self.text[start..self.pos];
            return SyntaxToken::new(SyntaxKind::WhiteSpace, start, text)

        }
        // Every this operation must be symbolic type
        self.pos += 1;
        if curr == '*' {
            return SyntaxToken::new(SyntaxKind::Times, self.pos+1, "*");
        }
        if curr == '-' {
            return SyntaxToken::new(SyntaxKind::Minus, self.pos+1, "-");
        }
        if curr == '/' {
            return SyntaxToken::new(SyntaxKind::Divide, self.pos+1, "/");
        }
        if curr == '%' {
            return SyntaxToken::new(SyntaxKind::Modulo, self.pos+1, "%");
        }
        if curr == '+' {
            return SyntaxToken::new(SyntaxKind::Plus, self.pos+1, "+");
        }
        if curr == '(' {
            return SyntaxToken::new(SyntaxKind::OpenParenthesis, self.pos+1, "(");
        }
        if curr == ')' {
            return SyntaxToken::new(SyntaxKind::CloseParenthesis, self.pos+1, ")");
        }

        return SyntaxToken::new(SyntaxKind::BadToken, self.pos-1, &self.text[self.pos-1..self.pos]);
    }
}
