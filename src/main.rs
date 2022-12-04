use itertools::Itertools;
use std::fs;

fn choice_score(round: (&str, &str)) -> u32 {
    match round {
        (_, "X") => 1,
        (_, "Y") => 2,
        (_, "Z") => 3,
        (_, _) => 0,
    }
}

fn result_score(round: (&str, &str)) -> u32 {
    match round {
        ("A", "X") | ("B", "Y") | ("C", "Z") => 3,
        ("B", "X") | ("C", "Y") | ("A", "Z") => 0,
        ("C", "X") | ("A", "Y") | ("B", "Z") => 6,
        _ => 0,
    }
}

fn day_two() -> Result<(), std::num::ParseIntError> {
    let file_path = "day2-input.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let mut total = 0;
    let mut total2 = 0;
    for line in contents.lines() {
        let mut s = line.split(' ');
        let round = (s.next().unwrap(), s.next().unwrap());
        total += choice_score(round);
        total += result_score(round);
        let choose = match round {
            (a, "X") => match a {
                "A" => (a, "Z"),
                "B" => (a, "X"),
                _ => (a, "Y"),
            },
            (a, "Y") => match a {
                "A" => (a, "X"),
                "B" => (a, "Y"),
                _ => (a, "Z"),
            },
            (a, "Z") => match a {
                "A" => (a, "Y"),
                "B" => (a, "Z"),
                _ => (a, "X"),
            },
            _ => round,
        };
        total2 += choice_score(choose);
        total2 += result_score(choose);
    }
    println!("total: {}", total);
    println!("total2: {}", total2);
    Ok(())
}

fn day_one() -> Result<(), std::num::ParseIntError> {
    let file_path = "day1-input.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut calorie_count = 0;
    let mut elves = Vec::new();
    for line in contents.lines() {
        if line.is_empty() {
            elves.push(calorie_count);
            // set up next elf
            calorie_count = 0;
        } else {
            calorie_count += line.parse::<u32>()?;
        }
    }
    elves.sort();
    let biggest = elves.pop().unwrap();
    println!("Most calories {}", biggest);

    let top3 = biggest + elves.pop().unwrap() + elves.pop().unwrap();
    println!("Top 3 calories {}", top3);
    Ok(())
}

fn letter_priority(c: char) -> u32 {
    let val;
    if c.is_uppercase() {
        let l = c.to_lowercase().next().unwrap();
        val = l as u32 - 70;
    } else {
        let u = c.to_uppercase().next().unwrap();
        val = u as u32 - 64;
    }
    val
}

fn day3_part1(contents: &str) {
    let mut total = 0;
    for line in contents.lines() {
        let (first, last) = line.split_at(line.len() / 2);
        let mut dup_letter = 'a';
        for letter in first.chars() {
            if last.to_string().contains(letter) {
                dup_letter = letter;
                break;
            }
        }
        let priority = letter_priority(dup_letter);
        total += priority;
    }
    println!("Part 1 Total: {}", total);
}

fn day3_part2(contents: &str) {
    let mut total = 0;
    let mut lines = contents.lines();
    while let Some(line1) = lines.next() {
        let line2 = lines.next().unwrap();
        let line3 = lines.next().unwrap();
        for letter in line1.chars() {
            if line2.to_string().contains(letter) && line3.to_string().contains(letter) {
                total += letter_priority(letter);
                break;
            }
        }
    }
    println!("Part 2 Total: {}", total);
}
fn day3_part2_chunk(contents: &str) {
    let mut total = 0;
    let chunks = contents.lines().chunks(3);
    for chunk in &chunks {
        println!("chunk");
        // send over a collected vector instead of a chunk interator
        //three_chunk_priority(chunk);
        for line in chunk {
            println!("line: {}", line);
        }
    }
    println!("Part 2 Total: {}", total);
}

/*
rewrite to take in a vector of 3 chunk vs iterator
fn three_chunk_priority(chunk: itertools::Chunk<std::io::Lines<&str>>) -> u32 {
    0
}
*/
fn day_three() {
    let file_path = "day3-test.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    day3_part1(&contents);
    day3_part2(&contents);
    day3_part2_chunk(&contents);
}

fn main() {
    day_three();
}
