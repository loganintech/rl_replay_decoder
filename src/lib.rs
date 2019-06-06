pub mod header;
mod utils;

use header::{Header, NamedProperty};
use utils::read_str;

use nom::{bytes::complete::take, number::complete::le_i32, IResult};

#[derive(Debug)]
struct Body {}

#[derive(Debug)]
pub struct Replay<'a> {
    pub header: Header<'a>,
    body: Body,
}

pub fn parse(i: &[u8]) -> IResult<&[u8], Replay> {
    let (i, header_len) = le_i32(i)?;
    let (i, _crc) = le_i32(i)?;
    let (i, header_data) = take(header_len as usize)(i)?;
    let (_, header) = Header::parse_header(header_data)?;
    // let (i, properties) = many0(NamedProperty::from_bytes)(i)?;

    // let header = dbg!(header);
    // println!("Rest of Data: {:x?}", &i[..20]);

    Ok((
        i,
        Replay {
            header,
            body: Body {},
        },
    ))
}
