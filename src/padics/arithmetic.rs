use std::ops::{Add, Mul, Neg, Sub, Div};
use num_traits::Pow;
use crate::padics::core::PAdicNumber;
use crate::padics::schemes::mul::{MulIter, MulNumber};
use crate::padics::schemes::neg::NegNumber;
use crate::padics::schemes::pow::PowNumber;
use crate::padics::schemes::div_p::DivPNumber;
use crate::padics::schemes::rational::RationalNumber;
use crate::padics::schemes::sum::{SumIter, SumNumber};


impl<const PRIME: u64> Add for Box<dyn PAdicNumber<PRIME>> {
    type Output = Box<dyn PAdicNumber<PRIME>>;
    fn add(self, rhs: Box<dyn PAdicNumber<PRIME>>) -> Self::Output {
        Box::new(SumNumber::new(self.iter(), rhs.iter()))
    }
}

impl<const PRIME: u64> Mul for Box<dyn PAdicNumber<PRIME>> {
    type Output = Box<dyn PAdicNumber<PRIME>>;
    fn mul(self, rhs: Box<dyn PAdicNumber<PRIME>>) -> Self::Output {
        Box::new(MulNumber::new(self.iter(), rhs.iter()))
    }
}

impl<const PRIME: u64> Neg for Box<dyn PAdicNumber<PRIME>> {
    type Output = Box<dyn PAdicNumber<PRIME>>;
    fn neg(self) -> Self::Output {
        Box::new(NegNumber::new(self.iter()))
    }
}

impl<const PRIME: u64> Sub for Box<dyn PAdicNumber<PRIME>> {
    type Output = Box<dyn PAdicNumber<PRIME>>;
    fn sub(self, rhs: Box<dyn PAdicNumber<PRIME>>) -> Self::Output {
        Box::new(SumNumber::new(self.iter(), (-rhs).iter()))
    }
}

impl<const PRIME: u64> Pow<usize> for Box<dyn PAdicNumber<PRIME>> {
    type Output = Box<dyn PAdicNumber<PRIME>>;
    fn pow(self, rhs: usize) -> Self::Output {
        let mut exp = rhs.clone();

        if exp == 0 {
            return Box::new(RationalNumber::new([1], []));
        }

        let mut base = self.clone_box();

        let mut result: Box<dyn PAdicNumber<PRIME>> = Box::new(RationalNumber::new([1], []));

        while exp > 1 {
            if exp & 1 == 1 {
                result = result * self.clone_box();
            }

            base = Box::new(PowNumber::<PRIME>::new(base.iter()));
            exp >>= 1;
        }

        result * base
    }
}

pub struct PrimePower<const PRIME: u64> {
    pub pow: u64
}

impl<const PRIME: u64> PrimePower<PRIME> {
    pub fn new(pow: u64) -> Self {
        Self { pow }
    }
}

impl<const PRIME: u64> Div<PrimePower<PRIME>> for Box<dyn PAdicNumber<PRIME>> {
    type Output = Box<dyn PAdicNumber<PRIME>>;
    fn div(self, rhs: PrimePower<PRIME>) -> Self::Output {
        Box::new(DivPNumber::new(self.iter().clone_box(), rhs.pow))
    }
}