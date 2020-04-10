use super::addressing::WordAddress;
use super::constants::header_offset::ABBREV_TABLE_START;
use super::traits::{AbbrevTable, Memory};
use crate::rszzy::addressing::ZOffset;
use anyhow::Result;

// Offset is location of abbrev table from header.
pub struct ZAbbrevTable(ZOffset);

impl ZAbbrevTable {
    pub fn new(memory: &impl Memory) -> Result<ZAbbrevTable> {
        Ok(ZAbbrevTable(ZOffset::from(
            memory.read_word(ZOffset::from(ABBREV_TABLE_START))?,
        )))
    }
}

impl AbbrevTable for ZAbbrevTable {
    /// table and idx are as described in ZSpec 3.3.
    fn abbrev_location(&self, memory: &impl Memory, table: u8, idx: u8) -> Result<WordAddress> {
        let offset = self.0 + usize::from(2 * (32 * (table - 1) + idx));
        let abbrev_offset = memory.read_word(offset)?;
        Ok(abbrev_offset.into())
    }
}
