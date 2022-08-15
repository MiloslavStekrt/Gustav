use crate::lex::Lexer;
use crate::st::{SyntaxKind, SyntaxToken};
use crate::Math;
use crate::esyntax::{BinnaryES, ExpressionSyntax};

#[derive(Clone)]
pub struct Parser {
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
    pub fn current(&self) -> SyntaxToken {
        self.tokens[self.pos]
    }
    pub fn peek(&self, offset: isize) -> SyntaxToken {
        let index =  (self.pos as isize + offset) as usize;

        if index >= self.tokens.len() {
            return self.tokens[self.tokens.len()-1].clone();
        }
        return self.tokens[index].clone();
    }
    pub fn next(&mut self) -> SyntaxToken {
        let curr = self.current().clone();
        self.pos += 1;
        curr
    }
    pub fn arimetric_operations(&self, token: SyntaxToken) -> Option<isize> {
        if match token.text() {
            "+" | "-" | "/" | "*" | "%" => false,
            _ => true
        } { return None; }

        let a = &self.peek(-1isize);
        let b = &self.peek(1isize);

        match token.kind() {
            SyntaxKind::Plus => Some(Math::add(&self.peek(-1isize).text(), &self.peek(1isize).text())),
            SyntaxKind::Minus => Some(Math::minus(&self.peek(-1isize).text(), &self.peek(1isize).text())),
            SyntaxKind::Divide => Some(Math::divide(&self.peek(-1isize).text(), &self.peek(1isize).text())),
            SyntaxKind::Times => Some(Math::times(&self.peek(-1isize).text(), &self.peek(1isize).text())),
            SyntaxKind::Modulo => Some(Math::modulo(&self.peek(-1isize).text(), &self.peek(1isize).text())),
            _ => None,
        }
    }
    fn is_aritmetic(&self, kind: &SyntaxKind) -> bool {
        let kinds = &[SyntaxKind::Plus, SyntaxKind::Minus];
        for x in kinds {
            if x == kind {return true;}
        }
        return false;
    }
    pub fn is_it(&self, kind: &SyntaxKind) -> SyntaxToken {
        let curr = self.current();
        if &curr.kind() == kind {
            return self.next()
        }
        return SyntaxToken::new(*kind, curr.pos(), "");
    }
    pub fn parser(&self) {
        let curr = self.current();
        let mut left = ParsePrimaryE();
        while self.is_aritmetic(&curr.kind()) {
            let operator = self.next();
            let right = ParsePrimaryE();
            left = Box::new(BinnaryES::new(left, operator, right));
        }
    }
    pub fn ParsePrimaryE() -> Box<dyn ExpressionSyntax> {
    }
    pub fn compute(&mut self) {
        while self.pos <= self.tokens.len() {
            if self.arimetric_operations(self.tokens[self.pos].clone()) != None {
                self.pos+=2;
                if self.arimetric_operations(self.tokens[self.pos].clone()) != None {
                    self.pos+=2;
                }
            }
        }
    }
}
