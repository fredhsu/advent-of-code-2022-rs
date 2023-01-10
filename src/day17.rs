use nom::branch::alt;
use nom::bytes::complete::*;
use nom::character::complete::{digit1, newline};
use nom::combinator::*;
use nom::multi::{many1, separated_list1};
use nom::sequence::*;
use nom::{Finish, IResult};
use std::cmp;
use std::collections::HashMap;
use std::collections::HashSet;
use std::ops::RangeInclusive;

// TODO: either create a name for the robot type or put into an enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Robot {
    ore_cost: u64,
    clay_cost: u64,
    obsidian_cost: u64,
}
impl Robot {
    pub fn parse(i: &str) -> IResult<&str, Robot> {
        alt((
            Self::parse_ore,
            Self::parse_clay,
            Self::parse_obsidian,
            Self::parse_geode,
        ))(i)
    }
    fn parse_ore(i: &str) -> IResult<&str, Robot> {
        map(
            delimited(
                tag("Each ore robot costs "),
                nom::character::complete::u64,
                tag(" ore"),
            ),
            |ore_cost: u64| Self {
                ore_cost,
                clay_cost: 0,
                obsidian_cost: 0,
            },
        )(i)
    }
    fn parse_clay(i: &str) -> IResult<&str, Self> {
        map(
            delimited(
                tag("Each clay robot costs "),
                nom::character::complete::u64,
                tag(" ore"),
            ),
            |ore_cost: u64| Self {
                ore_cost,
                clay_cost: 0,
                obsidian_cost: 0,
            },
        )(i)
    }
    fn parse_obsidian(i: &str) -> IResult<&str, Self> {
        map(
            delimited(
                tag("Each obsidian robot costs "),
                tuple((
                    nom::character::complete::u64,
                    tag(" ore and "),
                    nom::character::complete::u64,
                )),
                tag(" clay"),
            ),
            |(ore_cost, _, clay_cost)| Self {
                ore_cost,
                clay_cost,
                obsidian_cost: 0,
            },
        )(i)
    }
    fn parse_geode(i: &str) -> IResult<&str, Self> {
        map(
            delimited(
                tag("Each geode robot costs "),
                tuple((
                    nom::character::complete::u64,
                    tag(" ore and "),
                    nom::character::complete::u64,
                )),
                tag(" obsidian."),
            ),
            |(ore_cost, _, obsidian_cost)| Self {
                ore_cost,
                clay_cost: 0,
                obsidian_cost,
            },
        )(i)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Blueprint {
    robots: Vec<Robot>,
}
impl Blueprint {
    pub fn parse(i: &str) -> IResult<&str, Self> {
        map(
            tuple((
                delimited(tag("Blueprint "), digit1, tag(": ")),
                separated_list1(tag(". "), Robot::parse),
                newline,
            )),
            |(_, robots, _)| Self { robots },
        )(i)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Blueprints {
    blueprints: Vec<Blueprint>,
}
impl Blueprints {
    pub fn must_parse(i: &str) -> Self {
        all_consuming(Self::parse)(i)
            .finish()
            .expect("failed to parse input")
            .1
    }
    fn parse(i: &str) -> IResult<&str, Self> {
        map(many1(Blueprint::parse), |blueprints| Self { blueprints })(i)
    }
}

pub fn day_seventeen() {
    let input = include_str!("../input/day17-test.txt");
    let output = Blueprints::must_parse(input);
    println!("output parsing: {output:?}");
}
