use std::collections::HashSet;

type Point = (i32, i32);
fn is_adjacent(head: &Point, tail: &Point) -> bool {
    //println!("comparing {:?} to {:?}", head, tail);
    !((head.0 - tail.0).abs() >= 2 || (head.1 - tail.1).abs() >= 2)
}

fn move_left(dist: u32, head: &mut Point, tail: &mut Point) -> Vec<Point> {
    let mut moves = Vec::new();
    for _ in 0..dist {
        head.1 -= 1;
        if !is_adjacent(head, tail) {
            tail.1 -= 1;
            tail.0 = tail.0 + (head.0 - tail.0);
            moves.push((tail.0, tail.1));
        }
    }
    moves
}
fn move_up(dist: u32, head: &mut Point, tail: &mut Point) -> Vec<Point> {
    let mut moves = Vec::new();
    for _ in 0..dist {
        head.0 += 1;
        if !is_adjacent(head, tail) {
            tail.0 += 1;
            tail.1 = tail.1 + (head.1 - tail.1);
            moves.push((tail.0, tail.1));
        }
    }
    moves
}
fn move_down(dist: u32, head: &mut Point, tail: &mut Point) -> Vec<Point> {
    let mut moves = Vec::new();
    for _ in 0..dist {
        head.0 -= 1;
        if !is_adjacent(head, tail) {
            tail.0 -= 1;
            tail.1 = tail.1 + (head.1 - tail.1);
            moves.push((tail.0, tail.1));
        }
    }
    moves
}
fn move_right(dist: u32, head: &mut Point, tail: &mut Point) -> Vec<Point> {
    let mut moves = Vec::new();
    for _ in 0..dist {
        head.1 += 1;
        if let Some(new_tail) = follow_right(head, tail) {
            *tail = new_tail;
            moves.push(new_tail);
        }
    }
    moves
}

// For part 2
fn move_knots_right(dist: u32, knots: &mut Vec<Point>) -> Vec<Point> {
    let mut moves = Vec::new();
    for _ in 0..dist {
        knots[0].1 += 1;
        if let Some(x) = follow_right(&knots[0], &knots[1]) {
            shift_knots(x, knots);
            /*
            for i in 2..knots.len() {
                if !is_adjacent(&knots[i], &knots[i-1]) {
                knots[i] = knots[i-1];
                }
            }
            knots[1] = x;
            */
            moves.push(knots[knots.len() - 1]);
        }
    }
    moves
}
fn move_knots_left(dist: u32, knots: &mut Vec<Point>) -> Vec<Point> {
    let mut moves = Vec::new();
    for _ in 0..dist {
        knots[0].1 -= 1;
        if let Some(x) = follow_left(&knots[0], &knots[1]) {
            shift_knots(x, knots);
            /*
            for i in 2..knots.len() {
                if !is_adjacent(&knots[i], &knots[i-1]) {
                knots[i] = knots[i-1];
                }
            }
            knots[1] = x;
            */
            moves.push(knots[knots.len() - 1]);
        }
    }
    moves
}
fn move_knots_up(dist: u32, knots: &mut Vec<Point>) -> Vec<Point> {
    let mut moves = Vec::new();
    for _ in 0..dist {
        knots[0].0 += 1;
        if let Some(x) = follow_up(&knots[0], &knots[1]) {
            shift_knots(x, knots);
            /*
            for i in 2..knots.len() {
                if !is_adjacent(&knots[i], &knots[i-1]) {
                knots[i] = knots[i-1];
                }
            }
            knots[1] = x;
            */
            moves.push(knots[knots.len() - 1]);
        }
    }
    moves
}
fn move_knots_down(dist: u32, knots: &mut Vec<Point>) -> Vec<Point> {
    let mut moves = Vec::new();
    for _ in 0..dist {
        knots[0].0 -= 1;
        if let Some(x) = follow_down(&knots[0], &knots[1]) {
            shift_knots(x, knots);
            /*
            let mut prev = knots[1];
            knots[1] = x;
            for i in 2..knots.len() {
                if !is_adjacent(&prev, &knots[i]) {
                    let temp = knots[i];
                    knots[i] = prev;
                    prev = temp;
                }
            }
            */
            moves.push(knots[knots.len() - 1]);
        }
    }
    moves
}

