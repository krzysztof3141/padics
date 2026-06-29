use std::iter::Cycle;
use std::vec::IntoIter;
use crate::math_utils::to_base_p::to_base_p;
use crate::padics::core::PAdicNumber;
use crate::padics::schemes::rational::RationalNumber;

impl<const PRIME: u64> RationalNumber<PRIME> {
   pub fn from_natural(n: u64) -> Box<dyn PAdicNumber<PRIME>> {
       let base_p = to_base_p(n, PRIME);
        Box::new(RationalNumber::new(base_p.into_iter(), vec![0].into_iter().cycle()))
   }

    pub fn from_integer(integer: i64) -> Box<dyn PAdicNumber<PRIME>> {
        if integer >= 0 {
            return Self::from_natural(integer as u64);
        }

       - Self::from_natural(-integer as u64)
    }

    pub fn zero() -> Box<dyn PAdicNumber<PRIME>> {
        Self::from_natural(0)
    }
    
    pub fn one() -> Box<dyn PAdicNumber<PRIME>> {
        Self::from_natural(1)
    }
}
