use std::error::Error;
use nom::bytes::complete::tag;
use nom::character::complete::i32;
use nom::IResult;
use nom::sequence::{delimited, separated_pair};

#[derive(Debug, PartialEq)]
pub struct Coordinate {
    pub x: i32,
    pub y: i32,
}

// format: "x,y"
fn parse_int_pair(input: &str) -> IResult<&str, (i32, i32)> {
    separated_pair(i32, tag(","), i32)(input)
}

// format: "(x,y)"
fn parse_coordinate(input: &str) -> IResult<&str, Coordinate>{
    let (remain,(x,y)) = delimited(
        tag("("),
        parse_int_pair,
        tag(")")
    )(input)?;

    Ok((remain,Coordinate{x,y}))
}

fn main() -> Result<(),Box<dyn Error>>{
    let (_,rs) = parse_coordinate("(77,88)")?;
    assert_eq!(rs,Coordinate{x:77,y:88});

    let (_,rs) = parse_coordinate("(-128,127)")?;
    assert_eq!(rs,Coordinate{x:-128,y:127});

    let err_rs = parse_coordinate("(150,)");
    assert!(err_rs.is_err());


    Ok(())
}