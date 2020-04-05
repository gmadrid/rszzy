use super::addressing::ByteAddress;
use super::constants::header_offset;
use super::traits::Memory;

pub struct Header;

impl Header {
    pub fn start_pc(memory: &impl Memory) -> ByteAddress {
        let addr = memory.read_word(header_offset::START_PC).unwrap();
        ByteAddress::raw(addr)
    }
}
