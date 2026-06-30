use std::fmt::{Display};

pub trait PAdicNumber<const PRIME: u64> {
    fn iter(&self) -> Box<dyn PAdicIter>;
    fn clone_box(&self) -> Box<dyn PAdicNumber<PRIME>>;
}

pub trait PAdicIter: Iterator<Item = u64> {
    fn clone_box(&self) -> Box<dyn PAdicIter>;
}

impl<const PRIME: u64> Display for dyn PAdicNumber<PRIME> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const SIZE: u16 = 10;

        let mut working_iter = self.iter();

        let mut digits = Vec::new();
        for n in 0..SIZE - 1 {
            let digit = working_iter.next().unwrap_or(0);
            digits.push(format!("{}*{}^{}", digit, PRIME, n));
        }

        write!(f, "{} + ...", digits.join(" + "))
    }
}