use anyhow::Result;
use std::io::Read;

#[derive(Default)]
pub struct ZMachine {
    // The ZMachine's "core" memory. Loaded from the story file
    _memory: (),

    // program counter
    _pc: (),

    // The "CPU"
    _processor: (),

    // Runtime stack for procedure calls/local vars
    _stack: (),
}

impl ZMachine {
    pub fn from_reader<R>(_rdr: R) -> Result<ZMachine>
    where
        R: Read,
    {
        // There is currently nothing to do.
        Ok(ZMachine::default())
    }

    pub fn run(self) -> Result<()> {
        Ok(())
    }
}
