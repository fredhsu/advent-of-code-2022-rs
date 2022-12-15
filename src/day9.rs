use std::collections::HashSet;

type Point = (i32, i32);
fn is_adjacent(head: &Point, tail: &Point) -> bool {
    //println!("comparing {:?} to {:?}", head, tail);
    !((head.0 - tail.0).abs() >= 2 || (head.1 - tail.1).abs() >= 2)
}

fn print_knots(knots: &[Point]) {
    println!("{:?}", knots);
    let minrow = knots.iter().map(|x| x.0).min().unwrap();
    let maxrow = knots.iter().map(|x| x.0).max().unwrap();
    let mincol = knots.iter().map(|x| x.1).min().unwrap();
    let maxcol = knots.iter().map(|x| x.1).max().unwrap();
    for r in (minrow..=maxrow).rev() {
        print!("Row {r}:");
        for c in mincol..=maxcol {
            if knots.contains(&(r, c)) {
                print!(
                    "{}",
                    knots.iter().position(|&(x, y)| x == r && y == c).unwrap()
                );
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
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
    println!("Moving right");
    let mut moves = Vec::new();
    for _ in 0..dist {
        // move the head right
        knots[0].1 += 1;

        // find the next spot for all the followers
        for i in 1..knots.len() {
            if let Some(x) = follow_right(&knots[i - 1], &knots[i]) {
                // if the next knot should move then reassign current knot the new location
                knots[i] = x;
            }
        }
        // after moving everything add the end the the list
        moves.push(knots[knots.len() - 1]);
        print_knots(knots);
    }
    moves
}

fn move_knots_left(dist: u32, knots: &mut Vec<Point>) -> Vec<Point> {
    let mut moves = Vec::new();
    for _ in 0..dist {
        knots[0].1 -= 1;
        for i in 1..knots.len() {
            if let Some(x) = follow_left(&knots[i - 1], &knots[i]) {
                // if the next knot should move then reassign current knot the new location
                knots[i] = x;
            }
        }
        // after moving everything add the end the the list
        moves.push(knots[knots.len() - 1]);
        print_knots(knots);
    }
    moves
}
fn move_knots_up(dist: u32, knots: &mut Vec<Point>) -> Vec<Point> {
    let mut moves = Vec::new();
    for _ in 0..dist {
        knots[0].0 += 1;
        for i in 1..knots.len() {
            if let Some(x) = follow_up(&knots[i - 1], &knots[i]) {
                // if the next knot should move then reassign current knot the new location
                knots[i] = x;
            }
        }
        // after moving everything add the end the the list
        moves.push(knots[knots.len() - 1]);
        print_knots(knots);
    }
    moves
}
fn move_knots_down(dist: u32, knots: &mut Vec<Point>) -> Vec<Point> {
    let mut moves = Vec::new();
    for _ in 0..dist {
        knots[0].0 -= 1;
        for i in 1..knots.len() {
            if let Some(x) = follow_down(&knots[i - 1], &knots[i]) {
                // if the next knot should move then reassign current knot the new location
                knots[i] = x;
            }
        }
        // after moving everything add the end the the list
        moves.push(knots[knots.len() - 1]);
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
    // only move if its not adjacent
    if !is_adjacent(lead, follow) {
        // moving right is adding one to the column value
        let col = follow.1 + 1;
        // Determine if the row is different
        let row = match follow.0.cmp(&lead.0) {
            // the lead is above the follow, add one to follow
            std::cmp::Ordering::Less => follow.0 + 1,
            // Row is the same, no vertical movement
            std::cmp::Ordering::Equal => follow.0,
            // the lead is below the follow, subtrat one from follow
            std::cmp::Ordering::Greater => follow.0 - 1,
        };
        return Some((row, col));
    }
    None
}
fn follow(lead: &Point, follow: &Point) {
    if !is_adjacent(lead, follow) {
        // if directly left or right
        if lead.0 == follow.0 {
            let diff = lead.1 - follow.1; // if lead is 2 and follow is 0 = 2, lead -1 follow 1
                                          // diff is -2
            
        }
    }
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
        let row = follow.0 + 1;
        let col = match follow.1.cmp(&lead.1) {
            std::cmp::Ordering::Less => follow.1 + 1,
            std::cmp::Ordering::Equal => follow.1,
            std::cmp::Ordering::Greater => follow.1 - 1,
        };
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
    let lines = include_str!("../input/day9-small.txt").lines();
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
