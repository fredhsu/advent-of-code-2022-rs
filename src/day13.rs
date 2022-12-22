use nom::branch::alt;
use nom::bytes::complete::*;
use nom::character::complete::*;
use nom::combinator::*;
use nom::multi::{separated_list0, separated_list1};
use nom::sequence::*;
use nom::IResult;
use serde::Deserialize;
use std::cmp::Ordering;

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(untagged)]
pub enum Packet {
    List(Vec<Packet>),
    Integer(u32),
}
impl std::cmp::PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Packet::Integer(a), Packet::Integer(b)) => a.partial_cmp(b),
            (Packet::Integer(x), Packet::List(y)) => {
                Packet::List(vec![Packet::Integer(*x)]).partial_cmp(&Packet::List(y.clone()))
            }
            (Packet::List(x), Packet::Integer(y)) => {
                Packet::List(x.clone()).partial_cmp(&Packet::List(vec![Packet::Integer(*y)]))
            }
            (Packet::List(x), Packet::List(y)) => {
                //println!("Comparing lists: {x:?} {y:?}");
                Some(
                    x.iter()
                        .zip(y)
                        .map(|(a, b)| a.cmp(b))
                        .find(|&ord| ord != Ordering::Equal)
                        .unwrap_or_else(|| x.len().cmp(&y.len())),
                )
            }
        }
    }
}
impl std::cmp::Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}
pub fn parse_packets(i: &str) -> IResult<&str, Packet> {
    //println!("parse_packets: {i:?}");
    let (rest, parsed) = delimited(
        tag("["),
        alt((
            parse_empty,
            parse_packet_list,
            parse_packets,
            parse_packet_int,
        )),
        tag("]"),
    )(i)?;
    Ok((rest, parsed))
}

fn parse_empty(i: &str) -> IResult<&str, Packet> {
    let (rest, parsed) = tag("[]")(i)?;
    Ok((rest, Packet::List(vec![])))
}
fn parse_packet_int(i: &str) -> IResult<&str, Packet> {
    //println!("parse_packet_int: {i:?}");
    let (rest, num) = map_res(digit1, str::parse)(i)?;
    Ok((rest, Packet::Integer(num)))
}

fn parse_packet_list(i: &str) -> IResult<&str, Packet> {
    // separated list or a single packet or delimited
    //println!("parse_packet_list: {i:?}");
    let (rest, packets) = separated_list1(tag(","), alt((parse_packets, parse_packet_int)))(i)?;
    Ok((rest, Packet::List(packets)))
}

fn parse(i: &str) -> IResult<&str, Packet> {
    //let parser = alt((parse_packets, parse_packet_list))(i)?;
    let parser = alt((parse_empty, parse_packets))(i)?;
    Ok(parser)
}
pub fn day_thirteen() {
    //let mut lines = include_str!("../input/day13-test.txt").lines();
    let mut sum = 0;
    let mut sorted_packets: Vec<Packet> = vec![];
    for (i, groups) in include_str!("../input/day13-input.txt")
        .split("\n\n")
        .enumerate()
    {
        let i = i + 1;
        let lines = groups.lines();
        let mut packets = lines.map(|line| serde_json::from_str::<Packet>(line).unwrap());
        //let parse1 = parse(list1);
        //let parse2 = parse(list2);
        //
        let l = packets.next().unwrap();
        let r = packets.next().unwrap();
        if l < r {
            //println!("Compare is true for {i}");
            sum += i;
        }
        sorted_packets.push(l);
        sorted_packets.push(r);
        //if compare((&parse1.unwrap().1, &parse2.unwrap().1)) {
        //println!("Compare is true for {i}");
        //sum += i;
        //}
        dbg!(i);
        dbg!(sum);
    }
    let divider2 = Packet::List(vec![Packet::List(vec![Packet::Integer(2)])]);
    let divider6 = Packet::List(vec![Packet::List(vec![Packet::Integer(6)])]);
    sorted_packets.push(divider2.clone());
    sorted_packets.push(divider6.clone());
    sorted_packets.sort();
    println!("{sorted_packets:?}");
    let index2 = sorted_packets.iter().position(|x| *x == divider2).unwrap() + 1;
    let index6 = sorted_packets.iter().position(|x| *x == divider6).unwrap() + 1;
    println!(
        "Index of 2: {index2}, index of 6: {index6}, product is {}",
        index2 * index6
    );
}
