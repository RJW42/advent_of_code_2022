use std::fs;
use std::io::BufReader;
use std::io::prelude::*;

use std::collections::HashMap;
use std::collections::BTreeMap;
use std::iter::Peekable;

use std::cmp;

use crate::d15::Entity::*;

struct SesnorData {
    entities: HashMap<(i64, i64), Entity>,
    sensor_dist: HashMap<(i64, i64), u32>,
    sensor_space: HashMap<(i64, i64), Space>,
    top_left: (i64, i64),
    width: u32,
    height: u32,
}

struct Space {
    top_left: Line,
    top_right: Line,
    bottom_left: Line,
    bottom_right: Line,
}

struct Line {
    a: i64,
    b: i64, 
    c: i64,
    min_x: i64,
    min_y: i64, 
    max_x: i64,
    max_y: i64,
}

enum Entity {
    Beacon,
    Sesnor,
}


pub fn run(file_name: &str) -> std::io::Result<()> {
    let sensor_data = new_sesnor_data(
        parse_sesnor_data(file_name)?
    );

    //sensor_data.print(false);
    //println!();
    //sensor_data.print(true);

    // Part One
    let mut score = 0;
    let disaply = false;

    let y = 2000000;
    let min_x = sensor_data.top_left.0;
    let max_x = sensor_data.top_left.0 + sensor_data.width as i64;

    if disaply {
        sensor_data.print(false);   
    }

    for x in min_x..max_x {
        if !sensor_data.could_contain_beacon((x, y)) {
            score += 1;
        };
    }

    println!("score: {}", score);

    // Part Two
    let max_y = 4000000;
    let max_x = 4000000;

    for y in 0..max_y {
        let line = Line {
            a: 0,
            b: 1,
            c: -y,
            min_x: 0,
            max_x: max_x,
            min_y: y,
            max_y: y
        };

        let mut sizes = BTreeMap::new();


        print!("y={}: ", y);

        for (_, space) in &sensor_data.sensor_space {
            if let Some((start, end)) = space.intersects(&line) {
                print!("{} -> {}, ", start.0, end.0);

                let size = end.0 - start.0;

                if size < 0 {
                    panic!();
                }

                if let Some(old_size) = sizes.get(&start.0) {
                    if size > *old_size {
                        sizes.insert(start.0, size);    
                    }
                } else {
                    sizes.insert(start.0, size);
                } 
            }
        }

        println!();

        let mut current_max_x = 0;

        for (x, size) in sizes.iter() {
            if *x > current_max_x {
                println!("solution: {}, {}", *x - 1, y);
                println!("{}", (*x - 1) * 4000000 + y);
                return Ok(())
            }

            let next_x = *x + *size;

            if next_x > current_max_x {
                current_max_x = next_x;

                if current_max_x > max_x {
                    break;
                }
            }
        }
    }

    Ok(())
}


fn new_sesnor_data(data: Vec<((i64, i64), (i64, i64))>) -> SesnorData {
    let mut output = SesnorData {
        entities: HashMap::new(),
        sensor_dist: HashMap::new(),
        sensor_space: HashMap::new(),
        top_left: (0, 0),
        width: 1,
        height: 1,
    };

    for (sesnor_pos, beacon_pos) in data {
        println!("s:{},{} b:{},{}", 
            sesnor_pos.0, sesnor_pos.1,
            beacon_pos.0, beacon_pos.1
        );

        output.add_sesnor_data(sesnor_pos, beacon_pos);
    }   

    output
}


