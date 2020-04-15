mod abbrevs;
mod addressing;
mod constants;
mod header;
mod memory;
mod pc;
mod processor;
mod text;
mod traits;
mod versions;

use anyhow::Error;
use fehler::throws;
use header::Header;
use memory::ZMemory;
use pc::PC;
use processor::ZProcessor;
use std::io::Read;
use traits::Memory;

#[macro_export]
macro_rules! ensure {
    ( $c:expr, $e:expr ) => {
        if !($c) {
            return Err($e);
        }
    };
}

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
        MachineBuilder::new().memory(memory).build()
    }
}

/// Abstract representation of the ZMachine as outlined in the Overview of ZSpec 1.1.
/// All of the component types are represented as traits to facilitate testing.
pub struct Machine<M> {
    // The "CPU"
    processor: ZProcessor<M>,
}

impl<M> Machine<M>
where
    M: Memory,
{
    #[throws]
    pub fn run(self) {
        self.processor.process()?;
    }
}

pub struct MachineBuilder<M> {
    memory: Option<M>,
    pc: PC,
    stack: Option<()>,
}

impl<M> MachineBuilder<M>
where
    M: Memory,
{
    fn new() -> MachineBuilder<M> {
        MachineBuilder {
            memory: None,
            pc: PC::default(),
            stack: None,
        }
    }

    fn memory(mut self, memory: M) -> Self {
        self.memory = Some(memory);
        self
    }

    #[cfg(test)]
    fn pc(mut self, pc: PC) -> Self {
        self.pc = pc;
        self
    }

    fn build(mut self) -> Machine<M> {
        if self.pc.is_zero() {
            // If the PC has been set explicitly, leave it alone.
            // Otherwise, set it from the Header.
            self.pc = PC::at(Header::start_pc(self.memory.as_ref().unwrap()));
        }

        let processor = ZProcessor::new(self.memory.unwrap(), self.pc, ());
        Machine { processor }
    }
}
