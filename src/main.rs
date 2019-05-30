#![allow(clippy::just_underscores_and_digits)]

use nom::{
    bytes::complete::take,
    multi::count,
    number::complete::{be_f32, be_u32, le_f32, le_u32},
    Err, IResult,
};

fn main() {
    let bytes: &[u8] = include_bytes!("../sample_replay.replay");

    if let Err(e) = parse(bytes) {
        eprintln!("Error: {}", e.description());
    }
}

fn parse(bytes: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
    let (i, checksum) = (&bytes[20..], &bytes[..20]);
    let (i, _header_start) = (&i[24..], &i[..24]);
    let (i, header) = NamedProperty::from_bytes(i).map_err(|_| "Couldn't parse named property.")?;
    Ok(())
}

fn load_checksum(i: &[u8]) -> IResult<&[u8], &[u8]> {
    take(20u8)(i)
}

#[derive(Debug)]
enum Property<'a> {
    Int(u32), // Little Endian
    Str(&'a str),
    Float(f32),
    Name(&'a str),
    Array(Vec<NamedProperty<'a>>),
    Empty,
}

#[derive(Debug)]
struct NamedProperty<'a> {
    prop: Property<'a>,
    name: &'a str,
}

impl<'b, 'a: 'b> NamedProperty<'b> {
    fn from_bytes(i: &'a [u8]) -> IResult<&[u8], NamedProperty<'b>> {

        let (i, len) = le_u32(i)?;
        let (i, name) = read_str(i, len as usize)?;


        if name == "None" {
            return Ok((
                i,
                NamedProperty {
                    prop: Property::Empty,
                    name,
                },
            ));
        }

        let (i, type_len) = le_u32(i)?;
        let (i, name) = read_str(i, type_len as usize)?;

        match name {
            "IntProperty" => {
                let (i, val) = le_u32(i)?;
                Ok((
                    i,
                    NamedProperty {
                        prop: Property::Int(val),
                        name,
                    },
                ))
            }
            "StrProperty" => {
                let (i, len) = le_u32(i)?;
                let (i, val) = read_str(i, len as usize)?;
                Ok((
                    i,
                    NamedProperty {
                        prop: Property::Str(val),
                        name,
                    },
                ))
            }
            "FloatProperty" => {
                let (i, len) = le_f32(i)?;
                Ok((
                    i,
                    NamedProperty {
                        prop: Property::Float(len),
                        name,
                    },
                ))
            }
            "NameProperty" => {
                let (i, len) = le_u32(i)?;
                let (i, val) = read_str(i, len as usize)?;
                Ok((
                    i,
                    NamedProperty {
                        prop: Property::Name(val),
                        name,
                    },
                ))
            }
            "ArrayProperty" => {
                let (i, len) = le_u32(i)?;
                let (i, props) = count(NamedProperty::from_bytes, len as usize)(i)?;
                Ok((
                    i,
                    NamedProperty {
                        prop: Property::Array(props),
                        name,
                    },
                ))
            }
            _ => Ok((
                i,
                NamedProperty {
                    prop: Property::Empty,
                    name,
                },
            )),
        }
    }
}

fn read_str(i: &[u8], len: usize) -> IResult<&[u8], &str> {
    let (i, word) = take(len)(i)?;
    Ok((i, std::str::from_utf8(word).unwrap()))
}
