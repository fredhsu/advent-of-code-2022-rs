use std::fs;
pub fn day_six() -> Result<(), std::num::ParseIntError> {
    let file_path = "day6-test.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    Ok(())
}
