use crate::padics::core::{PAdicIter, PAdicNumber};

pub struct SumIter<const PRIME: u64> {
    lhs_iter: Box<dyn PAdicIter>,
    rhs_iter: Box<dyn PAdicIter>,
    carry: u64,
}

impl<const PRIME: u64> SumIter<PRIME> {
    pub fn new(lhs_iter: Box<dyn PAdicIter>, rhs_iter: Box<dyn PAdicIter>) -> Self {
        Self{lhs_iter, rhs_iter, carry: 0}
    }
}

impl<const PRIME: u64> Iterator for SumIter<PRIME> {
    type Item = u64;
    fn next(&mut self) -> Option<Self::Item> {
        let lhs_val = self.lhs_iter.next().unwrap_or(0);
        let rhs_val = self.rhs_iter.next().unwrap_or(0);
        let sum = lhs_val + rhs_val + self.carry;

        self.carry = sum / PRIME;
        Some(sum % PRIME)
    }
}

impl<const PRIME: u64> PAdicIter for SumIter<PRIME> {
    fn clone_box(&self) -> Box<dyn PAdicIter> {
        Box::new(Self::new(self.lhs_iter.clone_box(), self.rhs_iter.clone_box()))
    }
}

pub struct SumNumber<const PRIME: u64> {
    lhs_iter: Box<dyn PAdicIter>,
    rhs_iter: Box<dyn PAdicIter>,
}

impl<const PRIME: u64> SumNumber<PRIME> {
    pub fn new(lhs_iter: Box<dyn PAdicIter>, rhs_iter: Box<dyn PAdicIter>) -> Self {
        SumNumber{lhs_iter, rhs_iter}
    }
}

impl<const PRIME: u64> PAdicNumber<PRIME> for SumNumber<PRIME> {
    fn iter(&self) -> Box<dyn PAdicIter> {
        Box::new(SumIter::<PRIME>::new(self.lhs_iter.clone_box(), self.rhs_iter.clone_box()))
    }

    fn clone_box(&self) -> Box<dyn PAdicNumber<PRIME>> {
        Box::new(Self::new(self.lhs_iter.clone_box(), self.rhs_iter.clone_box()))
    }
}
