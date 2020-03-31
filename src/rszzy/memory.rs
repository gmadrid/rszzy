use super::addressing::ZOffset;
use super::constants::header_offset::{HIGH_MEMORY_MARK, STATIC_MEMORY_START, VERSION_NUMBER};
use super::traits::Memory;
use super::versions::number_to_version;
use anyhow::{anyhow, Result};
use std::io::Read;
use std::ops::Range;

pub struct ZMemory {
    bytes: Vec<u8>,

    dynamic_range: Range<usize>,
    static_range: Range<usize>,
}

mod bytes {
    #[inline]
    pub fn byte_from_slice<I>(slice: &[u8], idx: I) -> u8
    where
        I: Into<usize> + Copy,
    {
        slice[idx.into()]
    }

    #[inline]
    pub fn byte_to_slice<I>(slice: &mut [u8], idx: I, val: u8)
    where
        I: Into<usize> + Copy,
    {
        slice[idx.into()] = val;
    }

    #[inline]
    pub fn word_from_slice<I>(slice: &[u8], idx: I) -> u16
    where
        I: Into<usize> + Copy,
    {
        let high_byte = u16::from(byte_from_slice(slice, idx));
        let low_byte = u16::from(byte_from_slice(slice, idx.into() + 1));

        (high_byte << 8) + low_byte
    }

    #[cfg(test)]
    #[inline]
    pub fn word_to_slice<I>(slice: &mut [u8], idx: I, val: u16)
    where
        I: Into<usize> + Copy,
    {
        let high_byte = ((val >> 8) & 0xff) as u8;
        let low_byte = (val & 0xff) as u8;

        // big-endian
        byte_to_slice(slice, idx, high_byte);
        byte_to_slice(slice, idx.into() + 1, low_byte);
    }
}

impl ZMemory {
    pub fn from_reader<R>(mut rdr: R) -> Result<ZMemory>
    where
        R: Read,
    {
        let mut bytes = Vec::new();
        rdr.read_to_end(&mut bytes)?;

        let version_number = bytes::byte_from_slice(&bytes, VERSION_NUMBER);
        let version = number_to_version(version_number)?;

        if version.max_story_len < bytes.len() {
            return Err(anyhow!(
                "Story too long. Max: {}. Actual: {}.",
                version.max_story_len,
                bytes.len()
            ));
        }

        // ZSpec 1.1
        // - definition of three regions (dynamic, static, high)
        // - dynamic memory must have at least 64 bytes
        // - dynamic memory cannot overlap high memory
        let start_of_static = usize::from(bytes::word_from_slice(&bytes, STATIC_MEMORY_START));
        let end_of_static = std::cmp::min(0xffff, bytes.len());
        let start_of_high = usize::from(bytes::word_from_slice(&bytes, HIGH_MEMORY_MARK));

        if start_of_static < 64 {
            return Err(anyhow!(
                "Dynamic memory must contain at least 64 bytes, but contains {}",
                start_of_static
            ));
        }

        if start_of_high < start_of_static {
            return Err(anyhow!(
                "High memory begins at {} which overlaps dynamic memory which ends at {}",
                start_of_high,
                start_of_static - 1
            ));
        }

        Ok(ZMemory {
            bytes,
            dynamic_range: 0..start_of_static,
            static_range: start_of_static..end_of_static,
        })
    }
}

impl Memory for ZMemory {
    fn memory_size(&self) -> usize {
        self.bytes.len()
    }

    fn in_dynamic_range(&self, idx: ZOffset) -> bool {
        self.dynamic_range.contains(&usize::from(idx))
    }

    fn in_static_range(&self, idx: ZOffset) -> bool {
        self.static_range.contains(&usize::from(idx))
    }

    fn read_byte_unchecked(&self, offset: ZOffset) -> Result<u8> {
        Ok(bytes::byte_from_slice(&self.bytes, offset))
    }

    fn write_byte_unchecked(&mut self, offset: ZOffset, val: u8) -> Result<()> {
        bytes::byte_to_slice(&mut self.bytes, offset, val);
        Ok(())
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
