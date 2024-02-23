use std::error::Error;
use std::num::ParseIntError;
use nom::{IResult};
use nom::bytes::complete::{tag, take_while_m_n};
use nom::combinator::map_res;
use nom::sequence::Tuple;

#[derive(Debug, PartialEq)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

fn from_hex(input: &str) -> Result<u8, ParseIntError> {
    u8::from_str_radix(input, 16)
}

fn is_hex_digit(c: char) -> bool {
    c.is_digit(16)
}


// 1: pick 2 color codes every time.
// 2: invoke from_hex if is_hex_digit.
fn hex_primary(input: &str) -> IResult<&str, u8> {
    map_res(take_while_m_n(2, 2, is_hex_digit), from_hex)(input)
}

// #2F329F | #00FFFF
fn hex_color(input: &str) -> IResult<&str, Color> {
    let (input, _) = tag("#")(input)?;
    let (input, (red, green, blue)) = (hex_primary, hex_primary, hex_primary).parse(input)?;

    Ok((input, Color { red, green, blue }))
}

#[test]
fn parse_color() {
    assert_eq!(hex_color("#2F329F"),
               Ok(("", Color { red: 47, green: 50, blue: 159 }))
    )
}

fn main() -> Result<(),Box<dyn Error>>{
    let (_,rs) = hex_color("#2F329F")?;
    println!("{:?}",rs);

    Ok(())
}