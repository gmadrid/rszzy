use crate::rszzy::traits::Stack;
use crate::rszzy::processor::ZVariable;
use anyhow::Error;
use fehler::throws;

#[derive(Default)]
pub struct ZStack {

}

impl ZStack {

}

impl Stack for ZStack {
    #[throws]
    fn push_byte(&mut self, val: u8) {
        unimplemented!()
    }

    #[throws]
    fn pop_byte(&mut self) -> u8 {
        unimplemented!()
    }

    #[throws]
    fn read_local(&self, l: u8) -> u16 {
        unimplemented!()
    }

    #[throws]
    fn write_local(&mut self, l: u8, val: u16) {
        unimplemented!()
    }

    #[throws]
    fn push_frame(&mut self, return_pc: usize, num_locals: u8, return_var: ZVariable, operands: &[u16]) {
        unimplemented!()
    }

    #[throws]
    fn pop_frame(&mut self) {
        unimplemented!()
    }

    fn return_pc(&self) -> usize {
        unimplemented!()
    }

    fn return_variable(&self) -> ZVariable {
        unimplemented!()
    }
}