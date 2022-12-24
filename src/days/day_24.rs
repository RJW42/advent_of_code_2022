use std::fs;
use std::cmp::Ordering;
use std::io::BufReader;
use std::io::prelude::*;
use std::collections::HashMap;
use std::collections::BinaryHeap;

use crate::days::day_24::Direction::*;

#[derive(Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West
}

struct Grid {
    elements: Vec<Vec<Vec<Direction>>>,
    start_pos: (u32, u32),
    end_pos: (u32, u32),
    width: usize,
    height: usize,
    elements_width: usize,
    elements_height: usize,
}

pub fn run(file_name: &str) -> std::io::Result<()> {
    let mut grid = parse_grid(file_name)?;

    let start_pos = grid.start_pos;
    let end_pos = grid.end_pos;

    grid.print(0);

    let p1_score = perform_walk(
        &grid, grid.start_pos.0, grid.start_pos.1, 0
    );

    println!("p1: {}", p1_score);

    grid.start_pos = end_pos;
    grid.end_pos = start_pos;

    let back_to_start_t = perform_walk(
        &grid, grid.start_pos.0, grid.start_pos.1, p1_score
    );

    println!("bts: {}", back_to_start_t);

    grid.start_pos = start_pos;
    grid.end_pos = end_pos;

    let back_to_end_t = perform_walk(
        &grid, grid.start_pos.0, grid.start_pos.1, back_to_start_t
    );

    println!("p2: {}", back_to_end_t);
    
    Ok(())
}


#[derive(Eq)]
struct WalkTime {
    x: u32,
    y: u32,
    time: u32
}

impl Ord for WalkTime {
    fn cmp(&self, other: &Self) -> Ordering {
        other.time.cmp(&self.time)
    }
}

impl PartialOrd for WalkTime {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl PartialEq for WalkTime {
    fn eq(&self, other: &Self) -> bool {
        self.time == other.time
    }
}


fn perform_walk(grid: &Grid, x: u32, y: u32, start_time: u32) -> u32  {
    let mut times_heap: BinaryHeap<WalkTime> = BinaryHeap::new();
    let mut times: HashMap<(u32, u32, usize), u32> = HashMap::new();
    
    times.insert((x, y, start_time as usize % grid.elements.len()), start_time);
    times_heap.push(WalkTime {
        x: x,
        y: y,
        time: start_time
    });

    while let Some(walktime) = times_heap.pop() {
        let x = walktime.x;
        let y = walktime.y;
        let time = walktime.time;

        for (new_x, new_y, new_time) in get_possible_moves(grid, x, y, time) {
            let new_time_index = new_time as usize % grid.elements.len();

            if let Some(old_new_time) = times.get(&(new_x, new_y, new_time_index)) {
                if *old_new_time <= new_time {
                    continue;
                }
            }

            if new_x == grid.end_pos.0 && new_y == grid.end_pos.1 {
                return new_time;
            }

            times.insert((new_x, new_y, new_time_index), new_time);
            times_heap.push(WalkTime {
                x: new_x,
                y: new_y,
                time: new_time
            });
        }
    }

    panic!();
}


fn get_possible_moves(
        grid: &Grid, x: u32, y: u32, time: u32
    ) -> Vec<(u32, u32, u32)> {
    let next_time = time + 1;
    let time_index = time as usize % grid.elements.len();
    let next_time_index = (time_index + 1) % grid.elements.len();

    let mut moves = Vec::new();

    // Check all move directions
    static OFFSETS: [(i32, i32); 5] = [(0, 1), (0, -1), (-1, 0), (1, 0), (0, 0)];

    for offset in OFFSETS.iter() {
        let new_x = x as i32 + offset.0;
        let new_y = y as i32 + offset.1;

        if new_x == grid.end_pos.0 as i32 && new_y == grid.end_pos.1 as i32 {
            moves.push((grid.end_pos.0, grid.end_pos.1, next_time));
            continue;
        }

        if new_x == grid.start_pos.0 as i32 && new_y == grid.start_pos.1 as i32 {
            moves.push((grid.start_pos.0, grid.start_pos.1, next_time));
            continue;
        }

        if new_x < 1 || new_y < 1 || 
           new_x > grid.elements_width as i32 || 
           new_y > grid.elements_height as i32 {
            continue;
        }

        if grid.elements[next_time_index][
            grid.get_index(new_x as u32, new_y as u32).unwrap()
        ].len() != 0 {
            continue;
        }

        moves.push((
            new_x as u32, new_y as u32, next_time
        ));
    }

    moves
}



/* Functionality */
impl Direction {
    fn get_new_position(&self, grid: &Grid, x: u32, y: u32) -> (u32, u32) {
        match self {
            North => if y != 1 { (x, y - 1) } else { (x, grid.elements_height as u32) },
            East => if x != grid.elements_width as u32 { (x + 1, y) } else { (1, y) },
            South => if y != grid.elements_height as u32 { (x, y + 1) } else { (x, 1) },
            West => if x != 1 { (x - 1, y) } else { (grid.elements_width as u32, y) },
        }
    }
}


/* Debugging and Parsing */
impl Grid {
    fn print(&self, time: usize) {
        let time = time % self.elements.len();

        for i in 0..self.width {
            print!("{}", if i == 1 { '.'} else {'#'} );
        }
        println!();

        for y in 1..(self.height - 1) {
            print!("#");
            for x in 1..(self.width - 1) {
                let element = &self.elements[time][
                    self.get_index(x as u32, y as u32).unwrap()
                ];

                print!("{}", match element.len() {
                    0 => '.',
                    1 => element[0].to_char(),
                    _ => char::from_u32('0' as u32 + element.len() as u32).unwrap(),
                });
            }
            println!("#");
        }

        for i in 0..self.width {
            print!("{}", if i == self.width - 2 { '.'} else {'#'} );
        }
        println!();
    }

