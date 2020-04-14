use anyhow::Error;
use fehler::throws;
use std::io::Read;

#[derive(Default)]
pub struct ZMachine {
    _memory: (),
    _pc: (),
    _processor: (),
    _stack: (),
}

impl ZMachine {
    #[throws]
    pub fn from_reader<R>(_rdr: R) -> ZMachine
    where
        R: Read,
    {
        ZMachine::default()
    }

    #[throws]
    pub fn run(self) {}
}
