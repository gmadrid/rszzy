use crate::ensure;
use crate::rszzy::variable::ZVariable;
use crate::rszzy::addressing::{WordAddress, ZOffset};
use anyhow::{anyhow, Error};
use fehler::throws;

/// Abstract model of ZMachine memory as defined in ZSpec 1.
/// Implementors of the trait provide access to the backing store,
/// while the default functions will manage access control and
/// byte order.
pub trait Memory {
    fn memory_size(&self) -> usize;

    fn in_dynamic_range(&self, idx: ZOffset) -> bool;
    fn in_static_range(&self, idx: ZOffset) -> bool;

    #[throws]
    fn read_byte_unchecked(&self, offset: ZOffset) -> u8;
    #[throws]
    fn write_byte_unchecked(&mut self, offset: ZOffset, val: u8);

    #[throws]
    fn slice_at(&self, offset: ZOffset) -> &[u8];

    #[throws]
    fn read_byte(&self, offset: ZOffset) -> u8 {
        // ZSpec 1.1.1, 1.1.2, 1.1.3
        // - only dynamic and static memory may be read by the game.
        ensure!(
            self.in_dynamic_range(offset) || self.in_static_range(offset),
            anyhow!("Reading from illegal index: {}", offset)
        );
        self.read_byte_unchecked(offset)?
    }

    #[throws]
    fn write_byte(&mut self, offset: ZOffset, val: u8) {
        // ZSpec 1.1.1, 1.1.2, 1.1.3
        // - only dynamic memory may be written.
        // TODO ZSpec 1.1.1.1
        ensure!(
            self.in_dynamic_range(offset),
            anyhow!("Writing to illegal index: {}", offset)
        );
        self.write_byte_unchecked(offset, val)?;
    }

    #[throws]
    fn read_word<T>(&self, at: T) -> u16
    where
        T: Into<ZOffset> + Copy,
    {
        let offset = at.into();
        let high_byte = u16::from(self.read_byte(offset)?);
        let low_byte = u16::from(self.read_byte(offset + 1)?);
        (high_byte << 8) + low_byte
    }

    // May fail if word is outside dynamic memory.
    #[throws]
    fn write_word<T>(&mut self, at: T, val: u16)
    where
        T: Into<ZOffset> + Copy,
    {
        let offset = at.into();
        let high_byte = ((val >> 8) & 0xff) as u8;
        let low_byte = (val & 0xff) as u8;
        self.write_byte(offset, high_byte)?;
        self.write_byte(offset + 1, low_byte)?;
    }
}

pub trait AbbrevTable {
    #[throws]
    fn abbrev_location(&self, memory: &impl Memory, table: u8, idx: u8) -> WordAddress;
}

pub trait Stack {
    #[throws]
    fn push_byte(&mut self, val: u8);
    #[throws]
    fn pop_byte(&mut self) -> u8;

    #[throws]
    fn push_frame(&mut self, return_pc: ZOffset, num_locals:u8, return_var: ZVariable, operands:&[u16]);
    #[throws]
    fn pop_frame(&mut self);

    #[throws]
    fn read_local(&self, var:ZVariable) -> u16;
    #[throws]
    fn write_local(&mut self, var: ZVariable, val: u16);

    fn return_pc(&self) -> usize;
    fn return_variable(&self) -> ZVariable;

    #[throws]
    fn push_word(&mut self, word:u16) {
        self.push_byte((word >> 8 & 0xff) as u8)?;
        self.push_byte((word & 0xff) as u8)?;
    }

    #[throws]
    fn pop_word(&mut self) -> u16 {
        let low_byte = u16::from(self.pop_byte()?);
        let high_byte = u16::from(self.pop_byte()?);
        (high_byte << 8) + low_byte
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::rszzy::test::TestStack;

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

        #[throws]
        fn read_byte_unchecked(&self, offset: ZOffset) -> u8 {
            self.0[usize::from(offset)]
        }

        #[throws]
        fn write_byte_unchecked(&mut self, offset: ZOffset, val: u8) {
            self.0[usize::from(offset)] = val;
        }

        #[throws]
        fn slice_at(&self, offset: ZOffset) -> &[u8] {
            &self.0[usize::from(offset)..]
        }
    }

    #[test]
    fn test_size() {
        assert_eq!(100, TestMemory::default().memory_size())
    }

    #[throws]
    #[test]
    fn test_unchecked() {
        let mut m = TestMemory::default();
        assert_eq!(10, m.read_byte_unchecked(5.into())?);
        assert_eq!(30, m.read_byte_unchecked(15.into())?);
        assert_eq!(50, m.read_byte_unchecked(25.into())?);

        assert!(true);
        assert!(m.write_byte_unchecked(5.into(), 33).is_ok());
        assert!(m.write_byte_unchecked(15.into(), 34).is_ok());
        assert!(m.write_byte_unchecked(25.into(), 35).is_ok());

        assert_eq!(33, m.read_byte_unchecked(5.into())?);
        assert_eq!(34, m.read_byte_unchecked(15.into())?);
        assert_eq!(35, m.read_byte_unchecked(25.into())?);
    }

    #[test]
    #[throws]
    fn test_checked() {
        let mut m = TestMemory::default();
        assert_eq!(10, m.read_byte(5.into())?);
        assert_eq!(30, m.read_byte(15.into())?);

        // cannot read from high memory
        assert!(m.read_byte(25.into()).is_err());

        assert!(true);
        assert!(m.write_byte(5.into(), 33).is_ok());

        // cannot write to static or high memory
        assert!(m.write_byte(15.into(), 34).is_err());
        assert!(m.write_byte(25.into(), 35).is_err());

        assert_eq!(33, m.read_byte(5.into())?);

        // these values should be unchanged
        assert_eq!(30, m.read_byte_unchecked(15.into())?);
        assert_eq!(50, m.read_byte_unchecked(25.into())?);
    }

    #[test]
    #[throws]
    fn pushing_popping_words() {
        let mut stack = TestStack::default();

        stack.push_word(0x1234)?;
        stack.push_word(0x5678)?;

        assert_eq!(vec![0x12, 0x34, 0x56, 0x78], stack.vec);

        assert_eq!(0x5678, stack.pop_word()?);
        assert_eq!(0x1234, stack.pop_word()?);
    }
}
