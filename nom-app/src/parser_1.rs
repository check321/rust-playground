use std::error::Error;
use nom::bytes::complete::tag;
use nom::IResult;

fn parse_input(input:&str) -> IResult<&str,&str>{
    tag("hello")(input)
}

fn main() -> Result<(),Box<dyn Error>>{
    let (remain,output) = parse_input("hello rust")?;
    assert_eq!(remain," rust");
    assert_eq!(output,"hello");

    assert!(parse_input("hola rust").is_err());
    Ok(())
}