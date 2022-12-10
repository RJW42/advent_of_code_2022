use std::fs;
use std::io::BufReader;
use std::io::prelude::*;
use std::collections::HashSet;

use crate::d9::MovementDirection::*;


struct Movement {
    direction: MovementDirection,
    amount: u32,
}

enum MovementDirection {
    Right,
    Left,
    Up,
    Down,
}


pub fn run(file_name: &str) -> std::io::Result<()> {
    let file = fs::File::open(file_name)
        .expect("File not found");
    let reader = BufReader::new(file);
    
    let rope_length = 10 as usize;
    let mut rope_positions = Vec::<(i32, i32)>::new();

    for _ in 0..rope_length {
        rope_positions.push((0, 0));
    }

    let mut visited_tail_pos = HashSet::<(i32, i32)>::new();

    visited_tail_pos.insert(rope_positions[rope_length - 1]);

    for l in reader.lines() {
        let movement = parse_movement(&l?).unwrap();

        movement.print();

        for _ in 0..movement.amount {
            update_head_pos(&movement.direction, &mut rope_positions[0]);
            print!("h:({}, {})", rope_positions[0].0, rope_positions[0].1);
            
            for i in 1..rope_length {
                let prev = rope_positions[i - 1];
                update_tail_pos(&prev, &mut rope_positions[i]);

                print!(" - t{}({}, {})", i, rope_positions[i].0, rope_positions[i].1);
            }  

            println!();
            visited_tail_pos.insert(rope_positions[rope_length - 1]);
        }
    }

    println!("solution: {}", visited_tail_pos.len());

    Ok(())
}

fn update_head_pos(dir: &MovementDirection, head_pos: &mut (i32, i32)) {
    match dir {
        Right => head_pos.0 += 1,
        Left  => head_pos.0 -= 1,
        Up    => head_pos.1 += 1,
        Down  => head_pos.1 -= 1,
    };
}

fn update_tail_pos(head_pos: &(i32, i32), tail_pos: &mut (i32, i32)) {
    if (head_pos.0 - tail_pos.0).abs() <= 1 && 
       (head_pos.1 - tail_pos.1).abs() <= 1 {
        return;
    }

    fn get_modified(h: i32, t: i32) -> i32 {
        if h < t { -1 }
        else if h > t { 1 }
        else { 0 }
    }

    tail_pos.0 += get_modified(head_pos.0, tail_pos.0);
    tail_pos.1 += get_modified(head_pos.1, tail_pos.1);
}



/* Debug */
impl Movement {
    fn print(&self) {
        print!("== ");

        match self.direction {
            Right => print!("R"),
            Left  => print!("L"),
            Up    => print!("U"),
            Down  => print!("D"),
        }

        println!(" {} ==", self.amount);
    }
}


/* Parsing */
fn parse_movement(line: &str) -> Option<Movement> {
    Some(Movement {
        direction: parse_movement_dir(line)?,
        amount: parse_movement_amount(line),
    })
}


fn parse_movement_dir(line: &str) -> Option<MovementDirection> {
    match line.chars().next() {
        Some('R') => Some(Right),
        Some('L') => Some(Left),
        Some('U') => Some(Up),
        Some('D') => Some(Down),
        _ => None
    }
}

fn parse_movement_amount(line: &str) -> u32 {
    let mut output = 0;

    for ch in line.chars().skip(2) {
        if !('0'..='9').contains(&ch) {
            break;
        }

        output = (output * 10) +  (ch as u32 - '0' as u32);
    }

    output
}