fn shift_knots(x: Point, knots: &mut Vec<Point>) {
    let mut prev = knots[1];
    knots[1] = x;
    for i in 2..knots.len() {
        if !is_adjacent(&knots[i], &knots[i - 1]) {
            let temp = knots[i];
            knots[i] = prev;
            prev = temp;
        }
    }
    println!("Knots are {:?}", knots);
}
fn follow_right(lead: &Point, follow: &Point) -> Option<Point> {
    if !is_adjacent(lead, follow) {
        let col = follow.1 + 1;
        let row = match follow.0.cmp(&lead.0) {
            std::cmp::Ordering::Less => follow.0 + 1,
            std::cmp::Ordering::Equal => follow.0,
            std::cmp::Ordering::Greater => follow.0 - 1,
        };
        return Some((row, col));
    }
    None
}
fn follow_left(lead: &Point, follow: &Point) -> Option<Point> {
    if !is_adjacent(lead, follow) {
        let col = follow.1 - 1;
        let row = match follow.0.cmp(&lead.0) {
            std::cmp::Ordering::Less => follow.0 + 1,
            std::cmp::Ordering::Equal => follow.0,
            std::cmp::Ordering::Greater => follow.0 - 1,
        };
        return Some((row, col));
    }
    None
}
fn follow_up(lead: &Point, follow: &Point) -> Option<Point> {
    if !is_adjacent(lead, follow) {
        let col = match follow.1.cmp(&lead.1) {
            std::cmp::Ordering::Less => follow.1 + 1,
            std::cmp::Ordering::Equal => follow.1,
            std::cmp::Ordering::Greater => follow.1 - 1,
        };
        let row = follow.0 + 1;
        return Some((row, col));
    }
    None
}
fn follow_down(lead: &Point, follow: &Point) -> Option<Point> {
    if !is_adjacent(lead, follow) {
        let col = match follow.1.cmp(&lead.1) {
            std::cmp::Ordering::Less => follow.1 + 1,
            std::cmp::Ordering::Equal => follow.1,
            std::cmp::Ordering::Greater => follow.1 - 1,
        };
        let row = follow.0 - 1;
        return Some((row, col));
    }
    None
}

pub fn day_nine() {
    let lines = include_str!("../input/day9-large.txt").lines();
    //let mut head = (0, 0);
    //let mut tail = (0, 0);
    let mut knots = vec![(0, 0); 10];
    let mut visited: HashSet<Point> = HashSet::new();
    visited.insert((0, 0));
    let mut tail_moves: Vec<Point> = Vec::new();
    for line in lines {
        let (dir, dist) = line.split_once(' ').unwrap();
        let dist: u32 = dist.parse().unwrap();
        match dir {
            "R" => {
                //tail_moves.append(&mut move_right(dist, &mut head, &mut tail));
                tail_moves.append(&mut move_knots_right(dist, &mut knots));
            }
            "L" => {
                //tail_moves.append(&mut move_left(dist, &mut head, &mut tail));
                tail_moves.append(&mut move_knots_left(dist, &mut knots));
            }
            "U" => {
                //tail_moves.append(&mut move_up(dist, &mut head, &mut tail));
                tail_moves.append(&mut move_knots_up(dist, &mut knots));
            }
            "D" => {
                //tail_moves.append(&mut move_down(dist, &mut head, &mut tail));
                tail_moves.append(&mut move_knots_down(dist, &mut knots));
            }
            _ => println!("Unknown"),
        }
        println!(
            "Head is at: {:?}, End is at: {:?}",
            knots[0],
            knots[knots.len() - 1]
        );
    }
    tail_moves.iter().for_each(|t| {
        visited.insert(*t);
    });
    //println!("Visited: {:?}", visited);
    println!("Visited total: {}", visited.len());
}
