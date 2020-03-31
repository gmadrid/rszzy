use std::fmt::Display;

#[derive(Default, Debug, Clone, Copy)]
pub struct ZOffset(usize);

impl From<ZOffset> for usize {
    fn from(offset: ZOffset) -> usize {
        offset.0
    }
}

impl From<usize> for ZOffset {
    fn from(sz: usize) -> ZOffset {
        ZOffset(sz)
    }
}

impl<T> std::ops::Add<T> for ZOffset
where
    T: Into<ZOffset>,
{
    type Output = ZOffset;

    fn add(self, rhs: T) -> Self::Output {
        ZOffset(self.0 + rhs.into().0)
    }
}

impl Display for ZOffset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "ZO:{}", self.0)
    }
}
