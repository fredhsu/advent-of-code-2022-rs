use std::{fs, str};

#[derive(Debug)]
enum Command {
    CD(String),
    DIR(String),
    FILE(u32),
    LS,
}

#[derive(Debug)]
struct FileSystem<'a> {
    pub parent: Option<Box<FileSystem<'a>>>,
    pub name: String,
    pub file_size: u32,
    pub children: Vec<&'a FileSystem<'a>>,
}

impl FileSystem<'_> {
    fn add_child(&mut self, child: &FileSystem) {
        self.children.push(child);
        self.file_size += child.file_size;
    }
    fn contains(&self, child: &str) -> bool {
        for c in self.children {
            if c.name == child.to_string() {
                return true;
            }
        }
        false
    }
    fn cd(&self, child: &str) -> &FileSystem {
        for c in self.children {
            if c.name == child.to_string() {
                return c;
            }
        }
        self
    }
}
fn parse_command(line: &str) -> Command {
    let tokens: Vec<&str> = line.split(' ').collect();
    match tokens[1] {
        "cd" => Command::CD(tokens[2].to_string()),
        _ => Command::LS,
    }
}

fn parse_line(line: &str, filesystem: &FileSystem) -> Command {
    let tokens: Vec<&str> = line.split(' ').collect();
    match tokens[0] {
        "$" => match tokens[1] {
            "cd" => Command::CD(tokens[2].to_string()),
            _ => Command::LS,
        },
        "dir" => Command::DIR(tokens[1].to_string()),
        x => Command::FILE(x.parse().unwrap()),
    }
}
fn parse_ls(lines: &Vec<&str>) {
    for line in lines {
        println!("listing line: {}", line);
    }
}
pub fn day_seven() -> Result<(), str::Utf8Error> {
    let file_path = "input/day7-test.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let root = FileSystem {
        parent: None,
        name: "root".to_string(),
        file_size: 0,
        children: Vec::new(),
    };
    let mut pwd = root;
    let mut lines = contents.lines();

    for line in lines {
        let line_type = parse_line(line, &pwd);
        match line_type {
            Command::CD(d) => {
                // check if directory exists
                if pwd.contains(&d) {
                    pwd = *pwd.cd(&d);
                } else {
                    // add new directory entry
                    let child = FileSystem {
                        parent: Some(Box::new(pwd)),
                        name: d.to_string(),
                        file_size: 0,
                        children: Vec::new(),
                    };
                    pwd.add_child(&child);
                }
            }
            _ => (),
        }
        println!("line: {:?}", &line_type);
    }
    println!("Files: {:?}", pwd);
}
