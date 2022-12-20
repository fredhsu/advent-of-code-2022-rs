#[derive(Debug, Copy, Clone)]
struct Square {
    pub height: Height,
    pub visited: bool,
}

impl Square {
    pub fn new(height: Height) -> Self {
        Square {
            visited: false,
            height,
        }
    }
    pub fn is_end(&self) -> bool {
        self.height == Height::End
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Height {
    Start,
    End,
    Position(u32),
}
impl Height {
    pub fn get_value(&self) -> u32 {
        match self {
            Height::Start => 1,
            Height::End => 27,
            Height::Position(x) => *x,
        }
    }
}

struct Grid {
    squares: Vec<Vec<Square>>,
    height: usize,
    width: usize,
}

type Coord = (usize, usize);

impl Grid {
    pub fn is_inbounds(&self, current: &(isize, isize)) -> bool {
        current.0 >= 0
            && current.1 >= 0
            && current.0 < (self.height as isize)
            && current.1 < (self.width as isize)
    }
    pub fn get(&self, square: &Coord) -> &Square {
        &self.squares[square.0][square.1]
    }
    pub fn visit(&mut self, square: &Coord) {
        self.squares[square.0][square.1].visited = true;
    }
    pub fn reset_visit(&mut self) {
        for i in 0..self.height {
            for j in 0..self.width {
                self.squares[i][j].visited = false;
            }
        }
    }
    pub fn is_valid(&self, current: &Coord, neighbor: &Coord) -> bool {
        let neighbor_height = self.get(neighbor).height.get_value();
        /*
                println!(
                    "is_valid: current height {:?}, neighbor height {:?} value: {:?}",
                    self.get(current).height,
                    self.get(neighbor).height,
                    neighbor_height,
                );
        */
        match self.get(current).height {
            Height::Start => neighbor_height == 1,
            Height::End => neighbor_height == 26,
            Height::Position(x) => {
                if neighbor_height > x + 1 {
                    return false;
                }
                true
            }
        }
    }
    pub fn find_candidates(&self, current: Coord) -> Vec<Coord> {
        //let current_square = self.squares[current.0][current.1];
        let moves: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        moves
            .iter()
            .map(|m| (m.0 + (current.0 as isize), m.1 + (current.1 as isize)))
            .filter(|m| self.is_inbounds(m))
            .map(|m| (m.0 as usize, m.1 as usize))
            .filter(|m| self.is_valid(&current, m))
            .collect()
    }
    pub fn start_coord(&self) -> Coord {
        for (i, row) in self.squares.iter().enumerate() {
            for (j, square) in row.iter().enumerate() {
                if square.height == Height::Start {
                    return (i, j);
                }
            }
        }
        println!("Didn't find a start");
        (0, 0)
    }
}
fn parse_squares_row(s: &str) -> Vec<Square> {
    s.chars()
        .map(|c| {
            let height = match c {
                'S' => Height::Start,
                'E' => Height::End,
                _ => Height::Position(c as u32 - 96),
            };
            Square::new(height)
        })
        .collect()
}
fn parse_squares(s: &str) -> Vec<Vec<Square>> {
    //let (s, m) = parse_squares_row(s)?;
    let mut squares = Vec::new();
    for line in s.lines() {
        let m = parse_squares_row(line.trim());
        squares.push(m);
    }
    squares
}

pub fn day_twelve() {
    let lines = include_str!("../input/day12-input.txt");
    let squares = parse_squares(lines);
    let height = squares.len();
    let width = squares[0].len();
    println!("Height: {height}, Width: {width}");
    let mut grid = Grid {
        squares,
        height,
        width,
    };
    let mut shortest = grid.height * grid.width;

    let mut starting_points = Vec::new();
    for i in 0..grid.height {
        for j in 0..grid.width {
            if grid.get(&(i, j)).height.get_value() <= 1 {
                starting_points.push((i, j));
            }
        }
    }
    println!("Possible starting points: {starting_points:?}");
    //let start = grid.start_coord();
    for start in starting_points {
        println!("Trying position: {start:?}");
        let mut candidates = grid.find_candidates(start);
        let mut new_candidates = Vec::new();
        let mut count = 1;
        while !candidates.iter().any(|a| grid.get(a).is_end()) && count < shortest {
            for c in candidates {
                grid.visit(&c);
                let temp = grid.find_candidates(c);
                for t in temp {
                    if new_candidates.contains(&t) || grid.get(&t).visited {
                        continue;
                    }
                    new_candidates.push(t);
                }
            }
            //println!("New candidates: {new_candidates:?}");
            candidates = new_candidates.clone();
            new_candidates.clear();
            count += 1;
        }
        println!("count : {count}");
        if count < shortest {
            shortest = count;
        }
        grid.reset_visit();
    }
    println!("Shortest : {shortest}");
}
