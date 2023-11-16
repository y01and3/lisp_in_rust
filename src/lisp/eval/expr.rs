use super::num::number::Number;

pub enum Expr {
    Symbol(String),
    Number(Number),
    List(Vec<Expr>),
}

pub trait Eql {
    fn eql(&self, rhs: &Self) -> bool;
}
