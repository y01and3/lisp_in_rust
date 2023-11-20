use super::{list::List, num::number::Number};

pub enum Expr {
    Symbol(String),
    Number(Number),
    List(List),
}

pub trait Eql {
    fn eql(&self, rhs: &Self) -> bool;
}
