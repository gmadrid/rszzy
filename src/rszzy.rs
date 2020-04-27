mod abbrevs;
mod addressing;
mod bytes;
mod constants;
mod header;
mod memory;
mod pc;
mod processor;
mod stack;
#[cfg(test)]
mod test;
mod text;
mod traits;
mod variable;
mod versions;

use anyhow::Error;
use fehler::throws;
use header::Header;
use memory::ZMemory;
use pc::PC;
use processor::ZProcessor;
use stack::ZStack;
use std::io::Read;

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
pub type ZMachine = Machine;

impl ZMachine {
    #[throws]
    pub fn from_reader<R>(rdr: R) -> ZMachine
    where
        R: Read,
    {
        let memory = ZMemory::from_reader(rdr)?;
        let stack = ZStack::default();
        MachineBuilder::new().memory(memory).stack(stack).build()
    }
}

/// Abstract representation of the ZMachine as outlined in the Overview of ZSpec 1.1.
/// All of the component types are represented as traits to facilitate testing.
pub struct Machine {
    // The "CPU"
    processor: ZProcessor,
}

impl Machine {
    #[throws]
    pub fn run(self) {
        self.processor.process()?;
    }
}

pub struct MachineBuilder {
    memory: Option<ZMemory>,
    pc: PC,
    stack: Option<ZStack>,
}

impl MachineBuilder {
    fn new() -> MachineBuilder {
        MachineBuilder {
            memory: None,
            pc: PC::default(),
            stack: None,
        }
    }

    fn memory(mut self, memory: ZMemory) -> Self {
        self.memory = Some(memory);
        self
    }

    fn stack(mut self, stack: ZStack) -> Self {
        self.stack = Some(stack);
        self
    }

    #[cfg(test)]
    fn pc(mut self, pc: PC) -> Self {
        self.pc = pc;
        self
    }

    fn build(mut self) -> ZMachine {
        if self.pc.is_zero() {
            // If the PC has been set explicitly, leave it alone.
            // Otherwise, set it from the Header.
            self.pc = PC::at(Header::start_pc(self.memory.as_ref().unwrap()));
        }

        // CHECK Options and return a Result

        let processor = ZProcessor::new(self.memory.unwrap(), self.pc, self.stack.unwrap());
        Machine { processor }
    }
}
