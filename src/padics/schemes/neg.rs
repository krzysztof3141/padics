use std::marker::PhantomData;
use crate::padics::core::PAdicScheme;

#[derive(Default, Clone)]
pub struct NegScheme<Scheme: PAdicScheme> {
    _marker: PhantomData<Scheme>,
}

pub struct NegDyn<Scheme: PAdicScheme> {
    dyns: Scheme::Dyn,
    is_first_digit: bool,
}

impl<Scheme: PAdicScheme> Default for NegDyn<Scheme> {
    fn default() -> Self {
        Self {dyns: Scheme::Dyn::default(), is_first_digit: true}
    }
}

#[derive(Clone)]
pub struct NegBase<Scheme: PAdicScheme> {
    base: Scheme::Base,
    prime: u64
}

impl<Scheme: PAdicScheme> NegBase<Scheme> {
    pub fn new(base: Scheme::Base, prime: u64) -> Self {
        Self { base, prime }
    }
}

impl<Scheme: PAdicScheme> PAdicScheme for NegScheme<Scheme> {
    type Base = NegBase<Scheme>;
    type Dyn = NegDyn<Scheme>;

    fn next(base: &Self::Base, dyns: &mut Self::Dyn) -> u64 {
        let next = Scheme::next(&base.base, &mut dyns.dyns);

        if dyns.is_first_digit {
            dyns.is_first_digit = false;
            return base.prime - next;
        }

        base.prime - next - 1
    }
}