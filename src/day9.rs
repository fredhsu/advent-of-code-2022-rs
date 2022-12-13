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
fn move_right(dist: u32, head: &mut Point, tail: &mut Point) {
    for i in 0..dist {
        head.1 += 1;
        println!("adjacent: {}", is_adjacent(head, tail));
    }
}
pub fn day_nine() {
    let lines = include_str!("../input/day9-test.txt").lines();
    let mut head = (0, 0);
    let mut tail = (0, 0);
    for line in lines {
        let (dir, dist) = line.split_once(" ").unwrap();
        let dist: u32 = dist.parse().unwrap();
        match dir {
            "R" => {
                move_right(dist, &mut head, &mut tail);
            }
            "L" => {}
            "U" => {}
            "D" => {}
            _ => println!("Unknown"),
        }
        println!("head: {head:#?}, tail: {tail:#?}");
    }
}
