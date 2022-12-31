use core::fmt;
use std::collections::{HashMap, HashSet, VecDeque};

use nom::bytes::complete::*;
use nom::character::complete::*;
use nom::combinator::*;
use nom::multi::separated_list1;
use nom::sequence::*;
use nom::{branch::alt, Finish, IResult};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Valve {
    name: String,
    open: bool,
    flow: i64,
    tunnels: Vec<String>,
}
impl Valve {
    pub fn must_parse(i: &str) -> Self {
        all_consuming(Self::parse)(i)
            .finish()
            .expect("failed to parse valve")
            .1
    }
    fn parse(i: &str) -> IResult<&str, Valve> {
        //let result: IResult<&str, (&str, i64, Vec<&str>)> = tuple((
        map(
            tuple::<&str, _, _, _>((
                preceded(tag("Valve "), take(2_usize)),
                preceded(tag(" has flow rate="), i64),
                Self::parse_tunnels,
            )),
            |(name, flow, tunnels)| Self {
                name: name.to_owned(),
                open: false,
                flow,
                tunnels,
            },
        )(i)
    }
    fn parse_tunnels(i: &str) -> IResult<&str, Vec<String>> {
        map(
            preceded(
                alt((
                    tag("; tunnels lead to valves "),
                    tag("; tunnel leads to valve "),
                )),
                separated_list1(tag(", "), take(2_usize)),
            ),
            |vs: Vec<&str>| vs.iter().map(|&v| v.to_string()).collect(),
        )(i)
    }
}

#[derive(Debug, Clone)]
struct Valves {
    current: Valve,
    open_valves: HashSet<Valve>,
    closed_valves: HashMap<String, Valve>,
    flow: i64,
    time: i64,
}
impl fmt::Display for Valves {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "== Mintue {} ==", 30 - self.time);
        write!(f, "Valves ");
        for v in &self.open_valves {
            write!(f, "{}, ", v.name);
        }
        write!(f, "are open, releasing {} pressure\n", self.flow)
    }
}
impl Valves {
    fn open(&mut self) {
        let mut valve = self.current.clone();
        self.closed_valves.remove(&self.current.name);
        valve.open = true;
        self.time -= 1;
        self.flow += valve.flow * self.time;
        println!("{self}");
        println!("You open valve {}.", &valve.name);
        self.open_valves.insert(valve);
    }
    fn distances(&self, target: &str) -> Vec<(i64, String)> {
        println!("getting distance to {target} from {}", self.current.name);
        // bfs from current to all
        // only need to check closed valves
        let distances = Vec::new();
        let mut q = VecDeque::new();
        let visited = self.open_valves.clone();
        q.push_back(self.current.clone());
        //while let Some(t) = q.pop_front() {
        if let Some(t) = q.pop_front() {
            for c in &t.tunnels {
                println!("Checking tunnel {c}");
                // do a check here if there was the candidate found in closed
                let v = self.closed_valves.iter().find(|&x| x.0 == c).unwrap();
                q.push_back(v.1.clone());
            }
        }
        distances
    }
    fn move_to(&mut self, next: &str) {
        self.time -= 1;
        //self.current = next.to_string();
        println!("{self}");
        println!("You move to {next}");
    }
    fn get_candidates(&self) -> Vec<String> {
        // calculate flow * distance for each valve
        self.closed_valves.iter().map(|v| v.0.clone()).collect()
    }
}

pub fn day_sixteen() {
    let lines = include_str!("../input/day16-test.txt").lines();
    let mut closed_valves = HashMap::new();
    for line in lines {
        let valve = Valve::must_parse(line);
        closed_valves.insert(valve.name.clone(), valve.clone());
    }

    let current = closed_valves.get("AA").unwrap().clone();
    let mut valves = Valves {
        current,
        open_valves: HashSet::new(),
        closed_valves,
        flow: 0,
        time: 30,
    };
    println!("{valves:?}");
    println!("candidates {:?}", valves.get_candidates());
    valves.move_to("BB");
    let distance = valves.distances("BB");
    println!("Distance to BB {:?}", distance);
    println!("Distance to DD {:?}", valves.distances("DD"));
    println!("candidates {:?}", valves.get_candidates());
    valves.open();
    println!("candidates {:?}", valves.get_candidates());
}
