use super::real::Real;
use crate::lisp::eval::expr::Eql;

pub enum Number {
    Real(Real),
    Complex,
}

pub trait Arith {
    fn add(&self, rhs: &Self) -> Self;
    fn sub(&self, rhs: &Self) -> Self;
    fn mul(&self, rhs: &Self) -> Self;
    fn div(&self, rhs: &Self) -> Self;
    fn neg(&self) -> Self;
}

pub trait Zero {
    fn zero<'a>() -> &'a Self;
}

pub trait EqlNum: Eql + Zero {
    fn zerop(&self) -> bool {
        self.eql(Self::zero())
    }
}

pub trait Compare: Zero {
    fn smaller(&self, rhs: &Self) -> bool;
    fn bigger(&self, rhs: &Self) -> bool;
    fn min<'a>(&'a self, rhs: &'a Self) -> &'a Self {
        if self.smaller(rhs) {
            self
        } else {
            rhs
        }
    }
    fn max<'a>(&'a self, rhs: &'a Self) -> &'a Self {
        if self.bigger(rhs) {
            self
        } else {
            rhs
        }
    }
    fn minusp(&self) -> bool {
        self.smaller(Self::zero())
    }
    fn pulsp(&self) -> bool {
        self.bigger(Self::zero())
    }
}
