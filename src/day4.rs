use std::fs;
pub fn day_four() {
    let file_path = "day4-input.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let mut total = 0;
    let mut overlap = 0;
    for line in contents.lines() {
        // switch to split_once and get a tuple
        let pair = line.split(',');
        let pair2 = pair
            .map(|x| {
                x.split("-")
                    .map(|x| x.parse::<u32>().unwrap())
                    .collect::<Vec<u32>>()
            })
            .collect::<Vec<Vec<u32>>>();
        // if the upper bound of the first is less than the lower bound of
        // the second there will be overlap, but also need to make sure the lower bound of the
        // first is below the upper bound of the second otherwise it would
        // be completely outside for ex:  3-7, 8-9
        if pair2[0][1] >= pair2[1][0] && pair2[0][0] <= pair2[1][1] {
            println!("Overlap: {:?}", pair2);
            overlap += 1;
        } else {
            println!("Does not overlap {:?}", pair2);
        }
        if pair2[0][0] <= pair2[1][0] && pair2[0][1] >= pair2[1][1] {
            println!(
                "first pair fits: {} <= {} and {} >= {}",
                pair2[0][0], pair2[0][1], pair2[0][1], pair2[1][1]
            );
            total += 1;
            continue;
        } else if pair2[1][0] <= pair2[0][0] && pair2[1][1] >= pair2[0][1] {
            println!("second pair fits: {:?}", pair2);
            total += 1;
        } else {
            println!("does not fit");
        }
    }
    println!("total: {} ", total);
    println!("overlap: {}", overlap);
}