impl SesnorData {
    fn add_sesnor_data(&mut self, 
        sesnor_pos: (i64, i64), beacon_pos: (i64, i64)
    ) {
        self.add_entity(sesnor_pos, Sesnor);
        self.add_entity(beacon_pos, Beacon);

        let dist = manhattan(sesnor_pos, beacon_pos) as i64;

        let space = new_sensor_space(sesnor_pos, dist);

        self.sensor_space.insert(sesnor_pos, space);
        self.sensor_dist.insert(sesnor_pos, dist as u32);

        // Need to update to left and width 
        self.update_size((sesnor_pos.0 + dist, sesnor_pos.1));
        self.update_size((sesnor_pos.0 - dist, sesnor_pos.1));
        self.update_size((sesnor_pos.0, sesnor_pos.1 + dist));
        self.update_size((sesnor_pos.0, sesnor_pos.1 - dist));
    }

    fn could_contain_beacon(&self, pos: (i64, i64)) -> bool {
        match self.entities.get(&pos) {
            Some(Beacon) => return true,
            _ => (),
        };

        for (sesnor_pos, dist) in &self.sensor_dist {
            let dist_to_sesnor = manhattan(pos, *sesnor_pos);

            if dist_to_sesnor <= *dist {
                return false;
            }
        }

        true
    }


    fn add_entity(&mut self, 
        pos: (i64, i64), entity: Entity
    ) {
        match self.entities.get(&pos) {
            None => { self.entities.insert(pos, entity); },
            _ => (),
        };

        self.update_size(pos);
    }


    fn update_size(&mut self, pos: (i64, i64)) {
        // Update width 
        if pos.0 < self.top_left.0 {
            self.width += (self.top_left.0 - pos.0) as u32;
            self.top_left.0 = pos.0;
        } else if pos.0 >= self.top_left.0 + self.width as i64 {
            self.width = (pos.0 - self.top_left.0) as u32 + 1;
        }

        // Update height
        if pos.1 < self.top_left.1 {
            self.height += (self.top_left.1 - pos.1) as u32;
            self.top_left.1 = pos.1;
        } else if pos.1 >= self.top_left.1 + self.height as i64 {
            self.height = (pos.1 - self.top_left.1) as u32 + 1;
        }
    }


    fn print(&self, display_no_beacon: bool) {
        // Todo: print header
        let chars_for_left = cmp::max(
            length(self.top_left.1, 10),
            length(self.top_left.1 + self.height as i64, 10)
        );

        for row in self.top_left.1..(self.top_left.1 + self.height as i64) {
            self.print_row(row, chars_for_left, display_no_beacon);
        }
    }


    fn print_row(&self, row: i64, chars_for_left: u32, display_no_beacon: bool) {
        // Print row number 
        print!(
            "{: <1$}", "", (chars_for_left - length(row, 10)) as usize
        );
        print!("{} ", row);

        // Print elements
        for col in self.top_left.0..(self.top_left.0 + self.width as i64) {
            match self.entities.get(&(col, row)) {
                Some(Beacon) => print!("B"),
                Some(Sesnor) => print!("S"),
                None => if self.could_contain_beacon((col, row)) || 
                           !display_no_beacon {
                    print!(".");
                } else {
                    print!("#");
                },
            };
        }

        println!();
    }
}


impl Space {
    fn intersects(&self, line: &Line) -> Option<((i64, i64), (i64, i64))> {
        match (self.top_left.intersects(line), self.top_right.intersects(line)) {
            (Some(p1), Some(p2)) => return Some((p1, p2)),
            _ => ()
        };

        match (self.bottom_left.intersects(line), self.bottom_right.intersects(line)) {
            (Some(p1), Some(p2)) => return Some((p1, p2)),
            _ => ()
        };

        None
    }
}


impl Line {
    fn intersects(&self, other: &Line) -> Option<(i64, i64)> {
        let x_numerator = self.b * other.c - other.b * self.c;
        let x_denominator = self.a * other.b - other.a * self.b;
        let y_numerator = self.c * other.a - other.c * self.a;
        let y_denominator = self.a * other.b - other.a * self.b;

        if x_denominator == 0 || y_denominator == 0 {
            return None;
        }

        let x = x_numerator / x_denominator;
        let y = y_numerator / y_denominator;

        if x < self.min_x || x > self.max_x || 
           y < self.min_y || y > self.max_y {
            return None;
        }

        Some((x, y))
    }
}


