use crate::padics::core::{PAdicNumber, PAdicScheme};

use std::ops::{Add};
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
