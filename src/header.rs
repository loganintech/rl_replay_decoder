use nom::{
    bytes::complete::take,
    error::ErrorKind,
    multi::{count, many0},
    number::complete::{le_f32, le_i32, le_u32, le_u64},
    Err, IResult,
};

use std::borrow::Cow;

use crate::utils::read_str;

#[derive(Debug)]
pub struct Header<'a> {
    pub major: i32,
    pub minor: i32,
    pub net: Option<i32>,
    pub game_type: Cow<'a, str>,
    pub game_properties: Vec<NamedProperty<'a>>,
    pub player_stats: Vec<NamedProperty<'a>>,
}

impl<'a> Header<'a> {
    pub fn parse(i: &'a [u8]) -> IResult<&'a [u8], Header> {
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

        let (i, game_properties) = many0(NamedProperty::from_bytes)(i)?;
        Ok((
            i,
            Header {
                major,
                minor,
                net,
                game_type: Cow::Borrowed(game_type),
                game_properties,
                player_stats: vec![],
            },
        ))
    }
}

#[derive(Debug)]
pub enum Property<'a> {
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
pub struct NamedProperty<'a> {
    pub name: &'a str,
    pub prop: Property<'a>,
}

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
            prop => {
                // println!("Prop: {:x?}", prop);
                Err(Err::Error((i, ErrorKind::AlphaNumeric)))
            }
        }
    }
}
