use std::fs;
use std::fmt;
use std::io::BufReader;
use std::io::prelude::*;
use std::collections::HashSet;
use std::collections::HashMap;

use crate::days::day_17::JetDirection::*;
use crate::days::day_17::RockType::*;

struct Chamber {
    falling_rocks: HashSet<(u64, u64)>, 
    stationery_rocks: HashSet<(u64, u64)>,
    height: u64,
    prune_height: u64,
}

#[derive(Copy, Clone)]
enum JetDirection {
    Left,
    Right,
}

struct RockTypeIterator {
    position_in_enum: usize,
}

struct JetIterator {
    position_in_jets: usize,
    jets: Vec<JetDirection>,
}

#[derive(Copy, Clone)]
enum RockType {
    Horizontal,
    Vertical,
    Plus,
    Corner,
    Square,
}


pub fn run(file_name: &str) -> std::io::Result<()> {
    let directions = parse_jet_directions(file_name)?;

    part_one(&directions, 1000000000000, false, true, true);

    Ok(())
}

fn part_one(jets: &Vec<JetDirection>, max_rocks: usize, debug: bool, prune: bool, memo: bool) {
    let mut old_height_memoisiation = HashMap::new();
    let mut delta_memoisiation = HashMap::new();
    let mut loopable = HashSet::new();
    let mut chamber = Chamber::new();
    let mut jets_iter = JetIterator::new(jets).into_iter();
    let mut jet_index = 0;
    let mut rock_num_offset = 0;

    for (rock_num, (rock, rock_index)) in RockType::value_iter().into_iter().enumerate() {
        let index = (rock_index.clone(), jet_index.clone());

        if memo && delta_memoisiation.contains_key(&index) && loopable.contains(&index) {
            let (height_inc, num_inc) = delta_memoisiation.get(&index).unwrap();
            let number_of_increases = (max_rocks as u64 - rock_num_offset as u64 - rock_num as u64) / *num_inc as u64;

            rock_num_offset += *num_inc * number_of_increases as usize;
            chamber.prune_height += *height_inc as u64 * number_of_increases;

            println!("{}", number_of_increases);
        }


        if rock_num + rock_num_offset >= max_rocks {
            break;
        }

        if (rock_num + rock_num_offset) % 4096 == 0 {
            println!("Rock: {}", rock_num);
        }

        chamber.add_new_rock(rock);
        let next_jet_index = (chamber.simulate_rock_fall(&mut jets_iter, prune) + 1) % jets.len();

        if debug {
            chamber.print();
        }


        // Memoisation
        let height = chamber.height + chamber.prune_height;

        if old_height_memoisiation.contains_key(&index) {
            let (old_height, old_rock_num) = old_height_memoisiation.get(&index).unwrap();
            let (height_delta, num_delta) = (height - old_height, rock_num - old_rock_num);

            if delta_memoisiation.contains_key(&index) {
                let (old_height_delta, old_num_delta) = delta_memoisiation.get(&index).unwrap();
                if height_delta == *old_height_delta && num_delta == *old_num_delta {
                    loopable.insert(index);
                }
            } else {
                delta_memoisiation.insert(index, (height_delta, num_delta));
            }
        }
        
        old_height_memoisiation.insert(index, (height, rock_num));

        jet_index = next_jet_index;
    }

    println!("max height: {}", chamber.height + chamber.prune_height);
}


impl Chamber {
    fn simulate_rock_fall(&mut self, jets: &mut dyn Iterator<Item = (JetDirection, usize)>, prune: bool) -> usize {
        let mut rocks: Vec<(u64, u64)> = self.falling_rocks
            .iter().map(|p| *p).collect();

        let mut last_jet_index = 0 as usize;

        while let Some((jet, ji)) = jets.next() {
            // Apply jet to rocks 
            let x_offset = match jet {
                Left => -1,
                Right => 1,
            };

            last_jet_index = ji;

            if self.can_update_rock_positions(&rocks, x_offset, 0) {
                self.update_rock_positions(&mut rocks, x_offset, 0);
            }

            if !self.can_update_rock_positions(&rocks, 0, -1) {
                break;
            } else {
                self.update_rock_positions(&mut rocks, 0, -1);
            }
        }

        for rock in &rocks {
            if rock.1 >= self.height {
                self.height = rock.1 + 1;
            }

            self.stationery_rocks.insert(*rock);
        }

        self.falling_rocks.clear();

        if prune {
            self.attempt_to_prune(&rocks);
        }

        last_jet_index
    }

    fn attempt_to_prune(&mut self, added_rocks: &Vec<(u64, u64)>) {
        let mut seen_y = HashSet::new();

        for rock in added_rocks {
            if seen_y.contains(&rock.1) {
                continue;
            }

            seen_y.insert(rock.1);

            if self.is_prunable_below_y(rock.1) {
                self.prune_below(rock.1);
                break;
            } else if rock.1 > 1 && self.is_prunable_below_y(rock.1 - 1) {
                self.prune_below(rock.1 - 1);
                break;
            }
        }
    }

