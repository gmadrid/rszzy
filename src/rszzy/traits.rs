use super::addressing::ZOffset;
use super::Result;
use anyhow::anyhow;
use std::ops::Range;

pub trait Memory {
    fn memory_size(&self) -> usize;

    fn dynamic_range(&self) -> Range<usize>;
    fn static_range(&self) -> Range<usize>;
    fn high_memory_range(&self) -> Range<usize>;

    fn read_byte_unchecked(&self, offset: ZOffset) -> Result<u8>;
    fn write_byte_unchecked(&mut self, offset: ZOffset, val: u8) -> Result<()>;

    fn read_byte(&self, offset: ZOffset) -> Result<u8> {
        let idx = usize::from(offset);
        if !self.dynamic_range().contains(&idx) && !self.static_range().contains(&idx) {
            return Err(anyhow!("Reading from illegal index: {}", idx));
        }
        self.read_byte_unchecked(offset)
    }

    fn write_byte(&mut self, offset: ZOffset, val: u8) -> Result<()> {
        let idx = usize::from(offset);
        if !self.dynamic_range().contains(&idx) {
            return Err(anyhow!("Writing to illegal index: {}", idx));
        }
        self.write_byte_unchecked(offset, val)
    }

    /*




    fn get_byte<T>(&self, at: T) -> u8
    where
        T: Into<ZOffset> + Copy;

    fn set_byte<T>(&mut self, at: T, val: u8)
    where
        T: Into<ZOffset> + Copy;

    fn is_readable<T>(&self, _at: T) -> bool
    where
        T: Into<ZOffset> + Copy,
    {
        true
    }

    fn is_writeable<T>(&self, _at: T) -> bool
    where
        T: Into<ZOffset> + Copy,
    {
        true
    }

    fn read_byte<T>(&self, at: T) -> u8
    where
        T: Into<ZOffset> + Copy,
    {
        let offset = at.into();
        if !self.is_readable(offset) {
            // Reading from memory is the most common operation that the
            // ZMachine executes, and there are NO unreadable memory
            // locations.  I'm choosing to return the bare byte and not a
            // result, but on the chance that somebody implements unreadable
            // memory, I'm check-panicking.
            panic!("Attempt to read unreadable byte at {}", offset);
        }
        self.get_byte(offset)
    }

    fn write_byte<T>(&mut self, at: T, val: u8) -> Result<()>
    where
        T: Into<ZOffset> + Copy,
    {
        let offset = at.into();
        if !self.is_writeable(offset) {
            return Err(anyhow!("Writing unwritable offset {}", offset));
        }
        self.set_byte(offset, val);
        Ok(())
    }

    fn read_word<T>(&self, at: T) -> u16
    where
        T: Into<ZOffset> + Copy,
    {
        let offset = at.into();
        let high_byte = u16::from(self.read_byte(offset));
        let low_byte = u16::from(self.read_byte(offset.inc_by(1)));
        (high_byte << 8) + low_byte
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
        self.write_byte(offset.inc_by(1), low_byte)
    }*/
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

        fn dynamic_range(&self) -> Range<usize> {
            0..10
        }

        fn static_range(&self) -> Range<usize> {
            10..20
        }

        fn high_memory_range(&self) -> Range<usize> {
            20..self.memory_size()
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
    fn test_ranges() {
        let m = TestMemory::default();
        assert_eq!(0..10, m.dynamic_range());
        assert_eq!(10..20, m.static_range());
        assert_eq!(20..100, m.high_memory_range());
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
