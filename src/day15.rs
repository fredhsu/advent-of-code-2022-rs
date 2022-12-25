use nom::bytes::complete::*;
use nom::combinator::*;
use nom::multi::separated_list1;
use nom::sequence::*;
use nom::IResult;
use std::cmp;
use std::collections::HashMap;
use std::collections::HashSet;

type Point = (i32, i32);

fn get_coverage(sensor: Point, beacon: Point) -> HashSet<Point> {
    let mut points = HashSet::new();
    let mut ranges = HashMap::new();
    let mdist = (sensor.0.abs_diff(beacon.0) + sensor.1.abs_diff(beacon.1)) as i32;
    let low = sensor.1 - mdist;
    let high = sensor.1 + mdist;
    let low = cmp::max(0, low);
    let high = cmp::min(4_000_000, high);
    println!("low: {low}, high: {high}");

    for row in low..=high {
        // figure out what x positions need to be added based on the y distance from the sensor
        let y_dist = row.abs_diff(sensor.1) as i32;
        let x_dist = mdist - y_dist;
        let start = sensor.0 - x_dist;
        let end = sensor.0 + x_dist;
        let xrange = (start, end);
        ranges.insert(row, xrange);

        // create a tuple that defines the range instead and make a vector of those then use that
        // to calculate overlap/gaps
        /*
        for i in sensor.0 - x_dist..=sensor.0 + x_dist {
            points.insert((i, row));
        }
        */
    }
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
        println!("Getting coverage for {:?} b: {:?}", sensor, beacon);
        coverage.extend(get_coverage(sensor, beacon));
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
