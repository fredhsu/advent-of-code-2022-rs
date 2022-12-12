fn is_visible(trees: &Vec<Vec<u32>>, row: usize, col: usize) -> bool {
    let num_rows = trees[0].len();
    let num_cols = trees.len();
    let val = trees[row][col];

    if row == 0 || col == 0 || row == num_rows - 1 || col == num_cols - 1 {
        return true;
    }
    // test from left
    let left = &trees[row][0..col];
    //println!("Testing {val} from the left against {left:#?} for row: {row} col: {col}");
    if left.iter().all(|x| *x < val) {
        //println!("{val} is visibile from the left");
        return true;
    }
    // test from right
    let right = &trees[row][col + 1..num_cols];
    //println!("Testing {val} from the right against {right:#?} for row: {row} col: {col}");
    if right.iter().all(|x| *x < val) {
        //println!("{val} is visibile from the right");
        return true;
    }
    // test from up
    let column: Vec<u32> = trees.iter().map(|row| row[col]).collect();
    //println!("column is: {column:#?} for col {col}");
    let up = &column[0..row];
    //println!("Testing {val} from the up against {up:#?} for row: {row} col: {col}");
    if up.iter().all(|x| *x < val) {
        //println!("{val} is visibile from up");
        return true;
    }
    // test from down
    let down = &column[row + 1..num_rows];
    //println!("Testing {val} from the down against {down:#?} for row: {row} col: {col}");
    if down.iter().all(|x| *x < val) {
        //println!("{val} is visibile from down");
        return true;
    }

    false
}

fn scenic_score(trees: &Vec<Vec<u32>>, row: usize, col: usize) -> u32 {
    println!("Getting score for r:{row} c:{col}");
    let mut total = 0;
    let num_rows = trees[0].len();
    let num_cols = trees.len();
    let val = trees[row][col];
    // left total

    let left = &trees[row][0..col];
    if col != 0 {
        let left_view: Vec<&u32> = left.iter().rev().take_while(|x| *x < &val).collect();
        let mut left_score = left_view.len();
        if left_score < col {
            left_score += 1;
        }
        println!("left score: {}", left_score);
        total = left_score;
    }

    // right total
    let right = &trees[row][col + 1..num_cols];
    if col != num_cols - 1 {
        let right_view: Vec<&u32> = right.iter().take_while(|x| *x < &val).collect();
        let mut right_score = right_view.len();
        if right_score < right.len() - 1 {
            right_score += 1;
        }
        println!("rightscore: {}", right_score);
        if total == 0 {
            total = right_score;
        } else {
            total *= right_score;
        }
    }
    let column: Vec<u32> = trees.iter().map(|row| row[col]).collect();
    // up total
    let up = &column[0..row];
    // down total
    let down = &column[row + 1..num_rows];
    total as u32
}

pub fn day_eight() {
    let lines = include_str!("../input/day8-test.txt").lines();
    let mut trees = Vec::new();
    for line in lines {
        let trees_row: Vec<u32> = line
            .chars()
            .map(|char| (char.to_digit(10).unwrap()))
            .collect();
        trees.push(trees_row);
    }
    let mut count = 0;
    for r in 0..trees.len() {
        for c in 0..trees[0].len() {
            //println!("row: {r} col: {c} is_vis: {}", is_visible(&trees, r, c));
            if is_visible(&trees, r, c) {
                count += 1;
            }
        }
    }
    println!("Count is {count}");
    for r in 0..trees.len() {
        for c in 0..trees[0].len() {
            println!("row: {r} col: {c} score: {}", scenic_score(&trees, r, c));
        }
    }
}
