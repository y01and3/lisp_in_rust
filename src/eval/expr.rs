use super::{list::List, num::number::Number};
use std::fmt::Display;

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    Symbol(String),
    Number(Number),
    List(List),
}

pub trait Eql {
    fn eql(&self, rhs: &Self) -> bool;
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Symbol(s) => write!(f, "{}", s),
            Expr::Number(n) => write!(f, "{}", n),
            Expr::List(l) => write!(f, "{}", l),
        }
    }
}

impl Expr {
    pub fn symbol(&self) -> Option<&String> {
        match self {
            Expr::Symbol(s) => Some(s),
            _ => None,
        }
    }
    pub fn number(&self) -> Option<&Number> {
        match self {
            Expr::Number(n) => Some(n),
            _ => None,
        }
    }
    pub fn list(&self) -> Option<&List> {
        match self {
            Expr::List(l) => Some(l),
            _ => None,
        }
    }
}
