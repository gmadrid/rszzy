use crate::ensure;
use crate::rszzy::addressing::{WordAddress, ZOffset};
use crate::rszzy::constants::header_offset::ABBREV_TABLE_START;
use crate::rszzy::traits::{AbbrevTable, Memory};
use anyhow::{anyhow, Result};

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
        ensure!(
            1 <= table && table <= 3,
            anyhow!("Table number, {}, is outside legal range, [1,3].", table)
        );
        ensure!(
            idx < 32,
            anyhow!("Index number, {}, is outside legal range, [0,32)", idx)
        );

        let offset = self.0 + usize::from(2 * (32 * (table - 1) + idx));
        let abbrev_offset = memory.read_word(offset)?;
        Ok(abbrev_offset.into())
    }
}
