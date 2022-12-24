use std::fs;
use std::io::BufReader;
use std::io::prelude::*;
use std::collections::HashSet;
use std::collections::HashMap;
use std::collections::VecDeque;

use crate::days::day_23::Heading::*;

#[derive(Clone, Copy)]
enum Heading {
    North,
    East, 
    South,
    West
}

struct Grid {
    elf_positions: HashSet<(i32, i32)>,
    top_left_x: i32,
    top_left_y: i32,
    width: u16,
    height: u16,
}


pub fn run(file_name: &str) -> std::io::Result<()> {
    let mut grid = parse_grid(file_name)?;
    let mut headings = VecDeque::from([North, South, West, East]);
    let mut n = 0;
    let debug = false;

    loop {
        if debug {
            grid.print(1);
        }
        println!("End of round {}\n", n);
        n = n + 1;

        if !grid.make_moves(&headings) {
            break;
        }

        let top = headings.pop_front().unwrap();
        headings.push_back(top);
    }

    let score = (grid.width * grid.height) - grid.elf_positions.len() as u16;

    println!("P1: {}", score);
    println!("P2: {}", n);

    Ok(())
}


fn add_to_position_map(position: (i32, i32), elf: (i32, i32), positions: &mut HashMap<(i32, i32), Vec<(i32, i32)>>) {
    if let Some(elfs) = positions.get_mut(&position) {
        elfs.push(elf);
    } else {
        let mut elfs = Vec::new();
        elfs.push(elf);
        positions.insert(position, elfs);
    }
}


impl Grid {
    fn make_moves(&mut self, headings: &VecDeque<Heading>) -> bool {
        let mut proposed_position = self.get_proposed_moves(headings);

        loop {
            let mut can_continue = true;
            let mut final_positions = HashMap::new();

            for (position, elfs) in &proposed_position {
                if elfs.len() == 1 {
                    add_to_position_map(*position, elfs[0], &mut final_positions);
                    continue;
                }
                can_continue = false;
                
                for elf in elfs {
                    add_to_position_map(*elf, *elf, &mut final_positions);
                }
            }

            proposed_position = final_positions;

            if can_continue {
                break;
            }
        }

        if proposed_position.len() == 0 {
            return false;
        }

        let mut new_elf_positions = HashSet::new();
        let mut changed = false;

        let mut min_x = i32::MAX;
        let mut max_x = i32::MIN;
        let mut min_y = i32::MAX;
        let mut max_y = i32::MIN;

        for (position, elfs) in proposed_position {
            if elfs.len() != 1 {
                panic!();
            }

            if !self.elf_positions.contains(&position) {
                changed = true;
            }
            
            new_elf_positions.insert(position);

            min_x = std::cmp::min(min_x, position.0);
            max_x = std::cmp::max(max_x, position.0);
            min_y = std::cmp::min(min_y, position.1);
            max_y = std::cmp::max(max_y, position.1);
        }

        self.top_left_x = min_x;
        self.top_left_y = min_y;
        self.width = (max_x - min_x) as u16 + 1;
        self.height = (max_y - min_y) as u16 + 1;
        self.elf_positions = new_elf_positions;

        changed
    }


    fn get_proposed_moves(&self, 
        headings: &VecDeque<Heading>
    ) -> HashMap<(i32, i32), Vec<(i32, i32)>> {
        let mut new_positions_to_elfs_map: HashMap<(i32, i32), Vec<(i32, i32)>> = HashMap::new();

        for elf in &self.elf_positions {
            let proposed_position = if self.can_move(elf) {
                self.get_proposed_position(elf, headings.iter())
            } else {
                *elf
            };

            if let Some(elfs) = new_positions_to_elfs_map.get_mut(&proposed_position) {
                elfs.push(*elf);
            } else {
                let mut elfs = Vec::new();
                elfs.push(*elf);
                new_positions_to_elfs_map.insert(proposed_position, elfs);
            }
        }

        new_positions_to_elfs_map
    }

