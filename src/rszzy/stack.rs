use crate::rszzy::addressing::ZOffset;
use crate::rszzy::traits::Stack;
use crate::rszzy::processor::ZVariable;
use crate::rszzy::constants::stack::STACK_SIZE;
use crate::ensure;
use anyhow::{anyhow, Error};
use fehler::throws;

pub struct ZStack {
    stack: [u8; STACK_SIZE],

    fp: usize, // index in the stack of the current frame.

    s0: usize, // The bottom of the current frame's stack.
    // (The first byte after the local variables.)
    sp: usize, // points to the next empty byte.
    // Initialized to s0.
}

impl Default for ZStack {
    fn default() -> Self {
        let mut stack = ZStack {
            stack: [0; STACK_SIZE],
            fp: 0,
            s0: 0,
            sp: 0,
        };
        stack.init_new_stack().unwrap();
        stack.s0 = stack.sp;
        stack
    }
}

impl ZStack {
    #[throws]
    /// Create a pseudo-frame for the base frame.
    fn init_new_stack(&mut self) {
        // There is not previous frame, so point to an illegal value.
        self.push_word(STACK_SIZE as u16)?;
        // There is no continuation, so push zero.
        self.push_offset(0usize)?;
        // No return variable, so just push Global 0xef.
        self.push_byte(0 // ZVariable::Global doesn't exist yet.
            //u8::from(ZVariable::Global(0xef))
        )?;
        // There are no locals.
        self.push_byte(0)
    }

    #[throws]
    fn push_offset<Z: Into<ZOffset>>(&mut self, offset: Z) {
        let addr = usize::from(offset.into());
        self.push_word((addr >> 16 & 0xffff) as u16)?;
        self.push_word((addr >> 0 & 0xffff) as u16)?;
    }
}

impl Stack for ZStack {
    #[throws]
    fn push_byte(&mut self, val: u8) {
        ensure!(self.sp < STACK_SIZE,
        anyhow!("Pushed bytes off end of stack"));

        self.stack[self.sp] = val;
        self.sp += 1;
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