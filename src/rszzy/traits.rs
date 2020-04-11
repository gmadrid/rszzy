use super::addressing::{WordAddress, ZOffset};
use super::Result;
use crate::ensure;
use anyhow::anyhow;

/// Abstract model of ZMachine memory as defined in ZSpec 1.
/// Implementors of the trait provide access to the backing store,
/// while the default functions will manage access control and
/// byte order.
pub trait Memory {
    fn memory_size(&self) -> usize;

    fn in_dynamic_range(&self, idx: ZOffset) -> bool;
    fn in_static_range(&self, idx: ZOffset) -> bool;

    fn read_byte_unchecked(&self, offset: ZOffset) -> Result<u8>;
    fn write_byte_unchecked(&mut self, offset: ZOffset, val: u8) -> Result<()>;

    fn slice_at(&self, offset: ZOffset) -> Result<&[u8]>;

    fn read_byte(&self, offset: ZOffset) -> Result<u8> {
        // ZSpec 1.1.1, 1.1.2, 1.1.3
        // - only dynamic and static memory may be read by the game.
        ensure!(
            self.in_dynamic_range(offset) || self.in_static_range(offset),
            anyhow!("Reading from illegal index: {}", offset)
        );
        self.read_byte_unchecked(offset)
    }

    fn write_byte(&mut self, offset: ZOffset, val: u8) -> Result<()> {
        // ZSpec 1.1.1, 1.1.2, 1.1.3
        // - only dynamic memory may be written.
        // TODO ZSpec 1.1.1.1
        ensure!(
            self.in_dynamic_range(offset),
            anyhow!("Writing to illegal index: {}", offset)
        );
        self.write_byte_unchecked(offset, val)
    }

    fn read_word<T>(&self, at: T) -> Result<u16>
    where
        T: Into<ZOffset> + Copy,
    {
        let offset = at.into();
        let high_byte = u16::from(self.read_byte(offset)?);
        let low_byte = u16::from(self.read_byte(offset + 1usize)?);
        Ok((high_byte << 8) + low_byte)
    }

    // May fail if word is outside dynamic memory.
    fn write_word<T>(&mut self, at: T, val: u16) -> Result<()>
    where
        T: Into<ZOffset> + Copy,
    {
        let offset = at.into();
        let high_byte = ((val >> 8) & 0xff) as u8;
        let low_byte = (val & 0xff) as u8;
        self.write_byte(offset, high_byte)?;
        self.write_byte(offset + 1usize, low_byte)
    }
}

pub trait AbbrevTable {
    fn abbrev_location(&self, memory: &impl Memory, table: u8, idx: u8) -> Result<WordAddress>;
}

#[cfg(test)]
mod test {
    use super::*;

    struct TestMemory(Vec<u8>);

    impl Default for TestMemory {
        fn default() -> TestMemory {
            TestMemory((0..100).into_iter().map(|v| v * 2).collect::<Vec<_>>())
        }
    }

    impl Memory for TestMemory {
        fn memory_size(&self) -> usize {
            100
        }

        fn in_dynamic_range(&self, idx: ZOffset) -> bool {
            (0..10).contains(&usize::from(idx))
        }

        fn in_static_range(&self, idx: ZOffset) -> bool {
            (10..20).contains(&usize::from(idx))
        }

        fn read_byte_unchecked(&self, offset: ZOffset) -> Result<u8> {
            Ok(self.0[usize::from(offset)])
        }

        fn write_byte_unchecked(&mut self, offset: ZOffset, val: u8) -> Result<()> {
            self.0[usize::from(offset)] = val;
            Ok(())
        }
    }

    #[test]
    fn test_size() {
        assert_eq!(100, TestMemory::default().memory_size())
    }

    #[test]
    fn test_unchecked() {
        let mut m = TestMemory::default();
        assert_eq!(10, m.read_byte_unchecked(5.into()).unwrap());
        assert_eq!(30, m.read_byte_unchecked(15.into()).unwrap());
        assert_eq!(50, m.read_byte_unchecked(25.into()).unwrap());

        assert!(true);
        assert!(m.write_byte_unchecked(5.into(), 33).is_ok());
        assert!(m.write_byte_unchecked(15.into(), 34).is_ok());
        assert!(m.write_byte_unchecked(25.into(), 35).is_ok());

        assert_eq!(33, m.read_byte_unchecked(5.into()).unwrap());
        assert_eq!(34, m.read_byte_unchecked(15.into()).unwrap());
        assert_eq!(35, m.read_byte_unchecked(25.into()).unwrap());
    }

    #[test]
    fn test_checked() {
        let mut m = TestMemory::default();
        assert_eq!(10, m.read_byte(5.into()).unwrap());
        assert_eq!(30, m.read_byte(15.into()).unwrap());

        // cannot read from high memory
        assert!(m.read_byte(25.into()).is_err());

        assert!(true);
        assert!(m.write_byte(5.into(), 33).is_ok());

        // cannot write to static or high memory
        assert!(m.write_byte(15.into(), 34).is_err());
        assert!(m.write_byte(25.into(), 35).is_err());

        assert_eq!(33, m.read_byte(5.into()).unwrap());

        // these values should be unchanged
        assert_eq!(30, m.read_byte_unchecked(15.into()).unwrap());
        assert_eq!(50, m.read_byte_unchecked(25.into()).unwrap());
    }
}
