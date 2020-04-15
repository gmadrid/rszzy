mod abbrevs;
mod addressing;
mod constants;
mod header;
mod memory;
mod pc;
mod processor;
mod stack;
mod text;
mod traits;
mod versions;

use anyhow::Error;
use fehler::throws;
use header::Header;
use memory::ZMemory;
use pc::PC;
use processor::ZProcessor;
use stack::ZStack;
use std::io::Read;
use traits::{Memory, Stack};

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
        let stack = ZStack::default();
        MachineBuilder::new().memory(memory).stack(stack).build()
    }
}

/// Abstract representation of the ZMachine as outlined in the Overview of ZSpec 1.1.
/// All of the component types are represented as traits to facilitate testing.
pub struct Machine<M> {
    // The "CPU"
    processor: ZProcessor,
    _phantom: std::marker::PhantomData<M>
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

pub struct MachineBuilder<M, S> {
    memory: Option<M>,
    pc: PC,
    stack: Option<S>,
}

impl<M, S> MachineBuilder<M, S>
where
    M: Memory,
S: Stack,
{
    fn new() -> MachineBuilder<M, S> {
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

    fn stack(mut self, stack: S) -> Self {
        self.stack = Some(stack);
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

        // CHECK Options and return a Result

        let processor = ZProcessor::new(self.memory.unwrap(), self.pc, self.stack.unwrap());
        Machine { processor }
    }
}
