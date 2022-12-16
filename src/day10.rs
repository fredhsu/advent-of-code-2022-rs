fn check_cycle(cycle: i32, x: i32) -> i32 {
    if cycle == 20 || (cycle - 20) % 40 == 0 {
        let signal = cycle * x;
        println!("cycle: {cycle} -- register: {x} -- signal: {signal}");
        return signal;
    }
    0
}

fn print_pixel(cycle: i32, x: i32) {
    let position = (cycle - 1) % 40;
    let sprite = vec![x - 1, x, x + 1];
    if sprite.contains(&position) {
        print!("#");
    } else {
        print!(".");
    }
    if cycle % 40 == 0 {
        println!();
    }
}

pub fn day_ten() {
    let lines = include_str!("../input/day10-input.txt").lines();
    let mut cycle = 1;
    let mut x = 1;
    let mut sum = 0;
    for line in lines {
        if let Some((instruction, n)) = line.split_once(' ') {
            print_pixel(cycle, x);
            cycle += 1;
            //sum += check_cycle(cycle, x);
            print_pixel(cycle, x);
            cycle += 1;
            x += n.parse::<i32>().unwrap();
            //x = x % 40;
            //sum += check_cycle(cycle, x);
        } else {
            print_pixel(cycle, x);
            cycle += 1;
            //sum += check_cycle(cycle, x);
        };
    }
    //println!("Sum: {sum}");
}
