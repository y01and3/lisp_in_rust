use super::{float::Float, int::Int};

pub trait Real {
    fn to_float(&self) -> Float;
    fn truncate(&self) -> (Int, Float) {
        let number = self.to_float().value();
        let quotient = number.trunc() as i64;
        let remainder = number - quotient as f64;
        (Int::new(quotient), Float::new(remainder))
    }
    fn floor(&self) -> (Int, Float) {
        let number = self.to_float().value();
        let quotient = number.floor() as i64;
        let remainder = number - quotient as f64;
        (Int::new(quotient), Float::new(remainder))
    }
    fn ceiling(&self) -> (Int, Float) {
        let number = self.to_float().value();
        let quotient = number.ceil() as i64;
        let remainder = number - quotient as f64;
        (Int::new(quotient), Float::new(remainder))
    }
    fn round(&self) -> (Int, Float) {
        let number = self.to_float().value();
        let quotient = number.round() as i64;
        let remainder = number - quotient as f64;
        (Int::new(quotient), Float::new(remainder))
    }
    fn modulus(&self, rhs: &dyn Real) -> Float {
        let divisor = rhs.to_float().value();
        let number = self.to_float().value();
        let quotient = (number / divisor).floor() as i64;
        let remainder = number - quotient as f64;
        Float::new(remainder)
    }
    fn rem(&self, rhs: &dyn Real) -> Float {
        let divisor = rhs.to_float().value();
        let number = self.to_float().value();
        let quotient = (number / divisor).trunc() as i64;
        let remainder = number - quotient as f64;
        Float::new(remainder)
    }
}
