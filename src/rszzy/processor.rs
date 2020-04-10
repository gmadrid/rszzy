use anyhow::Result;
use super::memory::ZMemory;
use super::pc::PC;
use crate::rszzy::abbrevs::ZAbbrevTable;
use crate::rszzy::text::ZString;
use crate::rszzy::traits::{AbbrevTable, Memory};

type Stack = ();

#[derive(Default, Debug)]
pub struct ZProcessor<M = ZMemory> {
    // The ZMachine's "core" memory.
    memory: M,

    // program counter
    pc: PC,

    // Runtime stack for procedure calls/local vars
    stack: (),
}

impl<M> ZProcessor<M> where M: Memory {
    pub fn new(memory: M, pc: PC, stack: Stack) -> ZProcessor<M> {
        ZProcessor { memory, pc, stack }
    }

    pub fn process(&self) -> Result<()> {
                let abbrev_table = ZAbbrevTable::new(&self.memory)?;
        for table in 1..=3 {
            for i in 0..32 {
                let addr = abbrev_table.abbrev_location(&self.memory, table, i)?;
                let s = ZString::new(self.memory.slice_at(addr.into())?);
                println!("Table: {}, index: {}: {}", table, i, String::from(s));
            }
        }

        Ok(())
    }
}
