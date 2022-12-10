pub fn day_eight() {
    let lines = include_str!("../input/day8-test.txt").lines();
    //let trees = Vec::new();
    for line in lines {
        let mut trees_row = Vec::new();
        for char in line.chars() {
            trees_row.push(char.to_digit(10).unwrap());
        }
        println!("{:?}", trees_row);
    }
}