fn new_sensor_space(p: (i64, i64), dist_to_beacon: i64) -> Space {
    let top = (p.0, p.1 + dist_to_beacon);
    let left = (p.0 - dist_to_beacon, p.1);
    let right = (p.0 + dist_to_beacon, p.1);
    let bottom = (p.0, p.1 - dist_to_beacon);
    
    Space {
        top_left: new_line(left, top),
        top_right: new_line(top, right),
        bottom_left: new_line(left, bottom),
        bottom_right: new_line(bottom, right),
    }
}


fn new_line(p1: (i64, i64), p2: (i64, i64)) -> Line {
    let slope_numerator = p2.1 - p1.1;
    let slope_denominator = p2.0 - p1.0;

    let intercept_numerator = 
        slope_denominator * p1.0 - slope_numerator * p1.1;

    if intercept_numerator % slope_denominator != 0 {
        panic!();
    }

    let a = slope_denominator;
    let b = -slope_numerator;
    let c = -intercept_numerator;

    Line {
        a: a,
        b: b,
        c: c,
        min_x: cmp::min(p1.0, p2.0),
        max_x: cmp::max(p1.0, p2.0),
        min_y: cmp::min(p1.1, p2.1),
        max_y: cmp::max(p1.1, p2.1),
    }
}


/* Parsing */
fn parse_sesnor_data(file_name: &str) -> std::io::Result<Vec<((i64, i64), (i64, i64))>> {
    let file = fs::File::open(file_name)
        .expect("Failed to open");
    let reader = BufReader::new(file);
    let mut output = Vec::new();

    for line in reader.lines() {
        output.push(parse_sesnor_beacon_pair(&line?).unwrap());
    }

    Ok(output)
}

fn parse_sesnor_beacon_pair(line: &str) -> Option<((i64, i64), (i64, i64))> {
    let chars_vec = line.chars().collect::<Vec<char>>();
    let mut chars = chars_vec.iter().peekable();

    Some((
        parse_point(&mut chars)?,
        parse_point(&mut chars)?,
    ))
}


fn parse_point<'a, I>(chars: &mut Peekable<I>) -> Option<(i64, i64)> 
where I: Iterator<Item = &'a char> {
    advance_to_first_char_after_equals(chars)?;
    let x = parse_num(chars)?;
    advance_to_first_char_after_equals(chars)?;
    let y = parse_num(chars)?;

    Some((x, y))
}


fn advance_to_first_char_after_equals<'a, I>(chars: &mut Peekable<I>) -> Option<()>
where I: Iterator<Item = &'a char> {
    while let Some(ch) = chars.next() {
        if *ch == '=' {
            return Some(());
        }
    }
    return None;
}

fn parse_num<'a, I>(chars: &mut Peekable<I>) -> Option<i64> 
where I: Iterator<Item = &'a char> {
    let mut output = None;
    let mut output_val = 0;
    let mut sign = 1;

    loop {
        match chars.peek() {
            Some('-') => {
                sign = -1;
                chars.next();
            },
            ch @ Some('0'..='9') => {
                output_val = output_val * 10 + (
                    **ch.unwrap() as i64 - '0' as i64
                );
                output = Some(output_val);
                chars.next();
            },
            _ => break,
        };
    }

    output.map(|v| v * sign)
}


fn length(n: i64, base: u32) -> u32 {
    let mut power = base;
    let mut count = 1;
    let m = n.abs() as u32;

    if n < 0 {
        count += 1;
    }

    while m >= power {
        count += 1;
        if let Some(new_power) = power.checked_mul(base) {
            power = new_power;
        } else {
            break;
        }
    }

    count
}


fn manhattan(p1: (i64, i64), p2: (i64, i64)) -> u32 {
    (p1.0 - p2.0).abs() as u32 + (p1.1 - p2.1).abs() as u32   
}