    fn get_index(&self, x: u32, y: u32) -> Option<usize> {
        if x == 0 || x == self.width as u32 - 1 ||
           y == 0 || y == self.height as u32 - 1 {
            return None;
        }

        let x = (x - 1) as usize;
        let y = (y - 1) as usize;

        Some((y * self.elements_width) + x)
    }
}


impl Direction {
    fn to_char(&self) -> char {
        match self {
            North => '^',
            East => '>',
            South => 'v',
            West => '<',
        }
    }
}


fn parse_grid(file_name: &str) -> std::io::Result<Grid> {
    let file = fs::File::open(file_name)?;
    let reader = BufReader::new(file);

    let mut grid = Grid {
        elements: Vec::new(),
        start_pos: (1, 0),
        end_pos: (0 ,0),
        width: 0,
        height: 0,
        elements_width: 0,
        elements_height: 0,
    };

    grid.elements.push(Vec::new());

    for (y, l) in reader.lines().enumerate() {
        let line = l?;

        grid.height += 1;

        if y == 0 {
            grid.width = line.len();
            continue;
        }

        parse_grid_line(&line, &mut grid);
    }

    grid.elements_width = grid.width - 2;
    grid.elements_height = grid.height - 2;

    grid.end_pos.0 = grid.elements_width as u32;
    grid.end_pos.1 = grid.elements_height as u32 + 1;

    for time in 1..(grid.elements_width*grid.elements_height) {
        let new_elements = advance_blizards(&grid, time - 1);

        grid.elements.push(
            new_elements
        );
    }

    Ok(grid)
}


fn advance_blizards(grid: &Grid, time: usize) -> Vec<Vec<Direction>> {
    // Init new elements list
    let mut new_elements = Vec::new();

    for _ in 0..(grid.elements_width * grid.elements_height) {
        new_elements.push(Vec::new());
    }

    // Advance blizards 
    for x in 1..(grid.width - 1) {
        for y in 1..(grid.height - 1) {
            let x = x as u32;
            let y = y as u32;
            let index = grid.get_index(x, y).unwrap();

            for direction in &grid.elements[time][index] {
                let (new_x, new_y) = direction.get_new_position(
                    grid, x, y
                );

                let new_index = grid.get_index(new_x, new_y).unwrap();

                new_elements[new_index].push(*direction);
            }
        }
    }

    new_elements
}


fn parse_grid_line(line: &str, grid: &mut Grid) {
    for ch in line.chars().skip(1) {
        match ch {
            '#' => break,
            '.' => grid.elements[0].push(vec![]),
            '^' => grid.elements[0].push(vec![North]),
            '<' => grid.elements[0].push(vec![West]),
            '>' => grid.elements[0].push(vec![East]),
            'v' => grid.elements[0].push(vec![South]),
            _ => panic!(),
        }
    }
}