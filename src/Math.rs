use crate::st::{SyntaxKind, SyntaxToken};

pub fn add(a: &str, b: &str) -> isize {
    a.parse::<isize>().unwrap() + b.parse::<isize>().unwrap()
}
pub fn minus(a: &str, b: &str) -> isize {
    a.parse::<isize>().unwrap() - b.parse::<isize>().unwrap()
}
pub fn modulo(a: &str, b: &str) -> isize {
    a.parse::<isize>().unwrap() % b.parse::<isize>().unwrap()
}
pub fn divide(a: &str, b: &str) -> isize {
    a.parse::<isize>().unwrap() / b.parse::<isize>().unwrap()
}
pub fn times(a: &str, b: &str) -> isize {
    a.parse::<isize>().unwrap() * b.parse::<isize>().unwrap()
}
