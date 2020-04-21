#[inline]
pub fn byte_from_slice<I>(slice: &[u8], idx: I) -> u8
    where
        I: Into<usize> + Copy,
{
    slice[idx.into()]
}

#[inline]
pub fn byte_to_slice<I>(slice: &mut [u8], idx: I, val: u8)
    where
        I: Into<usize> + Copy,
{
    slice[idx.into()] = val;
}

#[inline]
pub fn word_from_slice<I>(slice: &[u8], idx: I) -> u16
    where
        I: Into<usize> + Copy,
{
    let high_byte = u16::from(byte_from_slice(slice, idx));
    let low_byte = u16::from(byte_from_slice(slice, idx.into() + 1));

    (high_byte << 8) + low_byte
}

#[inline]
pub fn word_to_slice<I>(slice: &mut [u8], idx: I, val: u16)
    where
        I: Into<usize> + Copy,
{
    let high_byte = ((val >> 8) & 0xff) as u8;
    let low_byte = (val & 0xff) as u8;

    // big-endian
    byte_to_slice(slice, idx, high_byte);
    byte_to_slice(slice, idx.into() + 1, low_byte);
}

#[inline]
pub fn long_word_from_slice<I>(slice: &[u8], idx: I) -> usize
where I: Into<usize> + Copy {
    let high_word = usize::from(word_from_slice(slice, idx));
    let low_word = usize::from(word_from_slice(slice, idx.into() + 2));

    (high_word << 16) + low_word
}