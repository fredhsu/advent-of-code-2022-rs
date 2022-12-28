use itertools::Itertools;
use nom::bytes::complete::*;
use nom::combinator::*;
use nom::multi::separated_list1;
use nom::sequence::*;
use nom::{Finish, IResult};
use std::cmp;
use std::collections::HashMap;
use std::collections::HashSet;
use std::ops::RangeInclusive;

type PPoint = (i32, i32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    x: i64,
    y: i64,
}
impl Point {
    pub fn parse(i: &str) -> IResult<&str, Point> {
        map(
            separated_pair(
                preceded(tag("x="), nom::character::complete::i64),
                tag(", "),
                preceded(tag("y="), nom::character::complete::i64),
            ),
            |(x, y)| Point { x, y },
        )(i)
    }
    pub fn manhattan_distance(self, other: Self) -> i64 {
        (self.x.abs_diff(other.x) + self.y.abs_diff(other.y)) as i64
    }
}

#[derive(Debug)]
pub struct Record {
    pub sensor: Point,
    pub beacon: Point,
}

impl Record {
    pub fn must_parse(i: &str) -> Self {
        all_consuming(Self::parse)(i)
            .finish()
            .expect("failed to parse input")
            .1
    }

    fn parse(i: &str) -> IResult<&str, Self> {
        // example line:
        // Sensor at x=2, y=18: closest beacon is at x=-2, y=15
        map(
            separated_pair(
                preceded(tag("Sensor at "), Point::parse),
                tag(": closest beacon is at "),
                Point::parse,
            ),
            |(sensor, beacon)| Record { sensor, beacon },
        )(i)
    }
}

struct Map {
    records: Vec<Record>,
}
impl Map {
    fn parse(input: &str) -> Self {
        let records = input.lines().map(Record::must_parse).collect();
        Self { records }
    }
    fn ranges_clamped(
        &self,
        y: i64,
        x_range: RangeInclusive<i64>,
    ) -> impl Iterator<Item = RangeInclusive<i64>> {
        self.ranges(y).filter_map(move |r| {
            let r = *r.start().max(x_range.start())..=*r.end().min(x_range.end());
            if r.start() > r.end() {
                None
            } else {
                Some(r)
            }
        })
    }
    fn beacon_position(
        &self,
        y_range: &RangeInclusive<i64>,
        x_range: &RangeInclusive<i64>,
    ) -> Option<Point> {
        y_range.clone().find_map(|y| {
            self.ranges_clamped(y, x_range.clone())
                .nth(1)
                .map(|r| Point {
                    x: r.start() - 1,
                    y,
                })
        })
    }

    fn num_impossible_positions(&self, y: i64) -> usize {
        let mut min_x = i64::MAX;
        let mut max_x = i64::MIN;
        let mut total = 0;
        for rec in &self.records {
            let radius = rec.sensor.manhattan_distance(rec.beacon);
            let y_dist = (y - rec.beacon.y).abs();
            if y_dist > radius {
                // coverage area doesn't touch line at `y`
                continue;
            }
            let middle = rec.sensor.x;
            let start = middle - radius;
            let end = middle + radius;
            min_x = min_x.min(start);
            max_x = max_x.max(end);
        }
        dbg!(min_x, max_x);
        for x in min_x..=max_x {
            let point = Point { x, y };
            if self.records.iter().any(|rec| rec.beacon == point) {
                // already have a beacon there, not an impossible position
            } else if self.records.iter().any(|rec| {
                let radius = rec.sensor.manhattan_distance(rec.beacon);
                rec.sensor.manhattan_distance(point) <= radius
            }) {
                // covered!
                total += 1
            }
        }
        total
    }
    fn ranges(&self, y: i64) -> impl Iterator<Item = RangeInclusive<i64>> {
        let mut ranges = vec![];
        for rec in &self.records {
            let radius = rec.sensor.manhattan_distance(rec.beacon);
            let y_dist = (y - rec.sensor.y).abs();
            if y_dist > radius {
                continue;
            }
            let d = radius - y_dist;
            let middle = rec.sensor.x;
            let start = middle - d;
            let end = middle + d;
            let range = start..=end;
            ranges.push(range);
        }

        ranges.sort_by_key(|r| *r.start());

        ranges.into_iter().coalesce(|a, b| {
            if b.start() - 1 <= *a.end() {
                if b.end() > a.end() {
                    Ok(*a.start()..=*b.end())
                } else {
                    Ok(a)
                }
            } else {
                Err((a, b))
            }
        })
    }
}
//const MAX: i32 = 20;
const MAX: i32 = 4_000_000;

