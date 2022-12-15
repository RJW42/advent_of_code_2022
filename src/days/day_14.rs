use std::fs;
use std::io::BufReader;
use std::io::prelude::*;

use std::iter::Peekable;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

use std::cmp::Ordering::*;

use crate::d14::CaveEntity::*;
use crate::d14::Heading::*;

struct CaveSystem {
    entities: HashMap<(u32, u32), CaveEntity>,
    sand_path: Option<HashSet<(u32, u32)>>,
    sand_source: (u32, u32),
    top_left: (u32, u32),
    width: u32,
    height: u32,
    has_floor: bool,
}

enum SandDrop {
    Ok((u32, u32)),
    Void,
    Full,
}

enum CaveEntity {
    Rock,
    Sand,
}

enum Heading {
    Up,
    Down,
    Left,
    Right,
}

struct Direction {
    heading: Heading,
    distance: u32,
}

struct RockDefinition {
    start_pos: (u32, u32),
    path: VecDeque<Direction>,
}


pub fn run(file_name: &str) -> std::io::Result<()> {
    let mut cave_system = parse_cave_system(file_name, true)?;

    println!("width: {}, height: {}", cave_system.width, cave_system.height);
    cave_system.print();

    let mut count = 0;

    while let SandDrop::Ok(_) = cave_system.spawn_sand() {
        count += 1;
        cave_system.print();
    }

    println!("count: {}", count);

    Ok(())
}


impl CaveSystem {
    fn spawn_sand(&mut self) -> SandDrop {
        let result = self.drop_sand(&self.sand_source);
        
        if let SandDrop::Ok(pos) = &result {
            println!("{}, {}", pos.0, pos.1);
            self.add_entitie(*pos, Sand);
        }

        result
    }

    fn drop_sand(&self, pos: &(u32, u32)) -> SandDrop {
        let mut current_pos = *pos;

        loop {
            // Check if in void
            if current_pos.1 > self.height {
                return SandDrop::Void;
            }

            // Try going down
            let down = (current_pos.0, current_pos.1 + 1);

            if self.space_empty(&down) {
                current_pos = down;
                continue;
            }

            // Try going down left
            let down_left = (current_pos.0 - 1, current_pos.1 + 1);

            if self.space_empty(&down_left) {
                current_pos = down_left;
                continue;
            }

            // Try going down right 
            let down_right = (current_pos.0 + 1, current_pos.1 + 1);

            if self.space_empty(&down_right) {
                current_pos = down_right;
                continue;
            }

            // Can't go any furhter 
            if current_pos == self.sand_source {
                return SandDrop::Full;
            }

            return SandDrop::Ok(current_pos);
        }
    }

    fn space_empty(&self, pos: &(u32, u32)) -> bool {
        if self.has_floor && pos.1 == self.height {
            return false;
        }

        !self.entities.contains_key(pos) 
    }

    fn add_rock(&mut self, rock: RockDefinition) {
        self.sand_path = None;
        self.add_entitie(rock.start_pos, Rock);

        let mut pos = rock.start_pos;

        // Follow path
        for direction in &rock.path {
            for _ in 0..direction.distance {
                pos = advance_pos_in_direction(pos, &direction.heading);
                self.add_entitie(pos, Rock);
            }
        }
    }

    fn add_entitie(&mut self, pos: (u32, u32), entitie: CaveEntity) {
        self.entities.insert(pos, entitie);

        // Update witdh
        if pos.0 < self.top_left.0 {
            self.width += self.top_left.0 - pos.0;
            self.top_left.0 = pos.0;
        } else if pos.0 >= self.top_left.0 + self.width {
            self.width = (pos.0 - self.top_left.0) + 1;
        }

        // Update height
        if pos.1 >= self.height {
            self.height = pos.1 + 1;
        }
    }

    fn print(&self) {
        // Todo: print header
        let chars_for_left = length(self.height - 1, 10);

        for row in 0..self.height {
            self.print_row(row, chars_for_left);
        }

        // print floor
        if self.has_floor {
            print!("{} ", self.height);
            println!(
                "{:#<1$}", "", (self.width) as usize
            );
        }
    }

