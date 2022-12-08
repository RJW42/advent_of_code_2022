use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;


struct AssignmentPair {
    elf1: (u32, u32),
    elf2: (u32, u32)
}


pub fn run(file_name: &str) -> std::io::Result<()> {
    let file = File::open(file_name)
        .expect("File not found");
    let reader = BufReader::new(file);

    let mut pairs_that_fully_contain = 0;

    for l in reader.lines() {
        let line = l?;
        let assigment = parse_assigment_pair(&line).unwrap();

        // Part One 
        // if (assigment.elf1.0 <= assigment.elf2.0 && assigment.elf1.1 >= assigment.elf2.1) || 
        //    (assigment.elf2.0 <= assigment.elf1.0 && assigment.elf2.1 >= assigment.elf1.1) {
        //     pairs_that_fully_contain += 1;
        // }

        if (assigment.elf1.0 <= assigment.elf2.0 && assigment.elf1.1 >= assigment.elf2.0) || 
           (assigment.elf2.0 <= assigment.elf1.0 && assigment.elf2.1 >= assigment.elf1.0) {
            pairs_that_fully_contain += 1;
        }

        println!(
            "{},{}-{},{} => {}",
            assigment.elf1.0,
            assigment.elf1.1,
            assigment.elf2.0,
            assigment.elf2.1,
            pairs_that_fully_contain
        );
    }

    println!("Res: {}", pairs_that_fully_contain);

    Ok(())
}


fn parse_assigment_pair(line: &str) -> Result<AssignmentPair, &'static str> {
    let (elf1, mid_pos) = parse_assigment(&line, 0).unwrap();
    let (elf2, _) = parse_assigment(&line, mid_pos + 1).unwrap();

    Ok(AssignmentPair {
        elf1: elf1,
        elf2: elf2
    })
}


fn parse_assigment(line: &str, start_pos: u32) -> Result<((u32, u32), u32), &'static str> {
    let (start, mid_pos) = parse_number(&line, start_pos).unwrap();
    // Check if char at mid_pos == ,
    let (end, end_pos) = parse_number(&line, mid_pos + 1).unwrap();

    Ok(((start, end), end_pos))
}

fn parse_number(line: &str, start_pos: u32) -> Result<(u32, u32), &'static str> {
    let mut res = 0;
    let mut end_pos = start_pos;

    for ch in line.chars().skip(start_pos.try_into().unwrap()) {
        match ch {
            '0'..='9' => res = (res * 10) + (ch as u32) - ('0' as u32),
            _ => break,
        };
        end_pos += 1;
    }


    Ok((res, end_pos))
}