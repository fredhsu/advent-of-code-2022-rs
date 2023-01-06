use core::fmt;
use std::collections::hash_map::Entry;
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
    flow: u64,
    tunnels: Vec<String>,
}
impl Valve {
    pub fn feasible(&self, delta: u64) -> Option<(u64, String)> {
        if !self.open && self.flow > 0 {
            return Some((self.flow * delta, self.name.clone()));
        }
        None
    }
    pub fn must_parse(i: &str) -> Self {
        all_consuming(Self::parse)(i)
            .finish()
            .expect("failed to parse valve")
            .1
    }
    fn parse(i: &str) -> IResult<&str, Valve> {
        //let result: IResult<&str, (&str, u64, Vec<&str>)> = tuple((
        map(
            tuple::<&str, _, _, _>((
                preceded(tag("Valve "), take(2_usize)),
                preceded(tag(" has flow rate="), u64),
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

#[derive(Debug)]
pub struct Network {
    valves: HashMap<String, Valve>,
}
pub type Path = Vec<(String, String)>;
impl Network {
    pub fn connections(&self, start: String) -> HashMap<String, Path> {
        // current will track the set of valves being considered with their current path
        let mut current: HashMap<String, Path> = Default::default();
        // start with the starting one and an empty path
        current.insert(start, vec![]);

        // connections is built on top of the current map that only has the start
        let mut connections = current.clone();

        //looping until we don't have anything else to start from
        while !current.is_empty() {
            //
            let mut next: HashMap<String, Path> = Default::default();
            // go through each of the current valves
            for (name, path) in current {
                // pull the connected links from the valves entry
                for link in self.valves[&name].tunnels.iter().clone() {
                    // check connections to see if there is a vacant spot for this valve
                    // in other words, if this entry doesn't already exist, create a spot for it
                    if let Entry::Vacant(e) = connections.entry(link.to_string()) {
                        let mut conn_path = path.clone();
                        conn_path.push((name.clone(), link.to_string()));
                        //                        let conn_path: Path = path
                        //                            .iter()
                        //                            .clone()
                        //                            .chain(std::iter::once((name, link)))
                        //                            .collect();
                        //e.insert(conn_path.clone());
                        e.insert(conn_path.clone());
                        next.insert(link.to_string(), conn_path);
                    }
                }
            }
            current = next;
        }
        connections
    }
}
#[derive(Debug, Clone)]
struct Valves {
    current: Valve,
    valves: HashMap<String, Valve>,
    flow: u64,
    time: u64,
    //max_time: u64,
}
impl fmt::Display for Valves {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "== Mintue {} ==", 30 - self.time)?;
        write!(f, "Valves ")?;
        for v in &self.valves {
            if v.1.open {
                write!(f, "{}, ", v.0)?;
            }
        }
        write!(f, "are open, releasing {} pressure\n", self.flow)
    }
}
impl Valves {
    fn valve_flow(&self, valve: &str) -> u64 {
        let v = self.valves.get(valve).unwrap();
        v.flow
    }
    fn open(&mut self) {
        let mut valve = self.current.clone();
        valve.open = true;
        self.time -= 1;
        self.flow += valve.flow * self.time;
        println!("{self}");
        println!("You open valve {}.", &valve.name);
        self.valves.remove(&valve.name);
    }
    fn distances(&self, start: &Valve) -> HashMap<String, u64> {
        // bfs from current to all
        // we could go ahead and calculate the flow and distance
        let mut q = VecDeque::new();
        let mut visited = HashMap::new();
        visited.insert(start.name.to_owned(), 0);
        q.push_back(start.clone());
        while let Some(t) = q.pop_front() {
            let distance = *visited.get(&t.name).unwrap();
            for c in &t.tunnels {
                // println!("Checking tunnel {c}");
                // do a check here if there was the candidate found in closed
                // This might not work if we need to traverse a closed valve
                if !visited.contains_key(c) {
                    //if visited.iter().find(|(_, _, name)| name == c).is_none() {
                    // can this just be using the hash string?
                    // if let Some(v) = self.valves.iter().find(|&x| x.0 == c) {
                    if let Some(v) = self.valves.get(c) {
                        q.push_back(v.clone());
                        let valve_dist = distance + 1;
                        visited.insert(c.to_owned(), valve_dist);
                    }
                }
            }
        }
        visited
    }
    fn move_to(&mut self, dest: &str, distance: u64) {
        self.time -= distance;
        let new_current = self.valves.get_mut(dest).unwrap();
        new_current.open = true;
        //new_current.flow = 0;
        println!("moving from {:?} move to {dest}", self.current);
        self.current = new_current.clone();
        //self.valves.remove(&self.current.name);
    }
}
#[derive(Debug, Clone)]
struct Move {
    reward: u64,
    target: String,
    path: Path,
}

impl Move {
    fn cost(&self) -> u64 {
        let travel_turns = self.path.len() as u64;
        let open_turns = 1_u64;
        travel_turns + open_turns
    }
}

#[derive(Debug, Clone)]
struct State<'a> {
    net: &'a Network,
    position: String,
    max_turns: u64,
    turn: u64,
    pressure: u64,
    open_valves: HashSet<String>,
}

impl State<'_> {
    fn apply(&self, mv: &Move) -> Self {
        let mut next = self.clone();
        next.position = mv.target.clone();
        next.turn += mv.cost();
        next.pressure += mv.reward;
        next.open_valves.insert(mv.target.clone());
        next
    }

    fn turns_left(&self) -> u64 {
        self.max_turns - self.turn
    }
    //fn moves(&self) -> impl Iterator<Item = Move> + '_ {
    fn moves(&self) -> Vec<Move> {
        self.net
            .connections(self.position.clone())
            .into_iter()
            .filter_map(|(name, path)| {
                if self.open_valves.contains(&name) {
                    return None;
                }
                let flow = self.net.valves[&name].flow;
                if flow == 0 {
                    return None;
                }

                let travel_turns = path.len() as u64;
                let open_turns = 1_u64;
                let turns_spent_open = self.turns_left().checked_sub(travel_turns + open_turns)?;
                let reward = flow * turns_spent_open;
                Some(Move {
                    reward,
                    target: name,
                    path,
                })
            })
            .collect()
    }

    // DFS for each possible move returning back the best set of moves from this point
    fn find_best_moves(&self) -> (Self, Vec<Move>) {
        let mut best_moves = vec![];
        let mut best_state = self.clone();

        for mv in self.moves() {
            // apply each possible move and create a new state
            let next = self.apply(&mv);
            // Get a new state and set of moves by recursively calling this function to find best
            // next moves
            let (next, mut next_moves) = next.find_best_moves();
            next_moves.push(mv);
            if next.pressure > best_state.pressure {
                best_moves = next_moves;
                best_state = next;
            }
        }
        (best_state, best_moves)
    }
}

pub fn day_sixteen() {
    let lines = include_str!("../input/day16-input.txt").lines();
    let mut input_valves = HashMap::new();
    for line in lines {
        let valve = Valve::must_parse(line);
        input_valves.insert(valve.name.clone(), valve.clone());
    }
    let net = Network {
        valves: input_valves,
    };
    let state = State {
        net: &net,
        position: "AA".to_string(),
        max_turns: 30,
        turn: 0,
        pressure: 0,
        open_valves: Default::default(),
    };

    //println!("Moves: {:?}", state.moves());
    let (state, moves) = state.find_best_moves();
    println!("moves = {:?}, final pressure = {}", moves, state.pressure);

    /*
    let current = input_valves.get("AA").unwrap().clone();
    let mut valves = Valves {
        current,
        valves: input_valves.clone(),
        flow: 0,
        time: 0,
        max_time: 30,
    };
    */
    // Create a matrix of flow rates for every valve to every other valve

    // take the current valve
    // figure out every feasible path that would start from this valve
    // create a path to each positive flow target
    // continue path from that target to every other target without repeating
    //

    /*
    let distances = valves.distances(&valves.current);
    let c = valves.clone();
    let mut closed: Vec<_> = c
        .valves
        .iter()
        .filter(|&(_, v)| !v.open && v.flow > 0)
        .collect();
    for dest in closed {
        let mut path: Vec<(u64, String)> = vec![(0, "AA".to_string())];
        path.push((1, dest.0.to_string()));
        valves.move_to(dest.0, 1);
        println!("Path {path:?}");
    }
    */
    // Do this again for each closed valve

    /*
    println!(
        "Feasible: JJ {:?}",
        valves.valves.get("JJ").unwrap().feasible(1)
    );
    let flows: Vec<_> = distances
        .iter()
        .filter_map(|(name, dist)| {
            valves
                .valves
                .get(name)
                .unwrap()
                .feasible(valves.time - dist - 1)
        })
        .collect();
    println!("Flows: {flows:?}");
    let net = Network {
        valves: input_valves.clone(),
    };

    println!("From AA:");
    for (name, path) in net.connections("AA".to_string()) {
        println!("We can get to {name} using path {path:?}");
    }
    */
    /*
    loop {
        let distances = valves.distances(&valves.current);
        println!(
            "Distances from current {:?} {:?}",
            valves.current, distances
        );
        let flows: Vec<(u64, u64, String)> = distances
            .iter()
            .filter(|(dist, flow, name)| !valves.valves.get(name).unwrap().open && *flow > 0)
            .map(|(dist, flow, name)| ((valves.time - dist - 1) * flow, *dist, name.to_owned()))
            .collect();
        println!("Flow choices are : {:?}", flows);
        if flows.len() == 0 {
            break;
        }
        let best = flows.iter().max_by(|&f1, &f2| f1.0.cmp(&f2.0)).unwrap();
        println!("Best valve to move to is: {:?}", best);
        println!("Moving to {:?}", best);
        valves.move_to(&best.2, best.1);
        println!("Remaining valves: {:?}", flows.len());
    }
    // distances could be filtered out for ones that have zero

    // calculate best flow from current
    */
}
