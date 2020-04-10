mod abbrevs;
mod addressing;
mod constants;
mod memory;
mod text;
mod traits;
mod versions;

use anyhow::Result;
use memory::ZMemory;
use traits::Memory;
use std::io::Read;

/// The public API for the ZMachine.
/// All component types are defined.
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

/// Abstract representation of the ZMachine as outlined in the Overview of ZSpec 1.1.
/// All of the component types are represented as traits to facilitate testing.
#[derive(Default)]
pub struct Machine<M> {
    // The ZMachine's "core" memory. Loaded from the story file
    _memory: M,

    // program counter
    _pc: (),

    // The "CPU"
    _processor: (),

    // Runtime stack for procedure calls/local vars
    _stack: (),
}

impl<M> Machine<M> where M: Memory {
    fn with_memory(memory: M) -> Result<Machine<M>> {
        Ok(Machine {
            _memory: memory,
            _pc: (),
            _processor: (),
            _stack: (),
        })
    }

    pub fn run(self) -> Result<()> {
        let addr = abbrevs::abbrev_location()?;
        let s = text::ZString::new(self._memory.
                                   slice_at(addr.into())?);
        let str = String::from(s);
        Ok(())
    }
}
