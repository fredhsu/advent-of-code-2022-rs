use std::collections::VecDeque;
use std::fs;

pub fn day_five() -> Result<(), std::num::ParseIntError> {
    let file_path = "input/day5-input.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    // Could figure out a way to read the number of stacks and the height
    let num_stack = 9;
    let stack_height = 8;

    let mut stacks = Vec::new();
    for _i in 0..num_stack {
        let stack: VecDeque<char> = VecDeque::new();
        stacks.push(stack)
    }
    // read in first n rows as stacks
    let mut lines = contents.lines();
    let stack_lines = lines.by_ref().take(stack_height);
    for line in stack_lines {
        //for i in 0..num_stack {
        for (i, item) in stacks.iter_mut().enumerate().take(num_stack) {
            let pkg = line.chars().nth(i * 4 + 1);
            //println!("{} :: {:?}", i, &pkg);
            let stack_pkg = match pkg {
                Some(' ') => None,
                Some(x) => Some(x),
                None => None,
            };
            if let Some(s) = stack_pkg {
                item.insert(0, s);
                // stacks[i].insert(0, s);
            }
            //println!("{:?}", stacks[i]);
            //println!("{:?}", item);
        }
    }

    // skip by the numbered columns and the extra space
    let buffer_lines = lines.by_ref().take(2);
    for _ in buffer_lines {}

    // read in instructions
    for line in lines {
        let instruction = line.split(' ').collect::<Vec<&str>>();
        let num_moves = instruction[1].parse().unwrap();
        let source: usize = instruction[3].parse::<usize>().unwrap() - 1;
        let dest: usize = instruction[5].parse::<usize>().unwrap() - 1;
        let mut buf = VecDeque::new();
        for _ in 0..num_moves {
            let c: char = stacks[source].pop_back().unwrap();
            //stacks[dest].push_back(c); // use this for part 1
            buf.push_front(c); // use this for part 2
        }
        stacks[dest].append(&mut buf); // use this for part 2
    }

    print_stacks(&stacks);
    Ok(())
}

fn print_stacks(stacks: &Vec<VecDeque<char>>) {
    println!("Print stacks:");
    for stack in stacks {
        println!("{:?}", stack);
    }
    println!("top of stacks");
    for stack in stacks {
        println!("{:?}", stack.back());
    }
}
