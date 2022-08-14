use std::io::{stdin, stdout, Write};
use st::{SyntaxToken, SyntaxKind};

mod st;
mod lex;
mod par;

fn main() {
    run();
    println!("Hello, world!");
}
fn run() {
    loop {
        let mut s = String::new();
        print!("> ");
        let _ = stdout().flush();
        stdin().read_line(&mut s).expect("Didn't enter correct string");
        let last: Option<char> = s.chars().next_back();
        if let Some('\n') = last {
            s.pop();
        }
        if let Some('\r') =  last {
            s.pop();
        }

        let mut lexer = lex::Lexer::new(&s);
        loop {
            let token = lexer.next_token();
            if token.kind() == SyntaxKind::FileEnd {
                break;
            }
            println!("{}: '{}'", token.kind(), token.text());
        }

        println!("You typed: {}", s);
    }
}
