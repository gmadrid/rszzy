use anyhow::Result;
use super::memory::ZMemory;
use super::pc::PC;

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

impl<M> ZProcessor<M> {
    pub fn new(memory: M, pc: PC, stack: Stack) -> ZProcessor<M> {
        ZProcessor { memory, pc, stack }
    }

    pub fn process() -> Result<()> {
        Ok(())
    }
}