fn manhattan_distance(sensor: PPoint, beacon: PPoint) -> i32 {
    (sensor.0.abs_diff(beacon.0) + sensor.1.abs_diff(beacon.1)) as i32
}

// find the edges of a diamond and return the points along the edges
fn get_edges(sensor: PPoint, beacon: PPoint) -> HashSet<PPoint> {
    let mut points = HashSet::new();
    let mdist = manhattan_distance(sensor, beacon);
    //println!("get_edges sensor: {sensor:?} mdist: {mdist:?}");
    let top = cmp::max(0, sensor.1 - mdist);
    let bottom = cmp::min(MAX, sensor.1 + mdist);
    // get top and bottom if it is within bounds
    if top > 0 {
        //println!("inserting top: {:?}", (sensor.0, top - 1));
        points.insert((sensor.0, top - 1));
    }
    if bottom < MAX {
        //println!("inserting bottom: {:?}", (sensor.0, bottom + 1));
        points.insert((sensor.0, bottom + 1));
    }
    // get left and right of each row
    for row in top..=bottom {
        let y_dist = row.abs_diff(sensor.1) as i32;
        //let y_dist = row - sensor.1;
        let x_dist = mdist - y_dist;
        //let start = cmp::max(1, sensor.0 - x_dist);
        //let end = cmp::min(MAX - 1, sensor.0 + x_dist);
        let start = sensor.0 - x_dist;
        let end = sensor.0 + x_dist;
        if row >= 0 && start > 0 {
            points.insert((start - 1, row));
        }
        if row >= 0 && end > 0 && end < MAX {
            points.insert((end + 1, row));
        }
        //println!("row: {row} edges: {points:?}");
    }
    points
}
fn get_ranges(sensor: PPoint, beacon: PPoint) -> HashMap<i32, PPoint> {
    let mut ranges = HashMap::new();
    let mdist = manhattan_distance(sensor, beacon);
    let low = sensor.1 - mdist;
    let high = sensor.1 + mdist;
    let low = cmp::max(0, low);
    let high = cmp::min(MAX, high);
    //println!("low: {low}, high: {high}");

    // create a tuple that defines the range instead and make a vector of those then use that
    // to calculate overlap/gaps
    for row in low..=high {
        // figure out what x positions need to be added based on the y distance from the sensor
        let y_dist = row.abs_diff(sensor.1) as i32;
        let x_dist = mdist - y_dist;
        let start = cmp::max(0, sensor.0 - x_dist);
        let end = cmp::min(MAX, sensor.0 + x_dist);
        let xrange = (start, end);
        ranges.insert(row, xrange);
    }
    ranges
}
fn get_coverage(sensor: PPoint, beacon: PPoint) -> HashSet<PPoint> {
    let points = HashSet::new();
    let mut ranges = HashMap::new();
    let mdist = (sensor.0.abs_diff(beacon.0) + sensor.1.abs_diff(beacon.1)) as i32;
    let low = sensor.1 - mdist;
    let high = sensor.1 + mdist;
    let low = cmp::max(0, low);
    let high = cmp::min(MAX, high);
    println!("low: {low}, high: {high}");

    for row in low..=high {
        // figure out what x positions need to be added based on the y distance from the sensor
        let y_dist = row.abs_diff(sensor.1) as i32;
        let x_dist = mdist - y_dist;
        let start = cmp::max(0, sensor.0 - x_dist);
        let end = cmp::min(20, sensor.0 + x_dist);
        let xrange = (start, end);
        ranges.insert(row, xrange);
    }
    points
}
fn parse_line(i: &str) -> IResult<&str, (PPoint, PPoint)> {
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

fn print_coverage(map: &HashSet<PPoint>) {
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
    //let mut beacons = HashSet::new();
    /*let (_, l) = all_consuming(terminated(
        separated_list1(tag("\n"), parse_line),
        tag("\n"),
    ))(lines)
    .unwrap();
    let mut coverage: HashMap<PPoint, i32> = HashMap::new();
    for (sensor, beacon) in l {
        //println!("Getting coverage for s: {sensor:?} b: {beacon:?}");
        //println!("Getting edges for s: {sensor:?} b: {beacon:?}");
        let edges = get_edges(sensor, beacon);
        for edge in edges {
            if let Some(c) = coverage.get_mut(&edge) {
                *c += 1;
            } else {
                coverage.insert(edge, 1);
            }
        }
    }
    */
    for (input, _y, range) in [(
        include_str!("../input/day15-input.txt"),
        2_000_000,
        0..=4_000_000,
    )] {
        let map = Map::parse(input);
        let bp = map.beacon_position(&range, &range).unwrap();
        dbg!(bp);
        println!("tuning freq: {}", bp.x * 4_000_000 + bp.y);
    }
}
