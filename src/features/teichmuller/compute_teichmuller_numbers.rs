use crate::features::hensel_lemma::lemma::hensel_lemma;
use crate::features::hensel_lemma::polynomial::Polynomial;
use crate::padics::core::PAdicNumber;
use crate::padics::schemes::rational::RationalNumber;

pub fn compute_teichmuller_numbers<const PRIME: u64>() -> Vec<Box<dyn PAdicNumber<PRIME>>> {
    let mut coefficients = vec![RationalNumber::zero(), - RationalNumber::one()];

    for _ in 2..PRIME {
        coefficients.push(RationalNumber::zero());
    }

    coefficients.push(RationalNumber::one());

    let p = Polynomial::new(coefficients); // x^p - x

    hensel_lemma(p)
}