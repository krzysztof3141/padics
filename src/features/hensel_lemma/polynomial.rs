use num_traits::Pow;
use crate::padics::core::PAdicNumber;
use crate::padics::schemes::rational::RationalNumber;

pub struct  Polynomial<const PRIME: u64> {
    coefficients: Vec<Box<dyn PAdicNumber<PRIME>>>,
}

impl<const PRIME: u64> Polynomial<PRIME>{
    pub fn new(coefficients: Vec<Box<dyn PAdicNumber<PRIME>>>) -> Self {
        Self { coefficients }
    }
}

impl<const PRIME: u64> Polynomial<PRIME> {

    pub fn clone(&self) -> Polynomial<PRIME> {
        let mut coefficients = vec![];

        for c in self.coefficients.iter() {
            coefficients.push(c.clone_box());
        }

        Polynomial::new(coefficients)
    }
    pub fn value(&self, x: Box<dyn PAdicNumber<PRIME>> ) -> Box<dyn PAdicNumber<PRIME>> {
        let mut res: Box<dyn PAdicNumber<PRIME>> = RationalNumber::zero();

        for (n, c) in self.coefficients.iter().enumerate() {
            res = res + (c.clone_box() * x.clone_box().pow(n));
        }

        res
    }

    pub fn derivative(&self) -> Polynomial<PRIME> {
        let mut coefficients = vec![];

        for (n, c) in self.coefficients.iter().enumerate() {
            if n != 0 {
                coefficients.push(c.clone_box() * RationalNumber::from_natural(n as u64));
            }
        }

        Polynomial::new(coefficients)
    }
}