use nom::{bytes::complete::take, number::complete::le_u32, IResult};

pub fn read_str(i: &[u8]) -> IResult<&[u8], &str> {
    let (i, len) = le_u32(i)?;
    // println!("Read: {:x?}", &i[..len as usize]);
    let (i, word) = take(len)(i)?;
    Ok((
        i,
        std::ffi::CStr::from_bytes_with_nul(word)
            .unwrap()
            .to_str()
            .unwrap(),
    ))
}
