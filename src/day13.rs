use color_eyre::owo_colors::OwoColorize;
use nom::branch::alt;
use nom::bytes::complete::*;
use nom::character::complete::*;
use nom::combinator::*;
use nom::multi::separated_list1;
use nom::sequence::*;
use nom::IResult;

fn parse_monkey(s: &str) -> IResult<&str, Monkey> {
    let (i, _) = tuple((tag("Monkey "), u64, tag(":\n")))(s)?;
    let (i, (_, _, items, _)) = tuple((
        space1,
        tag("Starting items: "),
        separated_list1(tag(", "), u64),
        tag("\n"),
    ))(i)?;
    let (i, (_, _, operation, _)) =
        tuple((space1, tag("Operation: "), parse_operation, tag("\n")))(i)?;
    let (i, (_, _, divisor, _)) = tuple((space1, tag("Test: divisible by "), u64, tag("\n")))(i)?;
    let (i, (_, _, true_target, _)) =
        tuple((space1, tag("If true: throw to monkey "), u64, tag("\n")))(i)?;
    let (i, (_, _, false_target, _)) =
        tuple((space1, tag("If false: throw to monkey "), u64, tag("\n")))(i)?;
    let m = Monkey {
        items,
        inspections: 0,
        operation,
        divisor,
        true_target: true_target as usize,
        false_target: false_target as usize,
    };
    Ok((i, m))
}
enum Packet {
    List,
    Integer,
}
pub fn parse_packets(i: &str) -> IResult<&str, Vec<Monkey>> {
    separated_list1(tag("\n"), parse_packet)(i)
}

fn parse_packet(i: &str) -> IResult<&str, &str> {}

pub fn day_thirteen() {
    let lines = include_str!("../input/day13-test.txt");
    //let mut monkeys = all_consuming(parse_monkeys)(lines).unwrap().1;
}
