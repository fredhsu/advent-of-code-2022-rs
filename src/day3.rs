use itertools::Itertools;
use std::fs;

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
pub fn day_three() {
    let file_path = "day3-test.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    day3_part1(&contents);
    day3_part2(&contents);
    day3_part2_chunk(&contents);
}
