use crate::padics::core::{PAdicIter, PAdicNumber};


pub struct RationalIter<const PRIME: u64, const START_SIZE: usize, const REPEAT_SIZE: usize> {
    start: [u64; START_SIZE],
    repeat: [u64; REPEAT_SIZE],
    index: usize,
    repeat_digits: bool
}

impl<const PRIME: u64, const START_SIZE: usize, const REPEAT_SIZE: usize> RationalIter<PRIME, START_SIZE, REPEAT_SIZE> {
    pub fn new(start: [u64; START_SIZE], repeat: [u64; REPEAT_SIZE]) -> Self {
        RationalIter{start, repeat, index:0, repeat_digits: false}
    }
}

impl<const PRIME: u64, const START_SIZE: usize, const REPEAT_SIZE: usize> PAdicIter for RationalIter<PRIME, START_SIZE, REPEAT_SIZE> {
    fn clone_box(&self) -> Box<dyn PAdicIter> {
        Box::new(Self::new(self.start, self.repeat))
    }
}

impl<const PRIME: u64, const START_SIZE: usize, const REPEAT_SIZE: usize> Iterator for RationalIter<PRIME, START_SIZE, REPEAT_SIZE> {
    type Item = u64;
    fn next(&mut self) -> Option<Self::Item> {
        let option = if self.repeat_digits {
            match self.repeat.get(self.index) {
                Some(x) => {
                    self.index += 1;
                    x.clone()
                },
                None => {
                    self.index = 1;
                    self.repeat.get(0).unwrap_or(&0).clone()
                }
            }
        } else {
            match self.start.get(self.index) {
                Some(x) => {
                    self.index += 1;
                    x.clone()
                }
                None => {
                    self.repeat_digits = true;
                    self.index = 1;
                    self.repeat.get(0).unwrap_or(&0).clone()
                }
            }
        };
        Some(option)
    }
}

pub struct RationalNumber<const PRIME: u64, const START_SIZE: usize, const REPEAT_SIZE: usize> {
    pub start: [u64; START_SIZE],
    pub repeat: [u64; REPEAT_SIZE],
}

impl<const PRIME: u64, const START_SIZE: usize, const REPEAT_SIZE: usize> RationalNumber<PRIME, START_SIZE, REPEAT_SIZE> {
    pub fn new(start: [u64; START_SIZE], repeat: [u64; REPEAT_SIZE]) -> Self {
        Self{start, repeat}
    }
}

impl<const PRIME: u64, const START_SIZE: usize, const REPEAT_SIZE: usize> PAdicNumber<PRIME> for RationalNumber<PRIME, START_SIZE, REPEAT_SIZE> {
    fn iter(&self) -> Box<dyn PAdicIter> {
        Box::new(RationalIter::<PRIME, START_SIZE, REPEAT_SIZE>::new(self.start, self.repeat))
    }

    fn clone_box(&self) -> Box<dyn PAdicNumber<PRIME>> {
        Box::new(Self::new(self.start.clone(), self.repeat.clone()))
    }
}