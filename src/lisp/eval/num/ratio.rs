use super::math::gcd::gcd;
use super::number::{Arith, Compare, Zero};
use crate::lisp::eval::expr::Eql;
use std::fmt::Display;

pub struct Ratio {
    numerator: i64,
    denominator: i64,
}

impl Arith for Ratio {
    fn add(&self, rhs: &Self) -> Self {
        let numerator = self.numerator * rhs.denominator + rhs.numerator * self.denominator;
        let denominator = self.denominator * rhs.denominator;
        let gcd = gcd(numerator.unsigned_abs(), denominator.unsigned_abs()) as i64;
        Ratio {
            numerator: numerator / gcd,
            denominator: denominator / gcd,
        }
    }
    fn sub(&self, rhs: &Self) -> Self {
        self.add(&rhs.neg())
    }
    fn mul(&self, rhs: &Self) -> Self {
        let numerator = self.numerator * rhs.numerator;
        let denominator = self.denominator * rhs.denominator;
        let gcd = gcd(numerator.unsigned_abs(), denominator.unsigned_abs()) as i64;
        Ratio {
            numerator: numerator / gcd,
            denominator: denominator / gcd,
        }
    }
    fn div(&self, rhs: &Self) -> Self {
        self.mul(&rhs.recip())
    }
    fn neg(&self) -> Self {
        Ratio {
            numerator: -self.numerator,
            denominator: self.denominator,
        }
    }
}

impl Display for Ratio {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} / {}", self.numerator, self.denominator)
    }
}

impl Zero for Ratio {
    fn zero<'a>() -> &'a Self {
        &Ratio {
            numerator: 0,
            denominator: 1,
        }
    }
}

impl Eql for Ratio {
    fn eql(&self, rhs: &Self) -> bool {
        self.numerator * rhs.denominator == rhs.numerator * self.denominator
    }
}

impl Compare for Ratio {
    fn smaller(&self, rhs: &Self) -> bool {
        self.numerator * rhs.denominator < rhs.numerator * self.denominator
    }
    fn bigger(&self, rhs: &Self) -> bool {
        self.numerator * rhs.denominator > rhs.numerator * self.denominator
    }
}

impl Ratio {
    pub fn recip(&self) -> Self {
        Ratio {
            numerator: self.denominator,
            denominator: self.numerator,
        }
    }
}
