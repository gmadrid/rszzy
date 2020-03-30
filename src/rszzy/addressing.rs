use std::fmt::Display;

#[derive(Default, Debug, Clone, Copy)]
pub struct ZOffset(usize);

impl ZOffset {
    pub fn inc_by(self, by: usize) -> ZOffset {
        ZOffset(self.0 + by)
    }
}

impl From<ZOffset> for usize {
    fn from(offset: ZOffset) -> usize {
        offset.0
    }
}

impl From<usize> for ZOffset{
    fn from(sz: usize) -> ZOffset{
        ZOffset(sz)
    }
}

impl Display for ZOffset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "ZO:{}", self.0)
    }
}