    fn get_proposed_position<'a, I>(&self, current_position: &(i32, i32), headings: I) -> (i32, i32) 
    where I: Iterator<Item = &'a Heading> {
        for heading in headings {
            if let Some(position) = self.heading_free(current_position, &heading) {
                return position;
            }
        }
        *current_position
    }

    fn can_move(&self, current_position: &(i32, i32)) -> bool {
        !(
            self.north_free(current_position).is_some() && 
            self.east_free(current_position).is_some() && 
            self.south_free(current_position).is_some() && 
            self.west_free(current_position).is_some()
        )
    }

    fn heading_free(&self, current_position: &(i32, i32), heading: &Heading) -> Option<(i32, i32)> {
        match heading {
            North => self.north_free(current_position),
            East => self.east_free(current_position),
            South => self.south_free(current_position),
            West => self.west_free(current_position),
        }
    }

    fn north_free(&self, current_position: &(i32, i32)) -> Option<(i32, i32)> {
        let (x, y) = *current_position;

        self.row_free((x - 1, y - 1), (x, y - 1), (x + 1, y - 1))
    }
    
    fn east_free(&self, current_position: &(i32, i32)) -> Option<(i32, i32)> {
        let (x, y) = *current_position;
        
        self.row_free((x + 1, y - 1), (x + 1, y), (x + 1, y + 1))
    }

    fn south_free(&self, current_position: &(i32, i32)) -> Option<(i32, i32)> {
        let (x, y) = *current_position;
        
        self.row_free((x - 1, y + 1), (x, y + 1), (x + 1, y + 1))
    }

    fn west_free(&self, current_position: &(i32, i32)) -> Option<(i32, i32)> {
        let (x, y) = *current_position;
        
        self.row_free((x - 1, y - 1), (x - 1, y), (x - 1, y + 1))
    }

    fn row_free(&self, a: (i32, i32), b: (i32, i32), c: (i32, i32)) -> Option<(i32, i32)> {
        if !self.position_free(&a) || 
           !self.position_free(&b) ||
           !self.position_free(&c) {
            None 
        } else {
            Some(b)
        }
    }

    fn position_free(&self, position: &(i32, i32)) -> bool {
        !self.elf_positions.contains(position)
    }
}


/* Debugging and Parsign */
impl Grid {
    fn print(&self, padding: u16) {
        let min_y = self.top_left_y - padding as i32;
        let max_y = self.top_left_y + self.height as i32 + padding as i32;
        let min_x = self.top_left_x - padding as i32;
        let max_x = self.top_left_x + self.width as i32 + padding as i32;
        

        for y in min_y..max_y {
            for x in min_x..max_x {
                if self.elf_positions.contains(&(x, y)) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }
}

fn parse_grid(file_name: &str) -> std::io::Result<Grid> {
    let file = fs::File::open(file_name)?;
    let reader = BufReader::new(file);
    let mut elf_positions = HashSet::new();
    let mut width = 0;
    let mut height = 0;
    let mut min_x = i32::MAX;
    let mut min_y = i32::MAX;

    for (y, line) in reader.lines().enumerate() {
        let mut line_had_elfs = false;

        for (x, ch) in line?.chars().enumerate() {
            if ch != '#' {
                continue;
            }

            line_had_elfs = true;

            elf_positions.insert((
                x as i32, y as i32
            ));

            if x as u16 >= width {
                width = x as u16 + 1;
            }
            if (x as i32) < min_x {
                min_x = x as i32;
            }
        }

        if line_had_elfs && y as u16 >= height {
            height = y as u16 + 1;
        }
        if line_had_elfs && (y as i32) < min_y {
            min_y = y as i32;
        }
    }

    Ok(Grid {
        elf_positions: elf_positions,
        top_left_x: min_x,
        top_left_y: min_y,
        width: width - min_x as u16,
        height: height - min_y as u16,
    })
}