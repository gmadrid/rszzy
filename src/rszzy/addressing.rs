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

// ZSpec 1.2 - three kinds of addresses.

// ZSpec 1.2.1
#[derive(Debug, Clone, Copy)]
pub struct ByteAddress(u16);

impl From<ByteAddress> for ZOffset {
    fn from(ba: ByteAddress) -> ZOffset {
        ZOffset(usize::from(ba.0))
    }
}

// ZSpec 1.2.2
#[derive(Debug, Clone, Copy)]
pub struct WordAddress(u16);

impl From<WordAddress> for ZOffset {
    fn from(wa: WordAddress) -> ZOffset {
        ZOffset(usize::from(wa.0 * 2))
    }
}

// ZSpec 1.2.3
#[derive(Debug, Clone, Copy)]
pub struct PackedAddress(u16);

impl PackedAddress {
    pub fn routine_offset(self) -> ZOffset {
        // TODO: This only works for V1-V3.
        ZOffset(usize::from(self.0 * 2))
    }

    pub fn string_offset(self) -> ZOffset {
        // TODO: This only works for V1-V3.
        ZOffset(usize::from(self.0 * 2))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_zoffset() {
        assert_eq!(32, usize::from(ZOffset::from(32)));
        assert_eq!(55, usize::from(ZOffset::from(50) + 5));
        assert_eq!("ZO:88", format!("{}", ZOffset(88)));
    }

    #[test]
    fn test_byte_address() {
        assert_eq!(22, usize::from(ZOffset::from(ByteAddress(22))));
    }

    #[test]
    fn test_word_address() {
        assert_eq!(44, usize::from(ZOffset::from(WordAddress(22))));
    }

    #[test]
    fn test_packed_address() {
        assert_eq!(44, usize::from(PackedAddress(22).routine_offset()));
        assert_eq!(44, usize::from(PackedAddress(22).string_offset()));
    }
}
