use std::ops::{Add, Neg, Sub};
use crate::padics::core::PAdicNumber;
use crate::padics::schemes::neg::NegNumber;
use crate::padics::schemes::sum::{SumIter, SumNumber};


impl<const PRIME: u64> Add for Box<dyn PAdicNumber<PRIME>> {
    type Output = Box<dyn PAdicNumber<PRIME>>;
    fn add(self, rhs: Box<dyn PAdicNumber<PRIME>>) -> Self::Output {
        Box::new(SumNumber::new(self.iter(), rhs.iter()))
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
