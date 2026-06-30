use crate::features::hensel_lemma::lemma::{compute_d, compute_next_digit_and_a};
use crate::features::hensel_lemma::polynomial::Polynomial;
use crate::padics::core::{PAdicIter, PAdicNumber};

pub struct HenselIter<const PRIME: u64> {
    polynomial: Polynomial<PRIME>,
    prev_a: Box<dyn PAdicNumber<PRIME>>,
    d: Box<dyn PAdicNumber<PRIME>>, // d = (f'(a))^(-1) mod p
    n: u64,
}

impl<const PRIME: u64> HenselIter<PRIME> {
    pub fn new(polynomial: Polynomial<PRIME>, a_0: Box<dyn PAdicNumber<PRIME>>) -> Self {
        let d = compute_d(&polynomial, a_0.clone_box());

        Self{polynomial, prev_a: a_0, d, n: 0}
    }
}

impl<const PRIME: u64> Iterator for HenselIter<PRIME> {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.n == 0 {
            self.n += 1;
            return Some(self.prev_a.iter().next().unwrap_or(0));
        }

        let (t, a) = compute_next_digit_and_a(&self.polynomial, self.d.clone_box(), self.prev_a.clone_box(), self.n);

        self.n += 1;
        self.prev_a = a;
        Some(t)
    }
}

impl<const PRIME: u64> PAdicIter for HenselIter<PRIME> {
    fn clone_box(&self) -> Box<dyn PAdicIter> {
        Box::new(Self::new(self.polynomial.clone(), self.prev_a.clone_box()))
    }
}

pub struct HenselNumber<const PRIME: u64> {
    polynomial: Polynomial<PRIME>,
    a_0: Box<dyn PAdicNumber<PRIME>>,
}

impl<const PRIME: u64> HenselNumber<PRIME> {
    pub fn new(polynomial: Polynomial<PRIME>, a_0: Box<dyn PAdicNumber<PRIME>>) -> Self {
        Self{polynomial, a_0}
    }
}

impl<const PRIME: u64> PAdicNumber<PRIME> for HenselNumber<PRIME>  {
    fn iter(&self) -> Box<dyn PAdicIter> {
        Box::new(HenselIter::new(self.polynomial.clone(), self.a_0.clone_box()))
    }

    fn clone_box(&self) -> Box<dyn PAdicNumber<PRIME>> {
        Box::new(HenselNumber::new(self.polynomial.clone(), self.a_0.clone_box()))
    }
}
