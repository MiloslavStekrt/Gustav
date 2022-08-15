use std::io::{stdin, stdout, Write};
use st::{SyntaxToken, SyntaxKind};

mod st;
mod lex;
mod par;
mod sn;
mod esyntax;
mod Math;

fn main() {
    run();
    println!("Hello, world!");
}
fn input() -> String {
    let mut s = String::new();
    print!("> ");
    let _ = stdout().flush();
    stdin().read_line(&mut s).expect("Didn't enter correct string");
    s
}
fn remove_last(s: String) -> String {
    let mut returner = s;
    let last: Option<char> = returner.chars().next_back();
    if let Some('\n') = last {
        returner.pop();
    }
    if let Some('\r') =  last {
        returner.pop();
    }
    returner.to_string()
}
fn run() {
    loop {
        let s = remove_last(input());

        let mut lexer = lex::Lexer::new(&s);
        loop {
            let token = lexer.next_token();
            if token.kind() == SyntaxKind::FileEnd {
                break;
            }
            println!("{}: '{}'", token.kind(), token.text());
        }
        let mut parser = par::Parser::new(&s);
        parser.compute();

        println!("You typed: {}", s);
    }
}
