use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

use bit_vec::BitVec;



pub fn run(file_name: &str) -> std::io::Result<()> {
    let file = File::open(file_name)
        .expect("File not found");
    let reader = BufReader::new(file);

    let mut score = 0;
    let mut group_rucksack = BitVec::from_elem(52, false);

    for (i, l) in reader.lines().enumerate() {
        let line = l?;
        let mut line_ruckstack = BitVec::from_elem(52, false);
        
        for c in line.chars() {
            let present = match c {
                'a' ..= 'z' => c as u32 - 'a' as u32,
                'A' ..= 'Z' => c as u32 - 'A' as u32 + 26,
                _ => 0,
            };

            line_ruckstack.set(present.try_into().unwrap(), true);
        }   

        if i % 3 == 0 {
            // Building new group 
            group_rucksack.clear();
            group_rucksack.or(&line_ruckstack);
            continue;
        }

        // Removing elements from group
        group_rucksack.and(&line_ruckstack);

        if i % 3 == 1 {
            // Need to do one more line 
            continue;
        }

        // Find remaning element
        for (i, taken) in group_rucksack.iter().enumerate() {
            if !taken {
                continue;
            }

            println!("Found: {}", i);
            score += i + 1;
        }

        println!("score: {}", score);
    }

    Ok(())
}   