use std::error::Error;

use nom::{IResult, Parser};
use nom::bytes::complete::{take_until, take_while};
use nom::sequence::terminated;

fn parse_sentence(input: &str) -> IResult<&str, &str> {
    terminated(take_until("."), take_while(|c| c == '.')).parse(input)
}

fn main() -> Result<(), Box<dyn Error>> {
    let (rest, parsed) = parse_sentence("The senate will decide your fate. I am the Senate!")?;
    assert_eq!(parsed, "The senate will decide your fate");
    assert_eq!(rest, " I am the Senate!");

    let parsing_error = parse_sentence("You underestimate my power");
    assert!(parsing_error.is_err());

    Ok(())
}