use std::fmt;
use anyhow::{anyhow, Error};
use fehler::{throw,throws};
use crate::ensure;

// These numbers all come from ZSpec 4.2.2.
// _RAW values are before converting to ZVariables. So, the Local represented by Raw 1 is L00.
// The MAX_ values are all after conversion, so the "last" local is L0e.
pub const LOCAL_START_RAW: u8 = 0x01;
pub const LOCAL_END_RAW: u8 = 0x0f;
pub const MAX_LOCAL: u8 = LOCAL_END_RAW - LOCAL_START_RAW;
pub const GLOBAL_START_RAW: u8 = 0x10;
pub const GLOBAL_END_RAW: u8 = 0xff;
pub const MAX_GLOBAL: u8 = GLOBAL_END_RAW - GLOBAL_START_RAW;

/// A variable location as described in ZSpec 4.2.2
/// Defaults to Stack.
#[derive(Copy, Clone, Default, Eq, PartialEq)]
pub struct ZVariable(u8);

impl ZVariable {
    pub fn raw(byte: u8) -> ZVariable {
        ZVariable(byte)
    }

    pub fn stack() -> ZVariable {
        ZVariable(0)
    }

    pub fn is_stack(&self) -> bool {
        self.0 == 0
    }

    #[throws]
    pub fn local(lidx: u8) -> ZVariable {
        ensure!((0..=MAX_LOCAL).contains(&lidx), anyhow!("Local variable out of range: {}", lidx) );
        ZVariable(lidx + LOCAL_START_RAW)
    }

    pub fn is_local(&self) -> bool {
        (LOCAL_START_RAW..=LOCAL_END_RAW).contains(&self.0)
    }

    #[throws]
    pub fn global(gidx: u8) -> ZVariable {
        ensure!((0..=MAX_GLOBAL).contains(&gidx), anyhow!("Global variable out of range: {}", gidx));
        ZVariable(gidx + GLOBAL_START_RAW)
    }

    pub fn is_global(&self) -> bool {
        (GLOBAL_START_RAW..=GLOBAL_END_RAW).contains(&self.0)
    }

    #[throws]
    pub fn index(&self) -> u8 {
        match self.0 {
            LOCAL_START_RAW..=LOCAL_END_RAW => self.0 - LOCAL_START_RAW,
            GLOBAL_START_RAW ..= GLOBAL_END_RAW => self.0 - GLOBAL_START_RAW,
            _ => throw!(anyhow!("Cannot provide index for Stack variable.")),
        }
    }
}

impl fmt::Display for ZVariable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_stack() {
            write!(f, "SP")
        } else if self.is_local() {
            // unwrap: should be safe because is_local == true.
            write!(f, "L{:02x}", self.index().unwrap())
        } else {
            // unwrap: should be safe because is_global == true
            write!(f, "G{:02x}", self.index().unwrap())
        }
    }
}

impl fmt::Debug for ZVariable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl From<ZVariable> for u8 {
    fn from(var: ZVariable) -> Self {
        var.0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_stack() {
        let var = ZVariable::stack();

        assert!(var.is_stack());
        assert!(!var.is_local());
        assert!(!var.is_global());
        assert!(var.index().is_err());
    }

    #[test]
    fn test_locals() {
        assert!(ZVariable::local(0).is_ok());
        assert!(ZVariable::local(MAX_LOCAL).is_ok());
        assert!(ZVariable::local(MAX_LOCAL + 1).is_err());

        let min = ZVariable::local(0).unwrap();
        let max = ZVariable::local(MAX_LOCAL).unwrap();

        assert_eq!(0, min.index().unwrap());
        assert_eq!(MAX_LOCAL, max.index().unwrap());

        assert!(!min.is_stack());
        assert!(min.is_local());
        assert!(!min.is_global());

        assert!(!max.is_stack());
        assert!(max.is_local());
        assert!(!max.is_global());
    }

    #[test]
    fn test_globals() {
        assert!(ZVariable::global(0).is_ok());
        assert!(ZVariable::global(MAX_GLOBAL).is_ok());
        assert!(ZVariable::global(MAX_GLOBAL + 1).is_err());

        let min = ZVariable::global(0).unwrap();
        let max = ZVariable::global(MAX_GLOBAL).unwrap();

        assert_eq!(0, min.index().unwrap());
        assert_eq!(MAX_GLOBAL, max.index().unwrap());

        assert!(!min.is_stack());
        assert!(!min.is_local());
        assert!(min.is_global());

        assert!(!max.is_stack());
        assert!(!max.is_local());
        assert!(max.is_global());
    }

}