    fn print_row(&self, row: u32, chars_for_left: u32) {
        // Print row number 
        print!(
            "{: <1$}", "", (chars_for_left - length(row, 10)) as usize
        );
        print!("{} ", row);

        // Print elements
        let row_i = self.top_left.1 + row;

        for col in 0..self.width {
            let col_i = self.top_left.0 + col;
            let pos = (col_i, row_i);

            if pos == self.sand_source {
                print!("+");
                continue;
            }

            match self.entities.get(&pos) {
                Some(Rock) => print!("#"),
                Some(Sand) => print!("o"),
                None => print!("."),
            };
        }

        println!();
    }
}

impl RockDefinition {
    fn print(&self) {
        print!("s({}, {}): ", self.start_pos.0, self.start_pos.1);

        for direction in &self.path {
            match direction.heading {
                Up => print!("U"),
                Down => print!("D"),
                Left => print!("L"),
                Right => print!("R"),
            };
            print!(" {} -> ", direction.distance);
        }

        println!();
    }
}

fn advance_pos_in_direction(pos: (u32, u32), heading: &Heading) -> (u32, u32) {
    match heading {
        Up => (pos.0, pos.1 + 1),
        Down => (pos.0, pos.1 - 1),
        Left => (pos.0 - 1, pos.1),
        Right => (pos.0 + 1, pos.1),
    }
}



/* Parsing */

fn parse_cave_system(file_name: &str, has_floor: bool) -> std::io::Result<CaveSystem> {
    let file = fs::File::open(file_name)
        .expect("File not found");
    let reader = BufReader::new(file);

    let mut cave_system = CaveSystem {
        entities: HashMap::new(),
        sand_path: None,
        sand_source: (500, 0),
        top_left: (500, 0),
        width: 1,
        height: 1,
        has_floor: has_floor,
    };

    for l in reader.lines() {
        let rock = parse_rock(&l?);
        rock.print();
        cave_system.add_rock(rock);
    }

    if has_floor {
        cave_system.width += 2;
        cave_system.height += 1;
        cave_system.top_left.0 -= 1;
    }

    Ok(cave_system)
}

fn parse_rock(line: &str) -> RockDefinition {
    let chars = line.chars().collect::<Vec<char>>();
    let mut chars_iter = chars.iter().peekable();
    let mut points = Vec::new();

    while let Some(pair) = parse_pair(&mut chars_iter) {
        points.push(pair);
    }

    let mut path = VecDeque::new();

    for i in 0..(points.len() - 1) {
        let start = points[i];
        let next = points[i + 1];
        
        // Determine direction 
        let ordering = (
            start.0.cmp(&next.0),
            start.1.cmp(&next.1),
        );

        let (heading, distance) = match ordering {
            (Equal, Less) => (Up, next.1 - start.1),
            (Equal, Greater) => (Down, start.1 - next.1),
            (Less, Equal) => (Right, next.0 - start.0),
            (Greater, Equal) => (Left, start.0 - next.0),
            _ => panic!(),
        };

        path.push_back( Direction {
            heading: heading,
            distance: distance,
        });
    }


    RockDefinition {
        start_pos: points[0],
        path: path,
    }
}

fn parse_pair<'a, I>(chars: &mut Peekable<I>) -> Option<(u32, u32)> 
where I: Iterator<Item = &'a char> {
    let l = parse_num(chars)?;
    chars.next()?; // Skip ','
    let r = parse_num(chars)?;
    chars.next(); chars.next(); // Skip ' -> ' if present
    chars.next(); chars.next();
    Some((l, r))
}

fn parse_num<'a, I>(chars: &mut Peekable<I>) -> Option<u32> 
where I: Iterator<Item = &'a char> {
    let mut output = None;
    let mut output_val = 0;

    loop {
        match chars.peek() {
            ch @ Some('0'..='9') => {
                output_val = output_val * 10 + (
                    **ch.unwrap() as u32 - '0' as u32
                );
                output = Some(output_val);
                chars.next();
            },
            _ => break,
        };
    }

    output
}


fn length(n: u32, base: u32) -> u32 {
    let mut power = base;
    let mut count = 1;
    while n >= power {
        count += 1;
        if let Some(new_power) = power.checked_mul(base) {
            power = new_power;
        } else {
            break;
        }
    }
    count
}