use crate::ensure;
use crate::rszzy::addressing::ZOffset;
use crate::rszzy::constants::header_offset::{
    HIGH_MEMORY_MARK, STATIC_MEMORY_START, VERSION_NUMBER,
};
use crate::rszzy::traits::Memory;
use crate::rszzy::versions::number_to_version;
use crate::rszzy::{
    bytes,
};
use anyhow::{anyhow, Error};
use fehler::throws;
use std::io::Read;
use std::ops::Range;

/// Concrete model of the ZMachine memory as defined in ZSpec 1.
///
/// Manages
/// * loading a story file from a Reader
/// * validating the story file after loading
/// * defining the memory regions as defined in the story file.
pub struct ZMemory {
    bytes: Vec<u8>,

    dynamic_range: Range<usize>,
    static_range: Range<usize>,
}

impl ZMemory {
    #[throws]
    pub fn from_reader<R>(mut rdr: R) -> ZMemory
    where
        R: Read,
    {
        let mut bytes = Vec::new();
        rdr.read_to_end(&mut bytes)?;

        let version_number = bytes::byte_from_slice(&bytes, VERSION_NUMBER);
        let version = number_to_version(version_number)?;

        ensure!(
            bytes.len() <= version.max_story_len,
            anyhow!(
                "Story too long. Max: {}. Actual: {}.",
                version.max_story_len,
                bytes.len()
            )
        );

        // ZSpec 1.1
        // - definition of three regions (dynamic, static, high)
        // - dynamic memory must have at least 64 bytes
        // - dynamic memory cannot overlap high memory
        let start_of_static = usize::from(bytes::word_from_slice(&bytes, STATIC_MEMORY_START));
        let end_of_static = std::cmp::min(0xffff, bytes.len());
        let start_of_high = usize::from(bytes::word_from_slice(&bytes, HIGH_MEMORY_MARK));

        ensure!(
            start_of_static >= 64,
            anyhow!(
                "Dynamic memory must contain at least 64 bytes, but contains {}",
                start_of_static
            )
        );

        ensure!(
            start_of_static < start_of_high,
            anyhow!(
                "High memory begins at {} which overlaps dynamic memory which ends at {}",
                start_of_high,
                start_of_static - 1
            )
        );

        ZMemory {
            bytes,
            dynamic_range: 0..start_of_static,
            static_range: start_of_static..end_of_static,
        }
    }
}

impl Memory for ZMemory {
    #[throws]
    fn slice_at(&self, idx: ZOffset) -> &[u8] {
        &self.bytes.as_slice()[usize::from(idx)..]
    }

    fn memory_size(&self) -> usize {
        self.bytes.len()
    }

    fn in_dynamic_range(&self, idx: ZOffset) -> bool {
        self.dynamic_range.contains(&usize::from(idx))
    }

    fn in_static_range(&self, idx: ZOffset) -> bool {
        self.static_range.contains(&usize::from(idx))
    }

    #[throws]
    fn read_byte_unchecked(&self, offset: ZOffset) -> u8 {
        bytes::byte_from_slice(&self.bytes, offset)
    }

    #[throws]
    fn write_byte_unchecked(&mut self, offset: ZOffset, val: u8) {
        bytes::byte_to_slice(&mut self.bytes, offset, val);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const FAKE_STATIC_START: usize = 0x50;
    const FAKE_HIGH_START: usize = 0x200;

    fn fake_memory(size: usize) -> ZMemory {
        let mut v = vec![0; size];
        bytes::byte_to_slice(&mut v, 0usize, 3); // Version 3
        bytes::word_to_slice(&mut v, STATIC_MEMORY_START, FAKE_STATIC_START as u16);
        bytes::word_to_slice(&mut v, HIGH_MEMORY_MARK, FAKE_HIGH_START as u16);
        ZMemory::from_reader(<&[u8]>::from(&v)).unwrap()
    }

    #[test]
    fn test_size() {
        let m = fake_memory(52);
        assert_eq!(52, m.memory_size());
    }

    #[test]
    fn test_ranges() {
        let m = fake_memory(0x1000);

        assert_eq!(true, m.in_dynamic_range(0.into()));
        assert_eq!(true, m.in_dynamic_range((FAKE_STATIC_START - 1).into()));
        assert_eq!(false, m.in_dynamic_range(FAKE_STATIC_START.into()));

        assert_eq!(false, m.in_static_range((FAKE_STATIC_START - 1).into()));
        assert_eq!(true, m.in_static_range(FAKE_STATIC_START.into()));
        assert_eq!(true, m.in_static_range(0x1ff.into()));
        assert_eq!(true, m.in_static_range(0x200.into()));
        assert_eq!(true, m.in_static_range(0x0fff.into()));
        assert_eq!(false, m.in_static_range(0x1000.into()));
    }

    #[test]
    fn test_read_write() {
        let mut m = fake_memory(0x1000);

        assert_eq!(0, m.read_byte_unchecked(0x34.into()).unwrap());
        assert!(m.write_byte_unchecked(0x34.into(), 87).is_ok());
        assert_eq!(87, m.read_byte_unchecked(0x34.into()).unwrap());
    }
}
