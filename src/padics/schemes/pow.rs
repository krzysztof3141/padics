use crate::padics::core::{PAdicIter, PAdicNumber};

pub struct PowIter<const PRIME: u64> {
    base_iter: Box<dyn PAdicIter>,
    base_prev_vals: Vec<u64>,
    carry: u64,
}

impl<const PRIME: u64> PowIter<PRIME> {
    pub fn new(base_iter: Box<dyn PAdicIter>) -> Self {
        Self{base_iter, base_prev_vals: vec![], carry: 0}
    }
}

impl<const PRIME: u64> Iterator for PowIter<PRIME> {
    type Item = u64;
    fn next(&mut self) -> Option<Self::Item> {
        let next_base_val = self.base_iter.next().unwrap_or(0);

        self.base_prev_vals.push(next_base_val);

        let mut sum = self.carry.clone();

        for (index, l) in self.base_prev_vals.iter().enumerate() {
            let r = self.base_prev_vals.get( self.base_prev_vals.len() - index - 1).unwrap_or(&0);
            sum += l*r;
        }

        self.carry = sum / PRIME;
        Some(sum % PRIME)
    }

}

impl<const PRIME: u64> PAdicIter for PowIter<PRIME> {
    fn clone_box(&self) -> Box<dyn PAdicIter> {
        Box::new(Self::new(self.base_iter.clone_box()))
    }
}

pub struct PowNumber<const PRIME: u64> {
    base_iter: Box<dyn PAdicIter>,
}

impl<const PRIME: u64> PowNumber<PRIME> {
    pub fn new(base_iter: Box<dyn PAdicIter>) -> Self {
        PowNumber{base_iter}
    }
}

impl<const PRIME: u64> PAdicNumber<PRIME> for PowNumber<PRIME> {
    fn iter(&self) -> Box<dyn PAdicIter> {
        Box::new(PowIter::<PRIME>::new(self.base_iter.clone_box()))
    }

    fn clone_box(&self) -> Box<dyn PAdicNumber<PRIME>> {
        Box::new(Self::new(self.base_iter.clone_box()))
    }
}