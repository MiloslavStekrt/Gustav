use crate::st::{SyntaxKind, SyntaxToken};


pub trait ES {
    fn kind(&self) -> SyntaxKind;
}

impl ES for BinnaryES {
    fn kind(&self) -> SyntaxKind {self.kind}
}
impl ES for NumberES {
    fn kind(&self) -> SyntaxKind {self.kind}
}


// Binnary ES
pub struct BinnaryES {
    kind: SyntaxKind,
    left: Option<Box<dyn ES>>,
    operator: SyntaxToken,
    right: Option<Box<dyn ES>>,
}

impl BinnaryES {
    pub fn new(
        // operator => SyntaxKind
        // left => NumberES | BinnaryES 
        // right => NumberES | BinnaryES 
        left: Option<Box<dyn ES>>, operator: SyntaxToken, right: Option<Box<dyn ES>>) -> Self {
        Self { left, operator, right, kind: SyntaxKind::BinnaryE }
    }
    pub fn get_child(&self) -> (&Option<Box<dyn ES>>, SyntaxToken, &Option<Box<dyn ES>>) {
        return (&self.left, self.operator.clone(), &self.right);
    }
    // Getters? 
    pub fn left(&self) -> &Option<Box<dyn ES>> {&self.left}
    pub fn operator(&self) -> &SyntaxToken {&self.operator}
    pub fn right(&self) -> &Option<Box<dyn ES>> {&self.right}
}


// Number ES
#[derive(Clone)]
pub struct NumberES {
    kind: SyntaxKind,
    val: isize,
}

impl NumberES {
    pub fn new(number: SyntaxToken) -> Self {
        Self {
            kind: SyntaxKind::Number,
            val: number.text().parse::<isize>().unwrap(),
        }
    }
    pub fn get_child(&self) -> SyntaxKind {
        return self.kind;
    }
    pub fn val(&self) -> isize { self.val }
}
