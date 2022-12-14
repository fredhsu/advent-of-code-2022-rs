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

pub fn day_two() -> Result<(), std::num::ParseIntError> {
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
