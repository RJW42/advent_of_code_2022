use std::fs;
use std::io::BufReader;
use std::io::prelude::*;
use std::collections::HashMap;

use crate::d10::Instruction::*;

enum Instruction {
    Nop,
    Add(i32),
}


pub fn run(file_name: &str) -> std::io::Result<()> {
    let file = fs::File::open(file_name)
        .expect("File not found");
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    let mut signal_strength_cycles = HashMap::from([
        (20, 0), (60, 0), (100, 0), (140, 0), (180, 0), (220, 0)
    ]);

    let mut screen = Vec::new();


    let mut current_cycle = 1;
    let mut reg_x = 1;
    let mut current_instr = None;
    let mut cycles_to_complete = 0; 
    let mut prev_cycle = 0;


    loop {
        print!("c {}, x {}: ", current_cycle, reg_x);

        // Update Screen State
        if prev_cycle != current_cycle {
            let sprint_start_pos = reg_x - 1; // reg_x points to middle of sprite
            let pixel_row_pos = (current_cycle - 1) % 40;
            let diff = pixel_row_pos - sprint_start_pos;
            let pixel = if diff >= 0 && diff < 3 {'#'} else {'.'};
            
            screen.push(pixel);
            prev_cycle = current_cycle;
        }


        if let Some(val) = signal_strength_cycles.get_mut(&current_cycle) {
            *val = reg_x;
        }

        // Update CPU State 
        if cycles_to_complete != 0 && !current_instr.is_none() {
            println!("exec");
            current_cycle += 1;
            cycles_to_complete -= 1;
            continue;
        }

        current_instr = match current_instr {
            Some(Add(x)) => {
                /* Complete add instruction */
                reg_x += x;
                println!("finished");
                current_cycle += 1;
                None
            },
            Some(Nop) => {
                /* Complete Noop */
                println!("finished");
                current_cycle += 1;
                None
            },
            None => {
                /* Parse next instruction */
                let line = lines.next();

                if line.is_none() {
                    /* Finished */
                    println!("end");
                    break;
                }
                
                let next = parse_instruction(&line.unwrap()?).unwrap();

                print!("start ");
                next.print();

                cycles_to_complete = match next {
                    Nop => 0,
                    Add(_) => 1,
                };

                Some(next)
            }
        };
    }

    println!("\nX: {}, Cycle: {}", reg_x, current_cycle);

    let mut signal_strength = 0;

    for (cycle, reg) in signal_strength_cycles.iter() {
        println!("c: {}, x: {}", cycle, reg);
        signal_strength += cycle * reg;
    }

    println!("sterngth: {}\n", signal_strength);

    for (i, pixel) in screen.iter().enumerate() {
        if i % 40 == 0 {
            print!("Cycle ");

            if i < 10 { print!("  "); }
            else if i < 100 { print!(" "); }

            print!("{} -> ", i);
        }

        print!("{}", pixel);

        if i % 40 == 39 {
            println!(" <- cycke {}", i);

            if i == 239 {break;}
        }
    }

    println!();

    Ok(())
}



/* Debug */
impl Instruction {
    fn print(&self) {
        match self {
            Nop     => println!("noop"),
            Add(x)  => println!("addx {}", x),
        };
    }
}

/* parsing */
fn parse_instruction(line: &str) -> Option<Instruction> {
    match line.chars().next()? {
        'n' => Some(Nop),
        'a' => parse_add(line),
        _   => None
    }
}


fn parse_add(line: &str) -> Option<Instruction> {
    let mut output = 0;
    let mut sign = 1 as i32;

    for ch in line.chars().skip(5) {
        match ch {
            '0'..='9' => output = output * 10 + (ch as u32 - '0' as u32),
            '-' => sign = -1,
            _   => break,
        }
    }


    Some(Add(output as i32 * sign))
}