    fn prune_below(&mut self, new_lower_y: u64) {
        let mut new_rocks = HashSet::new();

        for (x, y) in self.stationery_rocks.iter() {
            if *y < new_lower_y {
                continue;
            }
            new_rocks.insert((*x, *y - new_lower_y));
        }

        self.stationery_rocks = new_rocks;
        self.prune_height += new_lower_y;
        self.height -= new_lower_y;
    }

    fn is_prunable_below_y(&self, new_lower_y: u64) -> bool {
        for x in 0..7 {
            let y1 = new_lower_y;
            let y2 = new_lower_y + 1;

            if !self.stationery_rocks.contains(&(x, y1)) && 
               !self.stationery_rocks.contains(&(x, y2)) {
                return false;
            }
        }

        true
    }

    fn add_new_rock(&mut self, rock: RockType) {
        for (x, y) in rock.get_positions().iter() {
            let x = x + 2;
            let y = self.height + 3 + y;

            self.falling_rocks.insert((x, y));
        }
    }

    fn can_update_rock_positions(
        &self, rocks: &Vec<(u64, u64)>, x_offset: i64, y_offset: i64
    ) -> bool {
        for (rock_x, rock_y) in rocks {
            let new_x = (*rock_x as i64) + x_offset;
            let new_y = (*rock_y as i64) + y_offset;

            if new_x < 0 || new_y < 0 || new_x >= 7 {
                return false;
            }

            let new_x = new_x as u64;
            let new_y = new_y as u64;

            if self.stationery_rocks.contains(&(new_x, new_y)) {
                return false;
            }
        }
        return true;
    }

    fn update_rock_positions(
        &self, rocks: &mut Vec<(u64, u64)>, x_offset: i64, y_offset: i64
    ) {
        for i in 0..rocks.len() {
            rocks[i].0 = (rocks[i].0 as i64 + x_offset) as u64;
            rocks[i].1 = (rocks[i].1 as i64 + y_offset) as u64;
        }
    }
}


impl RockType {
    fn get_positions(&self) -> Vec<(u64, u64)> {
        match self {
            Horizontal => vec![(0, 0), (1, 0), (2, 0), (3, 0)],
            Vertical => vec![(0, 0), (0, 1), (0, 2), (0, 3)],
            Plus => vec![(0, 1), (1, 0), (1, 1), (1, 2), (2, 1)],
            Corner => vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)],
            Square => vec![(0, 0), (1, 0), (0, 1), (1, 1)],
        }
    }

    fn value_iter() -> RockTypeIterator {
        RockTypeIterator {
            position_in_enum: 5,
        }
    }
}

/* Debug */

impl fmt::Display for JetDirection {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{}", 
            match self {
                Left => '<',
                Right => '>'
            }
        )
    }
}

impl Chamber {
    fn new() -> Chamber {
        Chamber {
            falling_rocks: HashSet::new(),
            stationery_rocks: HashSet::new(),
            height: 0,
            prune_height: 0,
        }
    }

    fn print(&self) {
        // Print content 
        for y in (0..(self.height + 8)).rev() {
            print!("|");
            for x in 0..7 {
                if self.stationery_rocks.contains(&(x, y)) {
                    print!("#");
                } else if self.falling_rocks.contains(&(x, y)) {
                    print!("@");
                } else {
                    print!(".");
                }
            }
            println!("|");
        }
        
        // print bottom 
        println!("+-------+");
    }
}


impl Iterator for RockTypeIterator {
    type Item = (RockType, usize);

    fn next(&mut self) -> Option<(RockType, usize)> {
        static TYPES: [RockType; 5] = [
            Horizontal, Plus, Corner, Vertical, Square
        ];

        self.position_in_enum += 1;

        if self.position_in_enum >= 5 {
            self.position_in_enum = 0;
        } 

        Some((TYPES[self.position_in_enum], self.position_in_enum))
    }
}


impl Iterator for JetIterator {
    type Item = (JetDirection, usize);

    fn next(&mut self) -> Option<(JetDirection, usize)> {
        self.position_in_jets += 1;

        if self.position_in_jets >= self.jets.len() {
            self.position_in_jets = 0;
        } 

        Some((self.jets[self.position_in_jets], self.position_in_jets))
    }
}

impl JetIterator {
    fn new(jets: &Vec<JetDirection>) -> JetIterator {
        JetIterator {
            jets: jets.clone(),
            position_in_jets: jets.len() - 1,
        }
    }
}

/* Parsing */

fn parse_jet_directions(file_name: &str) -> std::io::Result<Vec<JetDirection>>{
    let file = fs::File::open(file_name)?;
    let mut reader = BufReader::new(file);
    let mut line = String::new();

    reader.read_line(&mut line)?;

    let mut output = Vec::new();

    for ch in line.chars() {
        match ch {
            '<' => output.push(Left),
            '>' => output.push(Right),
            _ => ()
        };
    }

    print!("Jet Directions: ");
    for dir in &output {
        print!("{}", dir);
    }
    println!();

    Ok(output)
}   