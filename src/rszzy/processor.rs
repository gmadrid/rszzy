use crate::rszzy::memory::ZMemory;
use crate::rszzy::pc::PC;
use crate::rszzy::traits::Memory;
use anyhow::Error;
use fehler::throws;

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

impl<M> ZProcessor<M>
where
    M: Memory,
{
    pub fn new(memory: M, pc: PC, stack: Stack) -> ZProcessor<M> {
        ZProcessor { memory, pc, stack }
    }

    #[throws]
    pub fn process(&self)  {
    }
}
