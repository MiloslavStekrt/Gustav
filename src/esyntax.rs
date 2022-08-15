use crate::st::{SyntaxKind, SyntaxToken};


pub trait ExpressionSyntax {
}

// Binnary ExpressionSyntax
pub struct BinnaryES {
    kind: SyntaxKind,
    left: Box<dyn ExpressionSyntax>,
    operator: Box<dyn ExpressionSyntax>,
    right: Box<dyn ExpressionSyntax>,
}

impl ExpressionSyntax for BinnaryES { }
impl BinnaryES {
    pub fn new(left: Box<dyn ExpressionSyntax>, operator: Box<dyn ExpressionSyntax>, right: Box<dyn ExpressionSyntax>) -> Self {
        Self { left, operator, right, kind: SyntaxKind::BinnaryE }
    }
    // Getters? 
    pub fn left(self) -> Box<dyn ExpressionSyntax> {self.left}
    pub fn operator(self) -> Box<dyn ExpressionSyntax> {self.operator}
    pub fn right(self) -> Box<dyn ExpressionSyntax> {self.right}
    fn kind(self) -> SyntaxKind {self.kind}
}


// Number ExpressionSyntax
pub struct NumberES {
    kind: SyntaxKind,
    val: isize,
}

impl ExpressionSyntax for NumberES {
}

impl NumberES {
    pub fn new(number: SyntaxToken) -> Self {
        Self {
            kind: SyntaxKind::Number,
            val: number.text().parse::<isize>().unwrap(),
        }
    }
    pub fn val(&self) -> isize { self.val }
    fn kind(&self) -> SyntaxKind { self.kind }
}
