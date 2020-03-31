use super::addressing::{ZOffset};

pub trait Version {
    const NUMBER: u8;

    fn routine_offset(word: u16) -> ZOffset;
    fn string_offset(word: u16) -> ZOffset;
}

#[derive(Debug, Clone, Copy)]
pub struct V3;

impl Version for V3 {
    const NUMBER: u8 = 3;

    fn routine_offset(word: u16) -> ZOffset {
        ZOffset::from(usize::from(word * 2))
    }

    fn string_offset(word: u16) -> ZOffset {
        ZOffset::from(usize::from(word * 2))
    }
}

#[derive(Debug, Clone, Copy)]
pub struct V5;

impl Version for V5 {
    const NUMBER: u8 = 5;

    fn routine_offset(word: u16) -> ZOffset {
        ZOffset::from(usize::from(word * 4))
    }

    fn string_offset(word: u16) -> ZOffset {
        ZOffset::from(usize::from(word * 4))
    }
}
