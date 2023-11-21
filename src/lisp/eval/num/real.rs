use super::{float::Float, int::Int, ratio::Ratio};

#[derive(Clone)]
pub enum Real {
    Int(Int),
    Float(Float),
    Ratio(Ratio),
}

impl Real {
    fn to_float(&self) -> Float {
        match self {
            Real::Int(i) => Float::new(i.value() as f64),
            Real::Float(f) => Float::new(f.value()),
            Real::Ratio(r) => Float::new(r.value()),
        }
    }
    fn truncate(&self) -> (Real, Real) {
        let number = self.to_float().value();
        let quotient = number.trunc() as i64;
        let remainder = number - quotient as f64;
        if remainder == remainder.floor() {
            (
                Real::Int(Int::new(quotient)),
                Real::Int(Int::new(remainder as i64)),
            )
        } else {
            (
                Real::Int(Int::new(quotient)),
                Real::Float(Float::new(remainder)),
            )
        }
    }
    fn floor(&self) -> (Real, Real) {
        let number = self.to_float().value();
        let quotient = number.floor() as i64;
        let remainder = number - quotient as f64;
        if remainder == remainder.floor() {
            (
                Real::Int(Int::new(quotient)),
                Real::Int(Int::new(remainder as i64)),
            )
        } else {
            (
                Real::Int(Int::new(quotient)),
                Real::Float(Float::new(remainder)),
            )
        }
    }
    fn ceiling(&self) -> (Real, Real) {
        let number = self.to_float().value();
        let quotient = number.ceil() as i64;
        let remainder = number - quotient as f64;
        if remainder == remainder.floor() {
            (
                Real::Int(Int::new(quotient)),
                Real::Int(Int::new(remainder as i64)),
            )
        } else {
            (
                Real::Int(Int::new(quotient)),
                Real::Float(Float::new(remainder)),
            )
        }
    }
    fn round(&self) -> (Real, Real) {
        let number = self.to_float().value();
        let quotient = number.round() as i64;
        let remainder = number - quotient as f64;
        if remainder == remainder.floor() {
            (
                Real::Int(Int::new(quotient)),
                Real::Int(Int::new(remainder as i64)),
            )
        } else {
            (
                Real::Int(Int::new(quotient)),
                Real::Float(Float::new(remainder)),
            )
        }
    }
    fn modulus(&self, rhs: &Self) -> Real {
        let divisor = rhs.to_float().value();
        let number = self.to_float().value();
        let quotient = (number / divisor).floor() as i64;
        let remainder = number - quotient as f64;
        if remainder == remainder.floor() {
            Real::Int(Int::new(remainder as i64))
        } else {
            Real::Float(Float::new(remainder))
        }
    }
    fn rem(&self, rhs: &Self) -> Real {
        let divisor = rhs.to_float().value();
        let number = self.to_float().value();
        let quotient = (number / divisor).trunc() as i64;
        let remainder = number - quotient as f64;
        if remainder == remainder.floor() {
            Real::Int(Int::new(remainder as i64))
        } else {
            Real::Float(Float::new(remainder))
        }
    }
}
