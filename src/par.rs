use crate::lex::Lexer;
use crate::st::{SyntaxKind, SyntaxToken};

#[derive(Clone)]
struct Parser {
    tokens: Vec<SyntaxToken>,
    pos: usize,
}
impl Parser {
    pub fn new(text: &str) -> Self {
        let mut lexer = Lexer::new(text);
        let mut tokens: Vec<SyntaxToken> = Vec::new();
        let mut token: SyntaxToken = lexer.next_token();

        while token.kind() != SyntaxKind::FileEnd {
            token = lexer.next_token();

            match token.kind() {
                SyntaxKind::WhiteSpace => (),
                SyntaxKind::BadToken => (),
                _ => tokens.push(token.clone()),
            }
        }
        Self { tokens, pos: 0 }
    }
    fn peek(&self, offset: usize) -> SyntaxToken {
        let index =  self.pos + offset;

        if index >= self.tokens.len() {
            return self.tokens[self.tokens.len()-1].clone();
        }
        return self.tokens[index].clone();
    }
}
