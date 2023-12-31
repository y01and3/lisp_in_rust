use super::{
    float::Float,
    number::{Arith, Compare, Zero},
    real::Real,
};
use crate::eval::expr::Eql;
use std::fmt::Display;

#[derive(Clone, Debug, PartialEq)]
pub struct Int(i64);

impl Real for Int {
    fn to_float(&self) -> Float {
        Float::new(self.0 as f64)
    }
}

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
