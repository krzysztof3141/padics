use crate::padics::core::{PAdicIter, PAdicNumber};

pub struct DivPNumber<const PRIME: u64> {
    div_iter: Box<dyn PAdicIter>,
    pow: u64
}

impl<const PRIME: u64> DivPNumber<PRIME>  {
    pub fn new(div_iter: Box<dyn PAdicIter>, pow: u64 ) -> Self{
        Self{div_iter, pow}
    }
}

impl<const PRIME: u64> PAdicNumber<PRIME> for DivPNumber<PRIME> {
    fn iter(&self) -> Box<dyn PAdicIter> {
        let mut cloned_iter = self.div_iter.clone_box();

        for _ in 0..self.pow {
            cloned_iter.next();
        }

        cloned_iter
    }

    fn clone_box(&self) -> Box<dyn PAdicNumber<PRIME>> {
        Box::new(Self::new(self.div_iter.clone_box(), self.pow.clone()))
    }
}