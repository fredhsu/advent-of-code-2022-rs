use core::fmt::{Display, Formatter, Result};
use nom::branch::alt;
use nom::bytes::complete::*;
use nom::character::complete::*;
use nom::combinator::*;
use nom::multi::{separated_list0, separated_list1};
use nom::sequence::*;
use nom::IResult;
use std::cmp::Ordering;
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Line {
    points: Vec<Point>,
}

impl Line {
    pub fn get_points(&self) -> HashSet<Point> {
        let mut result = HashSet::new();
        let mut points = self.points.clone();
        let mut p1 = points.pop().unwrap();
        while let Some(p2) = points.pop() {
            //println!("p1: {p1:?}, p2: {p2:?}");
            let mut xrange = p1.x..=p2.x;
            if p1.x > p2.x {
                xrange = p2.x..=p1.x;
            }
            let mut yrange = p1.y..=p2.y;
            if p1.y > p2.y {
                yrange = p2.y..=p1.y;
            }
            for x in xrange {
                result.insert(Point { x, y: p1.y });
            }
            for y in yrange {
                result.insert(Point { x: p1.x, y });
            }
            p1 = p2;
        }
        result
    }
}

struct Cave {
    sand: HashSet<Point>,
    rocks: HashSet<Point>,
}

impl Cave {
    fn left(&self) -> i32 {
        let r = self.rocks.iter().min_by_key(|&r| r.x).unwrap();
        let s = self.sand.iter().min_by_key(|&r| r.x).unwrap();
        if r.x < s.x {
            r.x
        } else {
            s.x
        }
    }
    fn right(&self) -> i32 {
        let r = self.rocks.iter().max_by_key(|&r| r.x).unwrap();
        let s = self.sand.iter().max_by_key(|&r| r.x).unwrap();
        if r.x > s.x {
            r.x
        } else {
            s.x
        }
    }
    fn top(&self) -> i32 {
        0
    }
    fn bottom(&self) -> i32 {
        let r = self.rocks.iter().max_by_key(|&r| r.y).unwrap();
        r.y
    }
    fn floor(&self) -> i32 {
        self.bottom() + 2
    }
    fn is_air(&self, p: &Point) -> bool {
        !self.rocks.contains(p) && !self.sand.contains(p)
    }
    // drop_sand drops a piece of sand down and returns its resting place or None if it fell off
    fn drop_sand(&self) -> Option<Point> {
        //starting at source, simulate sand falling
        let mut grain = Point { x: 500, y: 0 };
        let mut falling = true;
        while falling {
            let below = Point {
                x: grain.x,
                y: grain.y + 1,
            };
            let left_down = Point {
                x: grain.x - 1,
                y: grain.y + 1,
            };
            let right_down = Point {
                x: grain.x + 1,
                y: grain.y + 1,
            };
            if self.sand.contains(&grain) {
                return None;
            };
            if self.is_air(&below) && self.floor() > below.y {
                //println!("moving down floor: {}, below: {}", self.floor(), below.y);
                grain.y += 1;
                // check if falling off
            } else if self.is_air(&left_down) && self.floor() > left_down.y {
                //println!("moving left");
                grain.x -= 1;
                grain.y += 1;
            //} else if self.is_air(&right) && self.is_air(&right_down) {
            } else if self.is_air(&right_down) && self.floor() > right_down.y {
                //println!("moving right");
                grain.x += 1;
                grain.y += 1;
            } else {
                println!("resting: {grain:?}");
                falling = false;
            }
        }
        Some(grain)
    }
}
impl Display for Cave {
    fn fmt(&self, f: &mut Formatter) -> Result {
        for j in self.top()..=self.bottom() {
            write!(f, "{j} ")?;
            for i in self.left()..=self.right() {
                let p = Point { x: i, y: j };
                //writeln!(f, "Printing point {p:?}")?;
                if self.sand.contains(&p) {
                    write!(f, "o")?;
                } else if self.rocks.contains(&p) {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        writeln!(f)
    }
}

fn parse_point_value(i: &str) -> IResult<&str, i32> {
    let (rest, num) = map_res(digit1, str::parse)(i)?;
    Ok((rest, num))
}

fn parse_point(i: &str) -> IResult<&str, Point> {
    let (rest, (x, y)) = separated_pair(parse_point_value, char(','), parse_point_value)(i)?;
    Ok((rest, Point { x, y }))
}

fn parse_line(i: &str) -> IResult<&str, Line> {
    let (rest, points) = separated_list1(tag(" -> "), parse_point)(i)?;
    Ok((rest, Line { points }))
}
pub fn day_fourteen() {
    let lines = include_str!("../input/day14-input.txt").lines();
    let source = Point { x: 500, y: 0 };
    let mut rocks = HashSet::new();
    for line in lines {
        let l = parse_line(line).unwrap().1;
        //println!("{l:?}");
        rocks.extend(l.get_points());
        //println!("rocks: {rocks:?} length: {}", rocks.len());
    }
    let mut cave = Cave {
        rocks: rocks.clone(),
        sand: HashSet::new(),
    };
    while let Some(grain) = cave.drop_sand() {
        //for _ in 0..22 {
        //let grain = cave.drop_sand();
        //println!("{grain:?}");
        cave.sand.insert(grain);
    }
    //println!("Sand: {:?}", cave.sand);
    //println!("Rocks: {:?}", cave.rocks);
    println!("{cave}");
    println!("{}", cave.sand.len());
}
