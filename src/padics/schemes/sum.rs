use std::marker::PhantomData;
use crate::padics::core::{PAdicScheme};

#[derive(Default, Clone)]
pub struct SumScheme< LhsScheme: PAdicScheme, RhsScheme: PAdicScheme> {
    _marker_lhs: PhantomData<LhsScheme>,
    _marker_rhs: PhantomData<RhsScheme>,
}

#[derive(Default)]
pub struct SumDyn<LhsScheme: PAdicScheme, RhsScheme: PAdicScheme> {
    carry: u64,
    dyns_lhs: LhsScheme::Dyn,
    dyns_rhs: RhsScheme::Dyn
}

#[derive(Clone)]
pub struct SumBase<LhsScheme: PAdicScheme, RhsScheme: PAdicScheme> {
    base_lhs: LhsScheme::Base,
    base_rhs: RhsScheme::Base,
    prime: u64,
}

impl<LhsScheme: PAdicScheme, RhsScheme: PAdicScheme>  SumBase<LhsScheme, RhsScheme> {
    pub fn new(base_lhs: LhsScheme::Base, base_rhs: RhsScheme::Base, prime: u64) -> Self {
        Self{base_lhs, base_rhs, prime}
    }
}


impl<LhsScheme: PAdicScheme, RhsScheme: PAdicScheme> PAdicScheme for SumScheme<LhsScheme, RhsScheme> {
    type Base = SumBase<LhsScheme, RhsScheme>;
    type Dyn = SumDyn<LhsScheme, RhsScheme>;
    fn next(base: &Self::Base, dyns: &mut Self::Dyn) -> u64 {
        let lhs_val = LhsScheme::next(&base.base_lhs, &mut dyns.dyns_lhs);
        let rhs_val = RhsScheme::next(&base.base_rhs, &mut dyns.dyns_rhs);
        let sum = lhs_val + rhs_val + dyns.carry;

        dyns.carry = sum / base.prime;
        sum % base.prime
    }
}