use super::number::{Arith, Zero};
use crate::lisp::eval::expr::Eql;
use std::fmt::Display;

#[derive(Clone)]
pub struct Complex {
    real: f64,
    imaginary: f64,
}

impl Arith for Complex {
    fn add(&self, rhs: &Self) -> Self {
        Complex {
            real: self.real + rhs.real,
            imaginary: self.imaginary + rhs.imaginary,
        }
    }
    fn sub(&self, rhs: &Self) -> Self {
        Complex {
            real: self.real - rhs.real,
            imaginary: self.imaginary - rhs.imaginary,
        }
    }
    fn mul(&self, rhs: &Self) -> Self {
        Complex {
            real: self.real * rhs.real - self.imaginary * rhs.imaginary,
            imaginary: self.real * rhs.imaginary + self.imaginary * rhs.real,
        }
    }
    fn div(&self, rhs: &Self) -> Self {
        let denominator = rhs.real * rhs.real + rhs.imaginary * rhs.imaginary;
        Complex {
            real: (self.real * rhs.real + self.imaginary * rhs.imaginary) / denominator,
            imaginary: (self.imaginary * rhs.real - self.real * rhs.imaginary) / denominator,
        }
    }
    fn neg(&self) -> Self {
        Complex {
            real: -self.real,
            imaginary: -self.imaginary,
        }
    }
}

impl Display for Complex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#c({} {})", self.real, self.imaginary)
    }
}

impl Zero for Complex {
    fn zero<'a>() -> &'a Self {
        &Complex {
            real: 0f64,
            imaginary: 0f64,
        }
    }
}

impl Eql for Complex {
    fn eql(&self, rhs: &Self) -> bool {
        self.real == rhs.real && self.imaginary == rhs.imaginary
    }
}
