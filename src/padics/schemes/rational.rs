use std::iter::Cycle;
use std::vec::IntoIter;
use crate::padics::core::{PAdicIter, PAdicNumber};


pub struct RationalIter<const PRIME: u64> {
    start: IntoIter<u64>,
    repeat: Cycle<IntoIter<u64>>,
}

impl<const PRIME: u64> RationalIter<PRIME> {
    pub fn new(start: IntoIter<u64>, repeat: Cycle<IntoIter<u64>>) -> Self {
        RationalIter{start, repeat}
    }
}

impl<const PRIME: u64> PAdicIter for RationalIter<PRIME> {
    fn clone_box(&self) -> Box<dyn PAdicIter> {
        Box::new(Self::new(self.start.clone(), self.repeat.clone()))
    }
}

impl<const PRIME: u64> Iterator for RationalIter<PRIME> {
    type Item = u64;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next) = self.start.next() {
            return Some(next);
        }

        Some(self.repeat.next().unwrap_or(0))
    }
}

pub struct RationalNumber<const PRIME: u64> {
    start: IntoIter<u64>,
    repeat: Cycle<IntoIter<u64>>,
}

impl<const PRIME: u64> RationalNumber<PRIME> {
    pub fn new(start: IntoIter<u64>, repeat: Cycle<IntoIter<u64>>) -> Self {
        Self{start, repeat}
    }
}

impl<const PRIME: u64> PAdicNumber<PRIME> for RationalNumber<PRIME> {
    fn iter(&self) -> Box<dyn PAdicIter> {
        Box::new(RationalIter::<PRIME>::new(self.start.clone(), self.repeat.clone()))
    }

    fn clone_box(&self) -> Box<dyn PAdicNumber<PRIME>> {
        Box::new(Self::new(self.start.clone(), self.repeat.clone()))
    }
}