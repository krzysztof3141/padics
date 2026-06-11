use std::marker::PhantomData;

pub trait PAdicScheme {
    type Base: Default;
    type Dyn: Default;
    fn next(base: &Self::Base, dyns: &mut Self::Dyn) -> u64;
}

pub struct PAdicIter<'a, Scheme: PAdicScheme> {
    base: &'a Scheme::Base,
    dyns: Scheme::Dyn,
}

impl<'a, Scheme: PAdicScheme> PAdicIter<'a, Scheme> {
    pub fn new(base: &'a Scheme::Base) -> Self {
        Self {
            base,
            dyns: Default::default(),
        }
    }

    pub fn new_view(&self) -> Self {
        Self {
            base: self.base,
            dyns: Default::default(),
        }
    }
}

impl<'a, Scheme: PAdicScheme> Iterator for PAdicIter<'a, Scheme> {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        Some(Scheme::next(self.base, &mut self.dyns))
    }
}

struct PAdicNumber<Scheme: PAdicScheme, const Prime: u64> {
    base: Scheme::Base,
    _marker: PhantomData<Scheme>,
}

