use anyhow::Result;
use std::io::Read;

#[derive(Default)]
pub struct ZMachine {
    _memory: (),
    _pc: (),
    _processor: (),
    _stack: (),
}

impl ZMachine {
    pub fn from_reader<R>(_rdr: R) -> Result<ZMachine>
    where
        R: Read,
    {
        Ok(ZMachine::default())
    }

    pub fn run(self) -> Result<()> {
        Ok(())
    }
}
