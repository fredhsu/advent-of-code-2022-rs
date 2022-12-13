use std::collections::HashSet;

type Point = (i32, i32);
fn is_adjacent(head: &Point, tail: &Point) -> bool {
    !((head.0 - tail.0).abs() >= 2 || (head.1 - tail.1).abs() >= 2)
    /*
    if (head.0 - tail.0).abs() >= 2 || (head.1 - tail.1).abs() >= 2 {
        return false;
    }
    true
    */
}
fn move_right(dist: u32, head: &mut Point, tail: &mut Point) -> Vec<Point>{
    let mut moves = Vec::new();
    for _ in 0..dist {
        head.1 += 1;
        if !is_adjacent(head, tail) {
            tail.1 += 1;
            tail.0 = tail.0 + (head.0 - tail.0);
            moves.push((tail.0, tail.1));
        }
    }
    moves
}
fn move_left(dist: u32, head: &mut Point, tail: &mut Point) -> Vec<Point>{
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
fn move_up(dist: u32, head: &mut Point, tail: &mut Point) -> Vec<Point>{
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

pub fn day_nine() {
    let lines = include_str!("../input/day9-input.txt").lines();
    let mut head = (0, 0);
    let mut tail = (0, 0);
    let mut visited:HashSet<Point> = HashSet::new();
    visited.insert(tail);
    let mut tail_moves: Vec<Point> = Vec::new();
    for line in lines {
        let (dir, dist) = line.split_once(' ').unwrap();
        let dist: u32 = dist.parse().unwrap();
        match dir {
            "R" => {
                tail_moves.append(&mut move_right(dist, &mut head, &mut tail));
            }
            "L" => {
                tail_moves.append(&mut move_left(dist, &mut head, &mut tail));
            }
            "U" => {
                tail_moves.append(&mut move_up(dist, &mut head, &mut tail));
            }
            "D" => {
                tail_moves.append(&mut move_down(dist, &mut head, &mut tail));
            }
            _ => println!("Unknown"),
        }
    }
        println!("Tail moves: {:?}", tail_moves);
        tail_moves.iter().for_each(|t| {
            visited.insert(*t);
        }
        );
        println!("Visited: {}", visited.len());
}
