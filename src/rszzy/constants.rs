// Constants for offsets into the header.
pub mod header_offset {
    pub const VERSION_NUMBER: usize = 0x00;
    pub const HIGH_MEMORY_MARK: usize = 0x04;
    pub const STATIC_MEMORY_START: usize = 0x0e;
}
