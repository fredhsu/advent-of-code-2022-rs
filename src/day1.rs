use std::fs;
pub fn day_one() -> Result<(), std::num::ParseIntError> {
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
