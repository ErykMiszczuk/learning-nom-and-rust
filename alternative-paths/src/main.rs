extern crate nom;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::Parser;
use nom::IResult;
use std::error::Error;

fn parse_anc_or_def(input: &str) -> IResult<&str, &str> {
    alt((
        tag("abc"),
        tag("def")
    )).parse(input)
}

fn main() -> Result<(), Box<dyn Error>> {
    let (leftover_input, output) = parse_anc_or_def("abcWorld")?;
    assert_eq!(leftover_input, "World");
    assert_eq!(output, "abc");

    assert!(parse_anc_or_def("ghiWorld").is_err());
    Ok(())
}