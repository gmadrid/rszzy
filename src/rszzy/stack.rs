use crate::ensure;
use crate::rszzy::addressing::ZOffset;
use crate::rszzy::traits::Stack;
use crate::rszzy::variable::ZVariable;
use crate::rszzy::{bytes, constants::stack::STACK_SIZE};
use anyhow::{anyhow, Error};
use fehler::throws;

// Each frame has the following fields:
//
// fp: u16        - index on the stack of the previous frame.
//                  (The top frame has STACK_SIZE here which is an illegal value.)
// return_pc: u32 - Next pc value after returning.
// return_var: u8 - Encoded ZVariable for return value.
// num_locals: u8 - Number of local variables on the stack. (0-14)
// locals: u16    - One of these for each local, so up to 14.
//
// These following constants describe the frame.
const SAVED_FP_OFFSET: usize = 0;
const RETURN_PC_OFFSET: usize = 2;
const RETURN_VAR_OFFSET: usize = 6;
const NUM_LOCALS_OFFSET: usize = 7;
const LOCAL_VAR_OFFSET: usize = 8;

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
        self.push_byte(
            0x00, // ZVariable::Stack
        )?;
        // There are no locals.
        self.push_byte(0)?;
    }

    pub fn saved_fp(&self) -> usize {
        usize::from(bytes::word_from_slice(&self.stack, self.fp))
    }

    fn num_locals(&self) -> u8 {
        bytes::byte_from_slice(&self.stack, self.fp + NUM_LOCALS_OFFSET).into()
    }

    #[throws]
    fn push_offset<Z: Into<ZOffset>>(&mut self, offset: Z) {
        let addr = usize::from(offset.into());
        self.push_word((addr >> 16 & 0xffff) as u16)?;
        self.push_word((addr & 0xffff) as u16)?;
    }
}

impl Stack for ZStack {
    #[throws]
    fn push_byte(&mut self, val: u8) {
        ensure!(
            self.sp < STACK_SIZE,
            anyhow!("Pushed bytes off end of stack")
        );

        self.stack[self.sp] = val;
        self.sp += 1;
    }

    #[throws]
    fn pop_byte(&mut self) -> u8 {
        ensure!(self.sp > 0, anyhow!("Popped off an empty stack"));

        self.sp -= 1;
        self.stack[self.sp]
    }

    #[throws]
    fn push_frame(
        &mut self,
        return_pc: ZOffset,
        num_locals: u8,
        return_var: ZVariable,
        operands: &[u16],
    ) {
        // Steps:
        // - save sp to new_fp
        // - push fp
        // - save new_fp to fp
        // - push return_pc
        // - push return_var
        // - push num_locals
        // - push space for each local variable (initted to 0)
        // - set locals from operands
        // - set stack bottom to stack_next
        let new_fp = self.sp;
        let old_fp = dbg!(self.fp);
        self.push_word(old_fp as u16)?;
        self.fp = new_fp;
        self.push_offset(return_pc)?;
        self.push_byte(u8::from(return_var))?;
        self.push_byte(num_locals)?;
        for _ in 0..num_locals {
            self.push_word(0)?;
        }

        for (idx, op) in operands.iter().enumerate() {
            if idx >= num_locals.into() {
                // TODO: might want a warning here.
                break;
            }
            self.write_local(ZVariable::local(idx as u8)?, *op)?;
        }

        self.s0 = self.sp;
        dbg!("DONE");
    }

    #[throws]
    fn pop_frame(&mut self) {
        // Steps:
        // - Remember current fp (call it old_fp).
        // - Set fp to value from frame.
        // - Set sp to old_fp.
        // - Compute new value of s0.

        ensure!(
            self.saved_fp() < STACK_SIZE,
            anyhow!("Popped top stack frame.")
        );

        let old_fp = self.fp;
        self.sp = old_fp;
        let saved_fp = self.saved_fp();
        self.fp = saved_fp;

        self.s0 = self.fp + LOCAL_VAR_OFFSET + 2 * usize::from(self.num_locals());
    }

    #[throws]
    fn read_local(&self, var: ZVariable) -> u16 {
        ensure!(
            var.is_local(),
            anyhow!("Expected local variable, got {}", var)
        );
        ensure!(
            var.index().unwrap() < self.num_locals(),
            anyhow!(
                "Local out of range: {}, expected < {}",
                var.index().unwrap(),
                self.num_locals()
            )
        );
        bytes::word_from_slice(
            &self.stack,
            self.fp + LOCAL_VAR_OFFSET + usize::from(var.index()?) * 2,
        )
    }

    #[throws]
    fn write_local(&mut self, var: ZVariable, val: u16) {
        ensure!(
            var.is_local(),
            anyhow!("Expected local variable, got {}", var)
        );
        bytes::word_to_slice(
            &mut self.stack,
            self.fp + LOCAL_VAR_OFFSET + usize::from(var.index()?) * 2,
            val,
        );
    }

