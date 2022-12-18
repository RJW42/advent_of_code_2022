use std::cmp;
use std::fs;
//use std::fmt;
use std::io::BufReader;
use std::io::prelude::*;
use std::iter::Peekable;
use std::collections::HashMap;

use crate::days::day_18::Entity::*;


const CUBE_OFFSETS: [(i8,i8,i8); 6] = [
    (1, 0, 0),
    (0, 1, 0),
    (0, 0, 1),
    (-1, 0, 0),
    (0, -1, 0),
    (0, 0, -1)
];


struct Space {
    entities: Vec<Entity>,
    width: usize,
    height: usize,
    length: usize,
}

#[derive(Clone, Copy)]
enum Entity {
    Air(bool),
    CubeFace(u32, bool),
    Cube,
}


pub fn run(file_name: &str) -> std::io::Result<()> {
    let tuples = parse_tuples(file_name)?;

    part_one(tuples.clone());
    part_two(tuples.clone());

    Ok(())
}

/* Part Two */
fn part_two(tuples: Vec<(u32,u32,u32)>) {
    let mut space = Space::new();

    space.add_cubes(&tuples);
    space.init_outside();

    let mut score = 0;

    for entity in space.entities {
        if let CubeFace(cons, true) = entity {
            score += cons;
        }
    }

    println!("p2: {}", score);
}

impl Space {
    fn new() -> Space {
        Space {
            entities: Vec::new(),
            width: 0,
            height: 0,
            length: 0,
        }
    }

    fn add_cubes(&mut self, tuples: &Vec<(u32,u32,u32)>) {
        let entities_map = self.init_intities(tuples);
        
        self.entities = vec![Air(false); self.width * self.height * self.length];

        for ((x, y, z), entity) in entities_map {
            let index = self.get_index(x, y, z);

            self.entities[index] = entity;
        }
    }


    fn init_outside(&mut self) {
        self.entities[0] = Air(true);

        let mut visisted = vec![false; self.entities.len()];

        self.dfs(0, 0, 0, &mut visisted);
    }


    fn dfs(&mut self, x: u32, y: u32, z: u32, visisted: &mut Vec<bool>) {
        let index = self.get_index(x, y, z);

        if visisted[index] {
            return;
        }

        visisted[index] = true;

        if let Cube = self.entities[index] {
            return;
        }

        self.entities[index] = match self.entities[index] {
            Air(false) => Air(true),
            CubeFace(cons, false) => CubeFace(cons, true),
            other => other
        };

        for (x_offset, y_offset, z_offset) in CUBE_OFFSETS {
            let x = x as i32 + x_offset as i32;
            let y = y as i32 + y_offset as i32;
            let z = z as i32 + z_offset as i32;

            if x < 0 || y < 0 || z < 0 || x >= self.width as i32 || y >= self.height as i32 || z >= self.length as i32 {
                continue;
            }

            self.dfs(x as u32, y as u32, z as u32, visisted);
        }
    }


    fn get_index(&self, x: u32, y: u32, z: u32) -> usize {
        (z as usize * self.width * self.height) + (y as usize * self.width) + x as usize
    }


    // fn get_position(&self, index: usize) -> (u32, u32, u32) {
    //     let z = index / (self.width * self.height);
    //     let index = index - (z * (self.width * self.height));
    //     let y = index / self.width;
    //     let x = index % self.width;

    //     (x as u32, y as u32, z as u32)
    // }


    fn init_intities(&mut self, tuples: &Vec<(u32,u32,u32)>) -> HashMap<(u32, u32, u32), Entity> {
        let mut entities = HashMap::new();

        for (x, y, z) in tuples {
            self.add_cube_to_map(&mut entities, *x, *y, *z);
        }

        entities
    }


    fn add_cube_to_map(&mut self, entities: &mut HashMap<(u32, u32, u32), Entity>, x: u32, y: u32, z: u32) {
        entities.insert((x, y, z), Cube);

        for (x_offset, y_offset, z_offset) in CUBE_OFFSETS {
            let x = (x as i32 + x_offset as i32) as u32;
            let y = (y as i32 + y_offset as i32) as u32;
            let z = (z as i32 + z_offset as i32) as u32;

            self.width = cmp::max(self.width, (x + 1).try_into().unwrap());
            self.height = cmp::max(self.height, (y + 1).try_into().unwrap());
            self.length = cmp::max(self.length, (z + 1).try_into().unwrap());

            let pos = (x, y, z);

            match entities.get(&pos) {
                None => { 
                    entities.insert(pos, CubeFace(1, false)); 
                },
                Some(CubeFace(connections, is_outside)) => {
                    entities.insert(pos, CubeFace(
                        connections + 1, *is_outside
                    ));
                },
                _ => (),
            };
        }
    }
}



/* Part One */
fn part_one(tuples: Vec<(u32,u32,u32)>) {
    let mut entities = HashMap::new();

    for (x,y,z) in tuples {
        entities.insert((x,y,z), Cube);

        for (x_offset, y_offset, z_offset) in CUBE_OFFSETS {
            let x = (x as i32 + x_offset as i32) as u32;
            let y = (y as i32 + y_offset as i32) as u32;
            let z = (z as i32 + z_offset as i32) as u32;

            let pos = (x, y, z);

            if !entities.contains_key(&pos) {
                entities.insert(pos, CubeFace(1, true));
            } else if let Some(CubeFace(connected_cubes, _)) = entities.get(&pos) {
                entities.insert(pos, CubeFace(connected_cubes + 1, true));
            }
        }
    }

    let mut score = 0;

    for (_, v) in entities {
        if let CubeFace(count, _) = v {
            score += count;
        }
    }

    println!("p1: {}", score);
}



/* Parsing */

fn parse_tuples(file_name: &str) -> std::io::Result<Vec<(u32,u32,u32)>> {
    let file = fs::File::open(file_name)?;
    let reader = BufReader::new(file);
    let mut output = Vec::new();

    for line in reader.lines() {
        output.push(
            parse_tuple(&line?).unwrap()
        );
    }

    Ok(output)
}

fn parse_tuple(line: &str) -> Option<(u32, u32, u32)> {
    let mut chars = line.chars().into_iter().peekable();

    Some((
        parse_num(&mut chars)?,
        parse_num(&mut chars)?,
        parse_num(&mut chars)?,
    ))
}

fn parse_num<I>(chars: &mut Peekable<I>) -> Option<u32> 
where I: Iterator<Item = char> {
    let mut output = None;
    let mut output_val = 0;

    loop {
        match chars.peek() {
            ch @ Some('0'..='9') => {
                output_val = output_val * 10 + (
                    *ch.unwrap() as u32 - '0' as u32
                );
                output = Some(output_val);
                chars.next();
            },
            Some(',') => { chars.next(); break; },
            _ => break,
        };
    }

    output
}