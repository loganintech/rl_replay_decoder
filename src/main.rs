#![allow(clippy::just_underscores_and_digits)]

use nom::{
    bytes::complete::take,
    error::ErrorKind,
    multi::{count, many0},
    number::complete::{le_f32, le_i32, le_u32, le_u64},
    IResult,

};

use std::borrow::Cow;

fn main() {
    let bytes: &[u8] = include_bytes!("../sample_replay.replay");

    if let Err(e) = parse(bytes) {
        eprintln!("Error: {:?}", e);
    }
}

fn parse(i: &[u8]) -> IResult<&[u8], ()> {
    let (i, header_len) = le_i32(i)?;
    let (i, _crc) = le_i32(i)?;
    let (i, header_data) = take(header_len as usize)(i)?;
    let (i, header) = parse_header(header_data)?;
    let header = dbg!(header);


    Ok((i, ()))
}

fn parse_header(i: &[u8]) -> IResult<&[u8], Header> {
    let (i, major) = le_i32(i)?;
    let (i, minor) = le_i32(i)?;
    let (i, net) = if major > 865 && minor > 17 {
        let (i, netver) = le_i32(i)?;
        (i, Some(netver))
    } else {
        (i, None)
    };
    let (i, game_type) = read_str(i)?;

    println!(
        "{:?}, {:?}, {:?}, {:?}, {:?}",
        major, minor, net, game_type, ""
    );

    let (i, properties) = many0(NamedProperty::from_bytes)(i)?;
    Ok((
        i,
        Header {
            major,
            minor,
            net,
            game_type: Cow::Borrowed(game_type),
            properties,
        },
    ))
}

#[derive(Debug)]
struct Header<'a> {
    major: i32,
    minor: i32,
    net: Option<i32>,
    game_type: Cow<'a, str>,
    properties: Vec<NamedProperty<'a>>,
}

#[derive(Debug)]
enum Property<'a> {
    Int(u32), // Little Endian
    Str(&'a str),
    Float(f32),
    Name(&'a str),
    Array(Vec<NamedProperty<'a>>),
    Byte(&'a [u8]),
    QWord(&'a [u8]),
    Empty,
}

#[derive(Debug)]
struct NamedProperty<'a> {
    prop: Property<'a>,
    name: &'a str,
}

use nom::Err;
impl<'b, 'a: 'b> NamedProperty<'b> {
    fn from_bytes(i: &'a [u8]) -> IResult<&[u8], NamedProperty<'b>> {
        println!("Starting Process: {:x?}", &i[..50]);
        let (i, name) = read_str(i)?;

        println!("Name: {:?}", name);
        let (i, prop_type) = read_str(i)?;

        println!("Type: {:?}", prop_type);

        if name == "None" {
            return Err(Err::Error((i, ErrorKind::Count))); // We're done parsing this section of props
        }

        let i = &i[8..]; //Throw these bytes away

        match prop_type {
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
                let (i, val) = read_str(i)?;
                println!("StrProp: {}", val);
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
                let (i, val) = read_str(i)?;

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
            "ByteProperty" => {
                let (i, len) = le_u32(i)?;
                let (i, data) = take(len as usize)(i)?;

                Ok((
                    i,
                    NamedProperty {
                        prop: Property::Byte(data),
                        name,
                    },
                ))
            }
            "QWordProperty" => {
                let (i, len) = le_u64(i)?;
                let (i, data) = take(len as usize)(i)?;

                Ok((
                    i,
                    NamedProperty {
                        prop: Property::QWord(data),
                        name,
                    },
                ))
            }
            _ => Err(Err::Error((i, ErrorKind::Complete))),
        }
    }
}

fn read_str(i: &[u8]) -> IResult<&[u8], &str> {
    let (i, len) = le_u32(i)?;
    println!("Read: {:x?}", &i[..len as usize]);
    let (i, word) = take(len)(i)?;
    Ok((
        i,
        std::ffi::CStr::from_bytes_with_nul(word)
            .unwrap()
            .to_str()
            .unwrap(),
    ))
}
