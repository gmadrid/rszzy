/// Offsets into Header memory (the first 64bytes).
/// See ZSpec 11 for details.
pub mod header_offset {
    pub const VERSION_NUMBER: usize = 0x00;
    pub const HIGH_MEMORY_MARK: usize = 0x04;
    pub const START_PC: usize = 0x06;
    pub const STATIC_MEMORY_START: usize = 0x0e;
}
