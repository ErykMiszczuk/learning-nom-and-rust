extern crate nom;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::Parser;
use nom::IResult;
use std::error::Error;

fn parse_abc(input: &str) -> IResult<&str, &str> {
  tag("abc")(input)
}

fn parse_def_or_ghi(input: &str) -> IResult<&str, &str> {
    alt((
        tag("def"),
        tag("ghi")
    )).parse(input)
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = "abcghi";
    let (rest, abc) = parse_abc(input)?;
    let (_rest, def_or_ghi) = parse_def_or_ghi(rest)?;
    println!("first parsed: {abc}; then parsed: {def_or_ghi};");
    Ok(())
}