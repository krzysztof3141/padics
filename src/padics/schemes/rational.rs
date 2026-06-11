use crate::padics::core::PAdicScheme;

pub struct RationalScheme {}

#[derive(Default)]
pub struct RationalSeries {
    pub start: Vec<u64>,
    pub repeat: Vec<u64>,
}

pub struct RationalNumber {
    numerator: u64,
    denominator: u64,
}

#[derive(Default)]
pub struct RationalBase {
    series: RationalSeries,
    number:  Option<RationalNumber>,
}


impl RationalBase {
    pub fn new(series: RationalSeries) -> Self {
        Self {
            series, number: None
        }
    }
}

#[derive(Default)]
pub struct RationalDyn {
    pub index: usize,
    pub repeating: bool
}

impl PAdicScheme for RationalScheme {
    type Base = RationalBase;
    type Dyn = RationalDyn;

    fn next(base: &Self::Base, dyns: &mut Self::Dyn) -> u64 {
        if dyns.repeating {
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
                    dyns.repeating = true;
                    dyns.index = 1;
                    base.series.repeat.get(0).unwrap_or(&0).clone()
                }
            }
        }
    }
}
