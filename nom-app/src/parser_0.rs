mod parser_1;
mod parser_coordinate;
mod parser_16digit_color;

use std::error::Error;
use nom::IResult;
pub fn do_nothing_parser(input:&str) -> IResult<&str ,&str>{
    Ok((input,""))
}

fn main() -> Result<(),Box<dyn Error>>{
    let (remain, output) = do_nothing_parser("let's get hands dirty.")?;
    // parser_0 do nothing.
    assert_eq!(remain,"let's get hands dirty.");
    assert_eq!(output,"");
    Ok(())
}