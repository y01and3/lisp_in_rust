use super::number::{Arith, Compare, Zero};
use crate::lisp::eval::expr::Eql;
use std::fmt::Display;

pub struct Int(i64);

impl Arith for Int {
    fn add(&self, rhs: &Self) -> Self {
        Int(self.0 + rhs.0)
    }
    fn sub(&self, rhs: &Self) -> Self {
        Int(self.0 - rhs.0)
    }
    fn mul(&self, rhs: &Self) -> Self {
        Int(self.0 * rhs.0)
    }
    fn div(&self, rhs: &Self) -> Self {
        Int(self.0 / rhs.0)
    }
    fn neg(&self) -> Self {
        Int(-self.0)
    }
}

impl Display for Int {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Zero for Int {
    fn zero<'a>() -> &'a Self {
        &Int(0)
    }
}

impl Eql for Int {
    fn eql(&self, rhs: &Self) -> bool {
        self.0 == rhs.0
    }
}

impl Compare for Int {
    fn smaller(&self, rhs: &Self) -> bool {
        self.0 < rhs.0
    }
    fn bigger(&self, rhs: &Self) -> bool {
        self.0 > rhs.0
    }
}

impl Int {
    pub fn new(n: i64) -> Self {
        Int(n)
    }
    pub fn value(&self) -> i64 {
        self.0
    }
}
