use std::{fs, str};

pub fn day_six() -> Result<(), str::Utf8Error> {
    let file_path = "input/day6-input.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    //let win_size = 4;
    let win_size = 14;
    let mut index = 0;
    for w in contents.as_bytes().windows(win_size) {
        let mut win_str = String::from(str::from_utf8(w)?);
        while let Some(c) = win_str.pop() {
            if win_str.contains(c) {
                break;
            }
        }
        if win_str.len() == 0 {
            println!("index: {} marker: {}", index, index + win_size);
            break;
        } else {
            index += 1;
        }
    }
    Ok(())
}
