use crate::padics::core::{PAdicNumber, PAdicScheme};

use std::ops::{Add, Neg, Sub};
use crate::padics::schemes::neg::{NegBase, NegScheme};
use crate::padics::schemes::sum::{SumBase, SumScheme};

impl<'a, 'b, LhsScheme, RhsScheme, const PRIME: u64> Add<&'b PAdicNumber<RhsScheme, PRIME>>
for &'a PAdicNumber<LhsScheme, PRIME>
where
    LhsScheme: PAdicScheme,
    RhsScheme: PAdicScheme,
{
    type Output = PAdicNumber<SumScheme<LhsScheme, RhsScheme>, PRIME>;

    fn add(self, rhs: &'b PAdicNumber<RhsScheme, PRIME>) -> Self::Output {
        PAdicNumber::new(SumBase::new(
            self.get_base().clone(),
            rhs.get_base().clone(),
            PRIME,
        ))
    }
}

impl<'a, 'b, LhsScheme, RhsScheme, const PRIME: u64> Sub<&'b PAdicNumber<RhsScheme, PRIME>>
for &'a PAdicNumber<LhsScheme, PRIME>
where
    LhsScheme: PAdicScheme,
    RhsScheme: PAdicScheme,
{
    type Output = PAdicNumber<SumScheme<LhsScheme, NegScheme<RhsScheme>>, PRIME>;

    fn sub(self, rhs: &'b PAdicNumber<RhsScheme, PRIME>) -> Self::Output {
        PAdicNumber::new(SumBase::new(
            self.get_base().clone(),
            NegBase::new(rhs.get_base().clone(), PRIME),
            PRIME,
        ))
    }
}

impl <'a, Scheme, const PRIME: u64> Neg for &'a PAdicNumber<Scheme, PRIME>
where
    Scheme: PAdicScheme
{
    type Output = PAdicNumber<NegScheme<Scheme>, PRIME>;
    fn neg(self) -> Self::Output {
        PAdicNumber::new(NegBase::new(self.get_base().clone(), PRIME))
    }
}