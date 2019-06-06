#![allow(clippy::just_underscores_and_digits)]

use nom::{
    bytes::complete::take,
    error::ErrorKind,
    multi::{count, many0},
    number::complete::{le_f32, le_i32, le_u32, le_u64},
    IResult,
};


use std::env::args;
use std::fs::File;
use std::io::Read;

use std::borrow::Cow;

fn main() {
    let mut file = File::open(args().nth(1).expect("You must include a replay to parse."))
        .expect("Couldn't open file for reading.");
    let mut bytes: Vec<u8> = Vec::new();
    file.read_to_end(&mut bytes)
        .expect("Couldn't read bytes of file.");

    let (rest, replay) = parse(&bytes).unwrap_or_else(|e| {
        eprintln!("Error: {:?}", e);
        std::process::exit(1);
    });

    let replay: Replay = dbg!(replay);
    for prop in replay.header.properties {
        if prop.name == "Goals" {
            match prop.prop {
                Property::Array(goals) => {
                    for goal in goals.windows(4).step_by(4) {
                        println!(
                            "[{}] Goal! {} - {}",
                            if let Property::Int(val) = goal[0].prop {
                                val
                            } else {
                                eprintln!("{:?}", goal[0].prop);
                                unreachable!()
                            },
                            if let Property::Str(val) = goal[1].prop {
                                val
                            } else {
                                eprintln!("{:?}", goal[1].prop);
                                unreachable!()
                            },
                            if let Property::Int(val) = goal[2].prop {
                                val
                            } else {
                                eprintln!("{:?}", goal[2].prop);
                                unreachable!()
                            },
                        );

                    }
                }
                _ => unreachable!(),
            }
        }
    }

}

fn parse(i: &[u8]) -> IResult<&[u8], Replay> {
    let (i, header_len) = le_i32(i)?;
    let (i, _crc) = le_i32(i)?;
    let (i, header_data) = take(header_len as usize)(i)?;
    let (_, header) = parse_header(header_data)?;
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

    // println!(
    //     "{:?}, {:?}, {:?}, {:?}, {:?}",
    //     major, minor, net, game_type, ""
    // );

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
struct Body {}

#[derive(Debug)]
struct Replay<'a> {
    header: Header<'a>,
    body: Body,
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
    None,
}

#[derive(Debug)]
struct NamedProperty<'a> {
    name: &'a str,
    prop: Property<'a>,
}

use nom::Err;
impl<'prop, 'dat: 'prop> NamedProperty<'prop> {
    fn from_bytes(i: &'dat [u8]) -> IResult<&[u8], NamedProperty<'prop>> {
        // println!("Starting Process: {:x?}", &i[..50]);
        let (i, name) = read_str(i)?;

        // print!("Name: {:?}, ", name);

        if name == "None" {
            // println!("\nFound None");
            return Ok((
                i,
                NamedProperty {
                    prop: Property::None,
                    name,
                },
            )); // We're done parsing this section of props
        }

        let (i, prop_type) = read_str(i)?;
        // print!("Type: {:?}, ", prop_type);
        let i = &i[8..]; //Throw these bytes away


        match prop_type {
            "IntProperty" => {
                let (i, val) = le_u32(i)?;
                // println!("{}", val);
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
                // println!("{}", val);
                Ok((
                    i,
                    NamedProperty {
                        prop: Property::Str(val),
                        name,
                    },
                ))
            }
            "FloatProperty" => {
                let (i, val) = le_f32(i)?;
                // println!("{}", val);
                Ok((
                    i,
                    NamedProperty {
                        prop: Property::Float(val),
                        name,
                    },
                ))
            }
            "NameProperty" => {
                let (i, val) = read_str(i)?;
                // println!("{}", val);
                Ok((
                    i,
                    NamedProperty {
                        prop: Property::Name(val),
                        name,
                    },
                ))
            }
            "ArrayProperty" => {
                let (i, mut len) = le_u32(i)?;
                len *= 4;
                let (i, props) = count(NamedProperty::from_bytes, len as usize)(i)?;
                // println!("{:x?}", props);
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
                // println!("{:x?}", data);
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
                // println!("{:x?}", data);
                Ok((
                    i,
                    NamedProperty {
                        prop: Property::QWord(data),
                        name,
                    },
                ))
            }
            _ => Err(Err::Error((i, ErrorKind::AlphaNumeric))),
        }
    }
}

fn read_str(i: &[u8]) -> IResult<&[u8], &str> {
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
