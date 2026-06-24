use crate::padics::core::PAdicScheme;

#[derive(Default, Clone)]
pub struct RationalScheme<const START_SIZE: usize, const REPEAT_SIZE: usize> {}

#[derive(Clone)]
pub struct RationalSeries<const START_SIZE: usize, const REPEAT_SIZE: usize> {
    pub start: [u64; START_SIZE],
    pub repeat: [u64; REPEAT_SIZE],
}

pub struct RationalNumber {
    numerator: u64,
    denominator: u64,
}

#[derive(Clone)]
pub struct RationalBase<const START_SIZE: usize, const REPEAT_SIZE: usize> {
    series: RationalSeries<START_SIZE, REPEAT_SIZE>,
}

impl<const START_SIZE: usize, const REPEAT_SIZE: usize> RationalBase<START_SIZE, REPEAT_SIZE> {
    pub fn new(series: RationalSeries<START_SIZE, REPEAT_SIZE>) -> Self {
        Self { series }
    }
}

#[derive(Default)]
pub struct RationalDyn {
    pub index: usize,
    pub repeat: bool
}

impl<const START_SIZE: usize, const REPEAT_SIZE: usize> PAdicScheme for RationalScheme<START_SIZE, REPEAT_SIZE> {
    type Base = RationalBase<START_SIZE, REPEAT_SIZE>;
    type Dyn = RationalDyn;

    fn next(base: &Self::Base, dyns: &mut Self::Dyn) -> u64 {
        if dyns.repeat {
            match base.series.repeat.get(dyns.index) {
                Some(x) => {
                    dyns.index += 1;
                    x.clone()
                },
                None => {
                    dyns.index = 1;
                    base.series.repeat.get(0).unwrap_or(&0).clone()
                }
            }
        } else {
            match base.series.start.get(dyns.index) {
                Some(x) => {
                    dyns.index += 1;
                    x.clone()
                }
                None => {
                    dyns.repeat = true;
                    dyns.index = 1;
                    base.series.start.get(0).unwrap_or(&0).clone()
                }
            }
        }
    }
}
