use crate::padics::core::{PAdicIter, PAdicNumber};

pub struct MulIter<const PRIME: u64> {
    lhs_iter: Box<dyn PAdicIter>,
    rhs_iter: Box<dyn PAdicIter>,
    lhs_prev_vals: Vec<u64>,
    rhs_prev_vals: Vec<u64>,
    carry: u64,
}

impl<const PRIME: u64> MulIter<PRIME> {
    pub fn new(lhs_iter: Box<dyn PAdicIter>, rhs_iter: Box<dyn PAdicIter>) -> Self {
        Self{lhs_iter, rhs_iter, carry: 0, rhs_prev_vals: vec![], lhs_prev_vals: vec![]}
    }
}

impl<const PRIME: u64> Iterator for MulIter<PRIME> {
    type Item = u64;
    fn next(&mut self) -> Option<Self::Item> {
        let lhs_val = self.lhs_iter.next().unwrap_or(0);
        let rhs_val = self.rhs_iter.next().unwrap_or(0);

        self.lhs_prev_vals.push(lhs_val);
        self.rhs_prev_vals.push(rhs_val);

        let mut sum = self.carry.clone();

        for (index, l) in self.lhs_prev_vals.iter().enumerate() {
            let r = self.rhs_prev_vals.get( self.lhs_prev_vals.len() - index - 1).unwrap_or(&0);
            sum += l*r;
        }

        self.carry = sum / PRIME;
        Some(sum % PRIME)
    }

}

impl<const PRIME: u64> PAdicIter for MulIter<PRIME> {
    fn clone_box(&self) -> Box<dyn PAdicIter> {
        Box::new(Self::new(self.lhs_iter.clone_box(), self.rhs_iter.clone_box()))
    }
}

pub struct MulNumber<const PRIME: u64> {
    lhs_iter: Box<dyn PAdicIter>,
    rhs_iter: Box<dyn PAdicIter>,
}

impl<const PRIME: u64> MulNumber<PRIME> {
    pub fn new(lhs_iter: Box<dyn PAdicIter>, rhs_iter: Box<dyn PAdicIter>) -> Self {
        MulNumber{lhs_iter, rhs_iter}
    }
}

impl<const PRIME: u64> PAdicNumber<PRIME> for MulNumber<PRIME> {
    fn iter(&self) -> Box<dyn PAdicIter> {
        Box::new(MulIter::<PRIME>::new(self.lhs_iter.clone_box(), self.rhs_iter.clone_box()))
    }

    fn clone_box(&self) -> Box<dyn PAdicNumber<PRIME>> {
        Box::new(Self::new(self.lhs_iter.clone_box(), self.rhs_iter.clone_box()))
    }
}