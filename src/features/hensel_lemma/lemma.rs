use crate::features::hensel_lemma::polynomial::Polynomial;
use crate::features::hensel_lemma::scheme::HenselNumber;
use crate::math_utils::mod_inverse_prime::mod_inverse_prime;
use crate::padics::arithmetic::PrimePower;
use crate::padics::core::PAdicNumber;
use crate::padics::schemes::rational::RationalNumber;

pub fn hensel_lemma<const PRIME: u64>(p: Polynomial<PRIME>) -> Vec<Box<dyn PAdicNumber<PRIME>>> {
    let mut hensel_numbers: Vec<Box<dyn PAdicNumber<PRIME>>> = vec![];
    for n in 0..PRIME {
        let a_0 = RationalNumber::from_natural(n);
        let f_a = p.value(a_0.clone_box());
        let df_a = p.derivative().value(a_0.clone_box());

        if f_a.iter().next().unwrap_or(0) == 0 && df_a.iter().next().unwrap_or(0) != 0 {
            hensel_numbers.push(Box::new( HenselNumber::new(p.clone(), a_0)));
        }
    }

    hensel_numbers
}

pub fn compute_next_digit_and_a<const PRIME: u64>(p: &Polynomial<PRIME>, d: Box<dyn PAdicNumber<PRIME>>, prev_a: Box<dyn PAdicNumber<PRIME>>, n: u64)
-> (u64, Box<dyn PAdicNumber<PRIME>>){
    let t = (- d.clone_box() * p.value(prev_a.clone_box()) / PrimePower::<PRIME>::new(n)).iter().next().unwrap_or(0);

    let a = prev_a + RationalNumber::from_natural(t*PRIME.pow(n as u32));

    (t, a)
}

pub fn compute_d<const PRIME: u64>(p: &Polynomial<PRIME>, a_0: Box<dyn PAdicNumber<PRIME>>) -> Box<dyn PAdicNumber<PRIME>>{
    let first_digit = p.derivative().value(a_0).iter().next().unwrap_or(0);
    let inverse_digit = mod_inverse_prime(first_digit, PRIME).unwrap_or(1);

    RationalNumber::from_natural(inverse_digit)
}