    fn return_pc(&self) -> usize {
        bytes::long_word_from_slice(&self.stack, self.fp + RETURN_PC_OFFSET) as usize
    }

    fn return_variable(&self) -> ZVariable {
        ZVariable::raw(bytes::byte_from_slice(
            &self.stack,
            self.fp + RETURN_VAR_OFFSET,
        ))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    // From the ZSpec 4.2.2
    const GLOBAL_EF_VARIABLE: u8 = 0xff;

    // These values are implementation specific. See comments for ZStack.
    const INITIAL_STACK_LEN: usize = 8;
    const INITIAL_STACK: [u8; INITIAL_STACK_LEN] = [
        // 2 bytes: Initial frame pointer (4096 - one past the stack size).
        0x10, 0x00, //
        // 4 bytes: Initial return PC (0x00 - cannot return from main)
        0x00, 0x00, 0x00, 0x00, //
        // Variable for return value (ZVariable::Stack.into()
        0x00, //
        // 0 locals
        0x00, //
    ];

    /// Return a slice of the stack, skipping the INITIAL_STACK component.
    /// (Most test functions don't care about the INITIAL_STACK part.)
    fn stack_slice_after_initial_bytes(stack: &ZStack) -> &[u8] {
        &stack.stack[INITIAL_STACK_LEN..stack.sp]
    }

    #[test]
    fn test_new_stack() {
        // The initial Z3 stack should look be properly formed.  See INITIAL_STACK.
        let stack = ZStack::default();

        assert_eq!(INITIAL_STACK, stack.stack[0..8]);
        assert_eq!(0, stack.fp);

        // Each frame is 8 bytes, so sp/s0 point to next byte.
        assert_eq!(8, stack.sp);
        assert_eq!(8, stack.s0);

        // FP should point to invalid value (one past end of the stack array).
        let fp = usize::from(bytes::word_from_slice(&stack.stack, 0usize));
        assert_eq!(STACK_SIZE, fp);

        // return_pc should be 0
        let return_pc = bytes::long_word_from_slice(&stack.stack, 2usize);
        assert_eq!(0, return_pc);

        // return value is Stack (0x00)
        assert_eq!(u8::from(ZVariable::stack()), stack.stack[6]);

        // and there are no locals.
        assert_eq!(0, stack.stack[7]);
    }

    #[throws]
    #[test]
    fn test_push_pop_byte() {
        let mut stack = ZStack::default();

        let starting_sp = stack.sp;

        stack.push_byte(0x12)?;
        stack.push_byte(0x34)?;
        stack.push_byte(0x56)?;

        assert_eq!(
            [0x12u8, 0x34, 0x56],
            stack_slice_after_initial_bytes(&stack)
        );

        assert_eq!(0x56, stack.pop_byte()?);
        assert_eq!([0x12u8, 0x34], stack_slice_after_initial_bytes(&stack));

        assert_eq!(0x34, stack.pop_byte()?);
        assert_eq!([0x12u8], stack_slice_after_initial_bytes(&stack));
        assert_eq!(0x12, stack.pop_byte()?);
        assert_eq!([0u8; 0], stack_slice_after_initial_bytes(&stack));

        assert_eq!(stack.sp, starting_sp);
    }

    // idx is a pointer to a frame.
    // returns the idx to the previous frame
    fn show_frame(stack: &ZStack, idx: u16) -> (String, u16) {
        let fp = bytes::word_from_slice(&stack.stack, stack.fp + SAVED_FP_OFFSET);
        let addr = bytes::long_word_from_slice(&stack.stack, stack.fp + RETURN_PC_OFFSET);
        let return_var = bytes::byte_from_slice(&stack.stack, stack.fp + RETURN_VAR_OFFSET);
        let num_locals = bytes::byte_from_slice(&stack.stack, stack.fp + NUM_LOCALS_OFFSET);
        (
            format!(
                "Frame at:{:x}, fp:{:x}, addr:{:x}, ret:{:x}, locals:{:x}",
                idx, fp, addr, return_var, num_locals
            ),
            fp,
        )
    }

    #[throws]
    #[test]
    fn test_show_frame() {
        let mut stack = ZStack::default();
        // Yes, I'm testing a testing function.
        assert_eq!(
            (
                "Frame at:0, fp:1000, addr:0, ret:0, locals:0".into(),
                0x1000
            ),
            show_frame(&stack, 0)
        );

        let frame_1_fp = stack.sp as u16;

        stack.push_frame(ZOffset::from(0x10203040), 5, ZVariable::stack(), &[])?;
        assert_eq!(
            ("Frame at:8, fp:0, addr:10203040, ret:0, locals:5".into(), 0),
            show_frame(&stack, frame_1_fp)
        );

        let frame_2_fp = stack.sp as u16;

        stack.push_frame(
            ZOffset::from(0x22446680),
            2,
            ZVariable::global(0xef)?,
            &[0x1357],
        )?;
        assert_eq!(
            (
                "Frame at:1a, fp:8, addr:22446680, ret:ff, locals:2".into(),
                8
            ),
            show_frame(&stack, frame_2_fp)
        );
    }

    #[throws]
    #[test]
    fn test_push_offset() {
        let mut stack = ZStack::default();

        let offset = ZOffset::from(0xdeafdeedusize);
        stack.push_offset(offset)?;

        assert_eq!(
            [0xde, 0xaf, 0xde, 0xed],
            stack_slice_after_initial_bytes(&stack)
        );
    }

    #[throws]
    #[test]
    fn test_push_pop_frame() {
        let mut stack = ZStack::default();

        // Push some values for testing.
        stack.push_byte(0x12)?;
        stack.push_byte(0x34)?;
        stack.push_byte(0x56)?;

        let saved_sp1 = stack.sp;
        let saved_frame_base1 = stack.s0;
        let saved_fp1 = stack.fp;

        stack.push_frame(
            ZOffset::from(0xdeafd00dusize),
            5,
            ZVariable::global(3)?,
            &[34, 38],
        )?;

        let saved_sp2 = stack.sp;
        let saved_frame_base2 = stack.s0;
        let saved_fp2 = stack.fp;

        stack.push_frame(
            ZOffset::from(0xbabef00dusize),
            7,
            ZVariable::local(5)?,
            &[1, 3, 5],
        )?;

        assert_eq!(saved_fp2, stack.saved_fp());
        assert_eq!(0xbabef00d, stack.return_pc());
        assert_eq!(ZVariable::local(5)?, stack.return_variable());
        assert_eq!(7, stack.num_locals());
        assert_eq!(1, stack.read_local(ZVariable::local(0)?)?);
        assert_eq!(3, stack.read_local(ZVariable::local(1)?)?);
        assert_eq!(5, stack.read_local(ZVariable::local(2)?)?);
        assert_eq!(0, stack.read_local(ZVariable::local(3)?)?);
        assert_eq!(0, stack.read_local(ZVariable::local(4)?)?);
        assert_eq!(0, stack.read_local(ZVariable::local(5)?)?);
        assert_eq!(0, stack.read_local(ZVariable::local(6)?)?);

        stack.pop_frame()?;

        assert_eq!(saved_fp1, stack.saved_fp());
        assert_eq!(0xdeafd00d, stack.return_pc());
        assert_eq!(ZVariable::global(3)?, stack.return_variable());
        assert_eq!(5, stack.num_locals());
        assert_eq!(34, stack.read_local(ZVariable::local(0)?)?);
        assert_eq!(38, stack.read_local(ZVariable::local(1)?)?);
        assert_eq!(0, stack.read_local(ZVariable::local(2)?)?);
        assert_eq!(0, stack.read_local(ZVariable::local(3)?)?);
        assert_eq!(0, stack.read_local(ZVariable::local(4)?)?);

        stack.pop_frame()?;

        assert_eq!(0x56, stack.pop_byte()?);
        assert_eq!(0x34, stack.pop_byte()?);
        assert_eq!(0x12, stack.pop_byte()?);
    }

    #[test]
    fn test_push_too_many_operands() {
        let mut stack = ZStack::default();

        stack
            .push_frame(
                ZOffset::from(0xbabef00dusize),
                2,
                ZVariable::stack(),
                &[11, 24, 36, 48],
            )
            .unwrap();

        assert_eq!(2, stack.num_locals());
        assert_eq!(11, stack.read_local(ZVariable::local(0).unwrap()).unwrap());
        assert_eq!(24, stack.read_local(ZVariable::local(1).unwrap()).unwrap());

        // TODO: add a test to ensure that all of the operands don't get pushed.
    }

    #[test]
    fn test_local_range_check() {
        let mut stack = ZStack::default();

        stack
            .push_frame(ZOffset::from(0xbabef00dusize), 1, ZVariable::stack(), &[22])
            .unwrap();

        assert_eq!(22, stack.read_local(ZVariable::local(0).unwrap()).unwrap());
        assert!(stack.read_local(ZVariable::local(1).unwrap()).is_err());
    }

    #[test]
    fn test_pop_missing_stack_frame() {
        let mut stack = ZStack::default();

        assert!(stack.pop_frame().is_err());
    }

    #[test]
    fn test_stack_frame_overflow() {
        let mut stack = ZStack::default();

        // 170 stack frames if an many as fit on the current-sized frame.
        for _ in 0..170 {
            stack.push_frame(ZOffset::from(0x1000usize), 8, ZVariable::stack(), &[]).unwrap();
        }

        assert!(stack.push_frame(ZOffset::from(0x2000usize), 8, ZVariable::stack(), &[]).is_err());
    }
}
