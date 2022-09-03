use std::fmt::Display;

#[derive(PartialEq, Copy, Clone)]
pub enum SyntaxKind {
    Number, WhiteSpace, Plus, Times, Minus, Modulo, Divide, OpenParenthesis, CloseParenthesis, BadToken, FileEnd, 
    BinnaryE,
}
impl Display for SyntaxKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = match self {
            Self::Number => "NUMBER",
            Self::WhiteSpace => "<WhiteSpace>",
            Self::Plus => "PLUS",
            Self::Times => "Times",
            Self::Minus => "Minus",
            Self::Modulo => "Modulo",
            Self::Divide => "Divide",
            Self::OpenParenthesis => "OpenParenthesis",
            Self::CloseParenthesis => "CloseParenthesis",
            Self::BadToken => "BadToken",
            Self::FileEnd => "FileEnd",
            Self::BinnaryE => "BinnaryE",
        };
        write!(f, "{}", out )
    }
}
impl From<String> for SyntaxKind {
    fn from(text: String) -> Self {
        match text.to_lowercase().as_str() {
            "number" => Self::Number,
            " " => Self::WhiteSpace,
            "+" => Self::Plus,
            "*" => Self::Times,
            "-" => Self::Minus,
            "%" => Self::Modulo,
            "/" => Self::Divide,
            "(" => Self::OpenParenthesis,
            ")" => Self::CloseParenthesis,
            "\n" => Self::FileEnd,
            "Binnary" => Self::BinnaryE,
            _ => Self::BadToken,
        }
    }
}
#[derive(Clone)]
pub struct SyntaxToken {
    kind: SyntaxKind,
    pos: usize,
    text: String,
}
impl SyntaxToken {
    pub fn new(kind: SyntaxKind, pos: usize, text: &str) -> Self {
        Self {
            kind, pos, text: text.to_string(),
        }
    }
    pub fn kind(&self) -> SyntaxKind {
        self.kind
    }
    pub fn pos(&self) -> usize {
        self.pos
    }
    pub fn text(&self) -> &str {
        self.text.as_str()
    }
}
