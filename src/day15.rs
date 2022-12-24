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

fn get_coverage(sensor: Point, beacon: Point, row: i32) -> HashSet<Point> {
    //println!("Coverage for sensor: {sensor:?}, beacon: {beacon:?}");
    let mut points = HashSet::new();
    let mdist = (sensor.0.abs_diff(beacon.0) + sensor.1.abs_diff(beacon.1)) as i32;
    //println!("Manhattan Distance: {mdist}");
    if sensor.1 + mdist > row && sensor.1 - mdist < row {
        // figure out what x positions need to be added based on the y distance from the sensor
        let y_dist = row.abs_diff(sensor.1) as i32;
        let x_dist = mdist - y_dist;
        for i in sensor.0 - x_dist..=sensor.0 + x_dist {
            points.insert((i, row));
        }
    } else {
        println!(
            "Max: {}\nRow: {row}\nMin: {}\n\n",
            sensor.1 + mdist,
            sensor.1 - mdist
        );
    }
    /*
        for i in sensor.0 - mdist..=sensor.0 + mdist {
            let y_dist = mdist - (i.abs_diff(sensor.0) as i32);
            //println!("{i} : dist {y_dist}");
            for j in 0..=y_dist {
                points.insert(i, y);
            }
            /*for j in 0..=y_dist {
                let x = i;
                let y = sensor.1 + j;
                let y2 = sensor.1 - j;
                if y == row {
                    points.insert((x, y));
                } else if y2 == row {
                    points.insert((x, y2));
                }
                //println!("Inserting for ({x}, {y}) and ({x}, {y2})");
            }
            */
        }
    */
    //println!("Points: {points:?}");
    points
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
        map_res(take_until("\n"), str::parse),
    ))(i)?;
    Ok((rest, ((sx, sy), (bx, by))))
}

fn print_coverage(map: &HashSet<Point>) {
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
    let lines = include_str!("../input/day15-input.txt");
    //let mut sensors = Vec::new();
    let mut beacons = HashSet::new();
    let (_, l) = all_consuming(terminated(
        separated_list1(tag("\n"), parse_line),
        tag("\n"),
    ))(lines)
    .unwrap();
    let mut coverage = HashSet::new();
    let row = 2_000_000;
    for (sensor, beacon) in l {
        //println!("Getting coverage for {:?} b: {:?}", sensor, beacon);
        coverage.extend(get_coverage(sensor, beacon, row));
        beacons.insert(beacon);
    }
    println!("{}", coverage.len());
    //print_coverage(&coverage);
    let positions: Vec<&Point> = coverage
        .iter()
        .filter(|&p| p.1 == row && !beacons.contains(p))
        .collect();
    println!("count: {}", positions.len());
    //println!("Sensors: {sensors:?}, Beacons: {beacons:?}");
}
