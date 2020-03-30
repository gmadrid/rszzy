use super::addressing::ZOffset;
use super::constants::header_offset::{HIGH_MEMORY_MARK, STATIC_MEMORY_START};
use super::traits::Memory;
use anyhow::Result;
use std::io::Read;

pub struct ZMemory {
    bytes: Vec<u8>,

    start_of_high: usize,
    end_of_high: usize,
    start_of_static: usize,
    end_of_static: usize,
}

mod bytes {
    // TODO: range check all of this.

    #[inline]
    pub fn byte_from_slice<I>(slice: &[u8], idx: I) -> u8
    where
        I: Into<usize> + Copy,
    {
        slice[idx.into()]
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
}

impl ZMemory {
    pub fn from_reader<R>(mut rdr: R) -> Result<ZMemory>
    where
        R: Read,
    {
        let mut bytes = Vec::new();
        rdr.read_to_end(&mut bytes)?;

        let start_of_high = usize::from(bytes::word_from_slice(&bytes, HIGH_MEMORY_MARK));
        let end_of_high = bytes.len();
        let start_of_static = usize::from(bytes::word_from_slice(&bytes, STATIC_MEMORY_START));
        let end_of_static = std::cmp::min(0xffff, bytes.len());

        Ok(ZMemory {
            bytes,
            start_of_high,
            end_of_high,
            start_of_static,
            end_of_static,
        })
    }
}

/*
impl Memory for ZMemory {
    fn memory_size(&self) -> usize {
        self.bytes.len()
    }

    fn get_byte<T>(&self, at: T) -> u8
    where
        T: Into<ZOffset> + Copy,
    {
        self.bytes[usize::from(at.into())]
    }

    fn set_byte<T>(&mut self, at: T, val: u8)
    where
        T: Into<ZOffset> + Copy,
    {
        self.bytes[usize::from(at.into())] = val
    }
}
*/
