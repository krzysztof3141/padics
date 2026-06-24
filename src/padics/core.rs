use std::marker::PhantomData;

pub trait PAdicScheme: Default + Clone {
    type Base: Clone;
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

#[derive(Default, Clone)]
pub struct PAdicNumber<Scheme: PAdicScheme, const PRIME: u64> {
    base: Scheme::Base,
    _marker: PhantomData<Scheme>,
}

impl<Scheme: PAdicScheme, const PRIME: u64> PAdicNumber<Scheme, PRIME> {
    pub fn new(base: Scheme::Base) -> Self {
        Self{base, _marker: PhantomData}
    }
    pub fn get_iter(&'_ self) -> PAdicIter<'_, Scheme> {
        PAdicIter::new(&self.base)
    }
    pub fn get_base(&self) -> &Scheme::Base {
        &self.base
    }
}
