mod addressing;
mod constants;
mod memory;
mod traits;
mod versions;

use anyhow::Result;
use memory::ZMemory;
use std::io::Read;

pub type ZMachine = Machine<ZMemory>;

impl ZMachine {
    pub fn from_reader<R>(rdr: R) -> Result<ZMachine>
    where
        R: Read,
    {
        let memory = ZMemory::from_reader(rdr)?;
        Machine::with_memory(memory)
    }
}

#[derive(Default)]
pub struct Machine<M> {
    _memory: M,
    _pc: (),
    _processor: (),
    _stack: (),
}

impl<M> Machine<M> {
    pub fn with_memory(memory: M) -> Result<Machine<M>> {
        Ok(Machine {
            _memory: memory,
            _pc: (),
            _processor: (),
            _stack: (),
        })
    }

    pub fn run(self) -> Result<()> {
        Ok(())
    }
}
