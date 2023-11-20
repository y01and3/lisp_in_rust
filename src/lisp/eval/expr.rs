use super::{list::List, num::number::Number};
use std::{fmt::Display, rc::Rc};

pub enum Expr {
    Symbol(String),
    Number(Number),
    List(Rc<List>),
    Nil,
}

pub trait Eql {
    fn eql(&self, rhs: &Self) -> bool;
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Symbol(s) => write!(f, "{}", s),
            Expr::Number(n) => write!(f, "{}", n),
            Expr::List(l) => write!(f, "{}", **l),
            Expr::Nil => write!(f, "nil"),
        }
    }
}
