use crate::padics::core::{PAdicIter, PAdicNumber};

pub struct NegIter<const PRIME: u64> {
    neg_iter: Box<dyn PAdicIter>,
    was_any_non_zero: bool,
}
impl<const PRIME: u64> NegIter<PRIME> {
    fn new(neg_iter: Box<dyn PAdicIter>) -> Self {
        Self { neg_iter, was_any_non_zero : false }
    }
}
impl<const PRIME: u64> PAdicIter for NegIter<PRIME> {
   fn clone_box(&self) -> Box<dyn PAdicIter> {
       Box::new(Self::new(self.neg_iter.clone_box()))
   }
}

impl<const PRIME: u64> Iterator for NegIter<PRIME> {
    type Item = u64;
    fn next(&mut self) -> Option<u64> {
        let next = self.neg_iter.next().unwrap_or(0);
        if !self.was_any_non_zero && next == 0 {
            return Some(0);
        }
        else if !self.was_any_non_zero && next != 0 {
            self.was_any_non_zero = true;
            return Some(PRIME - next);
        }

        Some(PRIME - next - 1)
    }
}

pub struct NegNumber<const PRIME: u64> {
    neg_iter: Box<dyn PAdicIter>,
}

impl<const PRIME: u64> NegNumber<PRIME>  {
    pub fn new(neg_iter: Box<dyn PAdicIter>) -> Self{
        Self{neg_iter}
    }
}

impl<const PRIME: u64> PAdicNumber<PRIME> for NegNumber<PRIME> {
    fn iter(&self) -> Box<dyn PAdicIter> {
        Box::new(NegIter::<PRIME>::new(self.neg_iter.clone_box()))
    }

    fn clone_box(&self) -> Box<dyn PAdicNumber<PRIME>> {
        Box::new(Self::new(self.neg_iter.clone_box()))
    }
}