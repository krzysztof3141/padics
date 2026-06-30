use crate::features::hensel_lemma::lemma::hensel_lemma;
use crate::features::hensel_lemma::polynomial::Polynomial;
use crate::features::teichmuller::compute_teichmuller_numbers::compute_teichmuller_numbers;
use crate::padics::schemes::rational::RationalNumber;

pub mod padics;
pub mod features;
pub mod math_utils;

fn main() {
    const PRIME: u64 = 5;
    let one = RationalNumber::<PRIME>::one();
    let minus_one = - RationalNumber::<PRIME>::one();

    println!("PRIME = {}", PRIME );

    println!("ONE = {}", one);
    println!("-ONE = {}", minus_one);

    let teichmuller_numbers = compute_teichmuller_numbers::<PRIME>();

    for (index, t_number) in teichmuller_numbers.iter().enumerate() {
        println!("{}th t_number = {}", index, t_number);
    }

    // X^3 - 2
    let coefficients = vec![-RationalNumber::from_natural(2), RationalNumber::zero(), RationalNumber::zero(), RationalNumber::one()];
    let polynomial = Polynomial::<PRIME>::new(coefficients);

    let roots = hensel_lemma(polynomial);

    println!("5-ADIC ROOTS OF X^3 - 2");

    for root in roots {
        println!("ROOT = {}", root);
    }
}
