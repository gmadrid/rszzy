use super::addressing::ZOffset;
use std::ops::AddAssign;

#[derive(Debug, Default)]
pub struct PC(usize);

impl PC {
    pub fn at(offset: impl Into<ZOffset>) -> PC {
        PC(offset.into().into())
    }

    pub fn is_zero(&self) -> bool {
        self.0 == 0
    }
}

impl From<PC> for ZOffset {
    fn from(pc: PC) -> ZOffset {
        ZOffset::from(pc.0)
    }
}

impl<RHS> AddAssign<RHS> for PC
where
    RHS: Into<ZOffset> + Copy,
{
    fn add_assign(&mut self, rhs: RHS) {
        self.0 += usize::from(rhs.into());
    }
}
