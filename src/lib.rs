pub mod body;
pub mod header;
mod utils;

use body::Body;
use header::Header;

use nom::{bytes::complete::take, number::complete::le_i32, IResult};

#[derive(Debug)]
pub struct Replay<'a> {
    pub header: Header<'a>,
    body: Body,
}

pub fn parse(i: &[u8]) -> IResult<&[u8], Replay> {
    let (i, header_len) = le_i32(i)?;
    let (i, _crc) = le_i32(i)?;
    let (i, header_data) = take(header_len as usize)(i)?;
    let (_leftover_from_header, header) = Header::parse(header_data)?;
    let (rest, body) = Body::parse(i)?;
    // let header = dbg!(header);
    // println!("Rest of Data: {:x?}", &i[..20]);

    Ok((i, Replay { header, body }))
}
