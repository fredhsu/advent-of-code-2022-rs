use color_eyre::owo_colors::OwoColorize;
use nom::branch::alt;
use nom::bytes::complete::*;
use nom::character::complete::*;
use nom::combinator::*;
use nom::multi::separated_list1;
use nom::sequence::*;
use nom::IResult;

enum Packet {
    List(Vec<Packet>),
    Integer(u32),
}
pub fn parse_packets(i: &str) -> IResult<&str, &str> {
    let parser = delimited(tag("["), tag(","), tag("]"))(i)?;
    Ok(parser)
}

fn parse_packet(i: &str) -> IResult<&str, &str> {
    // separated list or a single packet or delimited
    let parser = separated_list1(",")(i)?;
}

pub fn day_thirteen() {
    let lines = include_str!("../input/day13-test.txt");
    //let mut monkeys = all_consuming(parse_monkeys)(lines).unwrap().1;
}
