use super::number::{Arith, Compare, Zero};
use crate::eval::expr::Eql;
use std::fmt::Display;

#[derive(Clone, Debug, PartialEq)]
pub struct Float(f64);

impl Arith for Float {
    fn add(&self, rhs: &Self) -> Self {
        Float(self.0 + rhs.0)
    }
    fn sub(&self, rhs: &Self) -> Self {
        Float(self.0 - rhs.0)
    }
    fn mul(&self, rhs: &Self) -> Self {
        Float(self.0 * rhs.0)
    }
    fn div(&self, rhs: &Self) -> Self {
        Float(self.0 / rhs.0)
    }
    fn neg(&self) -> Self {
        Float(-self.0)
    }
}

impl Display for Float {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Zero for Float {
    fn zero<'a>() -> &'a Self {
        &Float(0f64)
    }
}

impl Eql for Float {
    fn eql(&self, rhs: &Self) -> bool {
        self.0 == rhs.0
    }
}

impl Compare for Float {
    fn smaller(&self, rhs: &Self) -> bool {
        self.0 < rhs.0
    }
    fn bigger(&self, rhs: &Self) -> bool {
        self.0 > rhs.0
    }
}

impl Float {
    pub fn new(n: f64) -> Self {
        Float(n)
    }
    pub fn value(&self) -> f64 {
        self.0
    }
}
