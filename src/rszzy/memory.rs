use super::addressing::ZOffset;
use super::constants::header_offset::STATIC_MEMORY_START;
use super::traits::Memory;
use anyhow::Result;
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

        let start_of_static = usize::from(bytes::word_from_slice(&bytes, STATIC_MEMORY_START));
        let end_of_static = std::cmp::min(0xffff, bytes.len());

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

    #[test]
    fn test_size() {
        let v = vec![0; 52];
        let m = ZMemory::from_reader(<&[u8]>::from(&v)).unwrap();

        assert_eq!(52, m.memory_size());
    }

    #[test]
    fn test_ranges() {
        let static_start = 0x50;

        let mut v = vec![0; 0x1000];
        bytes::word_to_slice(&mut v, STATIC_MEMORY_START, static_start);
        let m = ZMemory::from_reader(<&[u8]>::from(&v)).unwrap();

        assert_eq!(true, m.in_dynamic_range(0.into()));
        assert_eq!(true, m.in_dynamic_range(0x49.into()));
        assert_eq!(false, m.in_dynamic_range(0x50.into()));

        assert_eq!(false, m.in_static_range(0x49.into()));
        assert_eq!(true, m.in_static_range(0x50.into()));
        assert_eq!(true, m.in_static_range(0x1ff.into()));
        assert_eq!(true, m.in_static_range(0x200.into()));
        assert_eq!(true, m.in_static_range(0x0fff.into()));
        assert_eq!(false, m.in_static_range(0x1000.into()));
    }

    #[test]
    fn test_read_write() {
        let v = vec![0; 0x1000];
        let mut m = ZMemory::from_reader(<&[u8]>::from(&v)).unwrap();

        assert_eq!(0, m.read_byte_unchecked(0x34.into()).unwrap());
        assert!(m.write_byte_unchecked(0x34.into(), 87).is_ok());
        assert_eq!(87, m.read_byte_unchecked(0x34.into()).unwrap());
    }
}
