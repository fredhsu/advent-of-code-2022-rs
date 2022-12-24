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

type Point = (i32, i32);
/*
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}
*/

fn get_coverage(sensor: Point, beacon: Point) -> Vec<Point> {
    println!("Coverage for sensor: {sensor:?}, beacon: {beacon:?}");
    let mut points = HashSet::new();
    let mdist = (sensor.0.abs_diff(beacon.0) + sensor.1.abs_diff(beacon.1)) as i32;
    //println!("Manhattan Distance: {mdist}");
    for i in sensor.0 - mdist..=sensor.0 + mdist {
        let y_dist = mdist - (i.abs_diff(sensor.0) as i32);
        //println!("{i} : dist {y_dist}");
        for j in 0..=y_dist {
            let x = i;
            let y = sensor.1 + j;
            let y2 = sensor.1 - j;
            points.insert((x, y));
            points.insert((x, y2));
        }
    }
    //println!("Points: {points:?}");
    points.into_iter().collect()
}
fn parse_line(i: &str) -> IResult<&str, (Point, Point)> {
    let (rest, (_, sx, _, sy, _, bx, _, by)) = tuple((
        tag("Sensor at x="),
        map_res(take_until(","), str::parse),
        tag(", y="),
        map_res(take_until(":"), str::parse),
        tag(": closest beacon is at x="),
        map_res(take_until(","), str::parse),
        tag(", y="),
        map_res(digit1, str::parse),
    ))(i)?;
    Ok((rest, ((sx, sy), (bx, by))))
}

fn print_coverage(map: Vec<Point>) {
    let top = map.iter().min_by_key(|&x| x.1).unwrap().1;
    let bottom = map.iter().max_by_key(|&x| x.1).unwrap().1;
    let left = map.iter().min_by_key(|&x| x.0).unwrap().0;
    let right = map.iter().max_by_key(|&x| x.0).unwrap().0;
    for i in top..=bottom {
        print!("{i} ");
        for j in left..=right {
            if map.contains(&(j, i)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}
pub fn day_fifteen() {
    let lines = include_str!("../input/day15-test.txt");
    //let mut sensors = Vec::new();
    //let mut beacons = Vec::new();
    let (_, l) = all_consuming(terminated(
        separated_list1(tag("\n"), parse_line),
        tag("\n"),
    ))(lines)
    .unwrap();
    for (sensor, beacon) in l {
        let coverage = get_coverage(sensor, beacon);
        print_coverage(coverage);
    }

    //println!("Sensors: {sensors:?}, Beacons: {beacons:?}");
}
