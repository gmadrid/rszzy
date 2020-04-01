use anyhow::{anyhow, Result};

struct ZCharIter<'a> {
    buf: &'a [u8],
}

impl<'a> ZCharIter<'a> {
    fn new(buf: &[u8]) -> Result<ZCharIter> {
        if buf.is_empty() {
            return Err(anyhow!("Empty ZString not allowed."));
        }
        if buf.len() %2 != 0 {
            return Err(anyhow!("ZString must have even number of bytes."));
        }
        
        Ok(ZCharIter { buf })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn test_empty() {
        assert!(ZCharIter::new(&[]).is_err());
    }

    #[test]
    fn test_odd() {
        assert!(ZCharIter::new(&[3]).is_err());
        assert!(ZCharIter::new(&[3, 4, 5]).is_err());
        assert!(ZCharIter::new(&[3, 4, 5, 6, 7]).is_err());
    }
}
