use crate::rszzy::memory::ZMemory;
use crate::rszzy::pc::PC;
use crate::rszzy::stack::ZStack;
use anyhow::Error;
use fehler::throws;

pub struct ZProcessor {
    // The ZMachine's "core" memory.
    memory: ZMemory,

    // program counter
    pc: PC,

    // Runtime stack for procedure calls/local vars
    stack: ZStack,
}

impl ZProcessor {
    pub fn new(memory: ZMemory, pc: PC, stack: ZStack) -> ZProcessor {
        ZProcessor { memory, pc, stack }
    }

    #[throws]
    pub fn process(&self) {}
}
