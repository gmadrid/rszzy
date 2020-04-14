mod addressing;
mod constants;
mod memory;
mod traits;
mod versions;

use anyhow::{Error};
use fehler::throws;
use memory::ZMemory;
use std::io::Read;

/// The public API for the ZMachine.
/// All component types are defined.
pub type ZMachine = Machine<ZMemory>;

impl ZMachine {
    #[throws]
    pub fn from_reader<R>(rdr: R) -> ZMachine
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

impl<M> Machine<M> {
    fn with_memory(memory: M) -> Machine<M> {
        Machine {
            _memory: memory,
            _pc: (),
            _processor: (),
            _stack: (),
        }
    }

    #[throws]
    pub fn run(self) {}
}
