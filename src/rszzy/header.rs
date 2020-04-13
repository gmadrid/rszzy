use crate::rszzy::addressing::ByteAddress;
use crate::rszzy::constants::header_offset::START_PC;
use crate::rszzy::traits::Memory;

pub struct Header;

impl Header {
    pub fn start_pc(memory: &impl Memory) -> ByteAddress {
        let addr = memory.read_word(START_PC).unwrap();
        ByteAddress::raw(addr)
    }
}
