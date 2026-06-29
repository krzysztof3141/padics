pub trait PAdicNumber<const PRIME: u64> {
    fn iter(&self) -> Box<dyn PAdicIter>;
    fn clone_box(&self) -> Box<dyn PAdicNumber<PRIME>>;
}

pub trait PAdicIter: Iterator<Item = u64> {
    fn clone_box(&self) -> Box<dyn PAdicIter>;
}