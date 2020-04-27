use crate::rszzy::{addressing::ZOffset, traits::Stack, variable::ZVariable};
use anyhow::Error;
use fehler::throws;

#[derive(Default)]
pub struct TestStack {
    pub vec: Vec<u8>,
}

impl Stack for TestStack {
    #[throws]
    fn push_byte(&mut self, val: u8) {
        self.vec.push(val);
    }

    #[throws]
    fn pop_byte(&mut self) -> u8 {
        self.vec.pop().unwrap()
    }

    #[throws]
    fn push_frame(
        &mut self,
        return_pc: ZOffset,
        num_locals: u8,
        return_var: ZVariable,
        operands: &[u16],
    ) {
        unimplemented!()
    }

    #[throws]
    fn pop_frame(&mut self) {
        unimplemented!()
    }

    #[throws]
    fn read_local(&self, l: ZVariable) -> u16 {
        unimplemented!()
    }

    #[throws]
    fn write_local(&mut self, l: ZVariable, val: u16) {
        unimplemented!()
    }

    fn return_pc(&self) -> usize {
        unimplemented!()
    }

    fn return_variable(&self) -> ZVariable {
        unimplemented!()
    }
}
