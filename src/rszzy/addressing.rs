use crate::rszzy::versions::Version;
use std::fmt::Display;

/// Abstract representation for an offset into the "memory" of the ZMachine.
/// The ZMachine has at least 4 different ways of representing a pointer:
///
/// * ByteAddress
/// * WordAddress
/// * PackedAddress
/// * the PC (this is not called out explicitly in the ZSpec, but it acts as a ZOffset.
///
/// See each address type for details.
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

impl From<u16> for ZOffset {
    fn from(sz: u16) -> ZOffset {
        ZOffset(usize::from(sz))
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
        write!(f, "ZO:0x{:x}", self.0)
    }
}

// ZSpec 1.2 - three kinds of addresses.

/// ZSpec 1.2.1 - 2-byte address that can be used directly.
#[derive(Debug, Clone, Copy)]
pub struct ByteAddress(u16);

impl ByteAddress {
    pub fn raw(addr: u16) -> ByteAddress {
        ByteAddress(addr)
    }
}

impl From<ByteAddress> for ZOffset {
    fn from(ba: ByteAddress) -> ZOffset {
        ZOffset(usize::from(ba.0))
    }
}

/// ZSpec 1.2.2 - 2-byte address that counts words, not bytes.
/// The underlying value is multiplied by 2 to create the ZOffset.
#[derive(Debug, Clone, Copy)]
pub struct WordAddress(u16);

impl From<u16> for WordAddress {
    fn from(val: u16) -> WordAddress {
        WordAddress(val)
    }
}

impl From<WordAddress> for ZOffset {
    fn from(wa: WordAddress) -> ZOffset {
        ZOffset(usize::from(wa.0 * 2))
    }
}

/// ZSpec 1.2.3 - 2 byte address, possibly into high memory.
/// Used to refer to routines and strings. Interpreted differently
/// on almost every ZVersion.
#[derive(Debug, Clone, Copy)]
pub struct PackedAddress(u16);

impl PackedAddress {
    pub fn routine_offset(self, version: &Version) -> ZOffset {
        ZOffset::from(usize::from(self.0 * version.packed_multiplier as u16))
    }

    pub fn string_offset(self, version: &Version) -> ZOffset {
        ZOffset::from(usize::from(self.0 * version.packed_multiplier as u16))
    }
}

#[cfg(test)]
mod test {
    use super::super::versions::number_to_version;
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
        let v3 = number_to_version(3).unwrap();
        assert_eq!(44, usize::from(PackedAddress(22).routine_offset(&v3)));
        assert_eq!(44, usize::from(PackedAddress(22).string_offset(&v3)));

        let v5 = number_to_version(5).unwrap();
        assert_eq!(88, usize::from(PackedAddress(22).routine_offset(&v5)));
        assert_eq!(88, usize::from(PackedAddress(22).string_offset(&v5)));
    }
}
