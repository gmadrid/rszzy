use std::char::{decode_utf16, REPLACEMENT_CHARACTER};

const V2_ALPHA_TABLE: &[u8] =
    b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ \n0123456789.,!?_#'\"/\\-:()";

struct ZSCII(u16);

struct ZString<'a> {
    zchars: ZCharIter<'a>,

    active_charset: u8,
}

impl<'a> ZString<'a> {
    fn new(buf: &[u8]) -> ZString {
        ZString {
            zchars: ZCharIter::new(buf),

            active_charset: 0,
        }
    }
}

impl From<ZString<'_>> for String {
    fn from(zs: ZString) -> String {
        decode_utf16(zs.map(|zc| zc.0))
            .map(|r| r.unwrap_or(REPLACEMENT_CHARACTER))
            .collect::<String>()
    }
}

impl Iterator for ZString<'_> {
    type Item = ZSCII;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let n = self.zchars.next();
            if let Some(zc) = n {
                match zc {
                    0 => return Some(ZSCII(b' ' as u16)),
                    1..=3 => unimplemented!("abbrev table unimplemented"),
                    4 => self.active_charset = 1,
                    5 => self.active_charset = 2,
                    6..=31 => {
                        if zc == 6 && self.active_charset == 2 {
                            unimplemented!("10-bit ZSCII unimplemented");
                        }

                        let idx: usize = (26 * self.active_charset + zc - 6) as usize;
                        self.active_charset = 0;
                        return Some(ZSCII(V2_ALPHA_TABLE[idx] as u16));
                    }
                    _ => {
                        panic!("Some zchar out of range: {}", zc);
                    }
                }
            } else {
                return None;
            }
        }
    }
}

struct ZCharIter<'a> {
    done: bool,
    buf: &'a [u8],

    zch_idx: usize,
}

impl<'a> ZCharIter<'a> {
    fn new(buf: &[u8]) -> ZCharIter {
        ZCharIter {
            buf,
            zch_idx: 0,
            done: buf.is_empty(),
        }
    }
}

impl<'a> Iterator for ZCharIter<'a> {
    type Item = u8;

    fn next(&mut self) -> Option<u8> {
        if self.done {
            return None;
        }

        let buf_idx = (self.zch_idx / 3) * 2;
        let sub_idx = self.zch_idx % 3;
        self.zch_idx += 1;

        if sub_idx == 0 {
            Some((self.buf[buf_idx] & 0b0111_1100) >> 2)
        } else if sub_idx == 1 {
            if buf_idx + 1 >= self.buf.len() {
                self.done = true;
                None
            } else {
                Some(
                    ((self.buf[buf_idx] & 0b0000_0011) << 3)
                        + ((self.buf[buf_idx + 1] & 0b1110_0000) >> 5),
                )
            }
        } else {
            /* sub_idx == 2 */
            self.done = ((self.buf[buf_idx] & 0b1000_0000) != 0) || (buf_idx + 2 >= self.buf.len());
            Some(self.buf[buf_idx + 1] & 0b000_11111)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_empty() {
        let i = ZCharIter::new(&[]);
        let vec = i.collect::<Vec<_>>();
        assert_eq!(Vec::<u8>::default(), vec);
    }

    #[test]
    fn test_odd() {
        let i = ZCharIter::new(&[0b1_00100_01]);
        let vec = i.collect::<Vec<_>>();
        assert_eq!(vec![0b00100], vec);
    }

    #[test]
    fn test_three() {
        let i = ZCharIter::new(&[0b1_00100_01, 0b111_00011]);
        let vec = i.collect::<Vec<_>>();
        assert_eq!(vec![0b00100, 0b01111, 0b00011], vec);
    }

    #[test]
    fn test_next_after_done() {
        let mut i = ZCharIter::new(&[0b1_00100_01, 0b111_00011]);

        assert_eq!(0b00100, i.next().unwrap());
        assert_eq!(0b01111, i.next().unwrap());
        assert_eq!(0b00011, i.next().unwrap());
        assert_eq!(None, i.next());
        assert_eq!(None, i.next());
        assert_eq!(None, i.next());
    }

    #[test]
    fn test_six() {
        let i = ZCharIter::new(&[0b0_00100_01, 0b111_00011, 0b1_10101_11, 0b111_00101]);
        let vec = i.collect::<Vec<_>>();
        assert_eq!(
            vec![0b00100, 0b01111, 0b00011, 0b10101, 0b11111, 0b00101],
            vec
        );
    }

    #[test]
    fn test_six_no_terminator() {
        let i = ZCharIter::new(&[0b0_00100_01, 0b111_00011, 0b0_10101_11, 0b111_00101]);
        let vec = i.collect::<Vec<_>>();
        assert_eq!(
            vec![0b00100, 0b01111, 0b00011, 0b10101, 0b11111, 0b00101],
            vec
        );
    }

    #[test]
    fn test_three_of_six() {
        let i = ZCharIter::new(&[0b1_00100_01, 0b111_00011, 0b1_10101_11, 0b111_00101]);
        let vec = i.collect::<Vec<_>>();
        assert_eq!(vec![0b00100, 0b01111, 0b00011], vec);
    }

    #[test]
    fn test_empty_string() {
        let zs = ZString::new(&[0b1001_0100, 0b1010_0101]); // [4, 4, 4]
        assert_eq!("", String::from(zs));
    }

    #[test]
    fn test_aaa_string() {
        let zs = ZString::new(&[0b1001_1000, 0b1100_0110]);
        assert_eq!("aaa", String::from(zs));
    }

    #[test]
    fn test_abc_string() {
        let zs = ZString::new(&[0b1001_1000, 0b1110_1000]);
        assert_eq!("abc", String::from(zs));
    }

    #[allow(non_snake_case)]
    #[test]
    fn test_aA_string() {
        let zs = ZString::new(&[0b1001_1000, 0b1000_0110]);
        assert_eq!("aA", String::from(zs));
    }

    #[allow(non_snake_case)]
    #[test]
    fn test_Zc_string() {
        let zs = ZString::new(&[0b1001_0011, 0b1110_1000]);
        assert_eq!("Zc", String::from(zs));
    }

    #[allow(non_snake_case)]
    #[test]
    fn test_cSPACEEPOINT_string() {
        let zs = ZString::new(&[0b0010_0000, 0b0000_0101, 0b1101_0000, 0b1000_0100]);
        assert_eq!("c !", String::from(zs));
    }

    #[allow(non_snake_case)]
    #[test]
    fn test_Charlie_Brown_string() {
        let zs = ZString::new(&[
            0b0001_0001,
            0b0000_1101, // Ch
            0b0001_1010,
            0b1111_0001, // arl
            0b0011_1001,
            0b0100_0000, // "ie "
            0b0001_0000,
            0b1111_0111, // Br
            0b0101_0011,
            0b1001_0011, // own
            0b1001_0110,
            0b1010_0100, // ?
        ]);
        assert_eq!("Charlie Brown?", String::from(zs));
    }
}
