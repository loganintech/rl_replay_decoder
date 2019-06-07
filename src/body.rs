use nom::IResult;

#[derive(Debug)]
pub struct Body {}

impl Body {
    pub fn parse<'a>(i: &'a [u8]) -> IResult<&'a [u8], Body> {
        Ok((i, Body {}))
    }
}
