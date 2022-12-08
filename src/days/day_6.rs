use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

use std::collections::VecDeque;
use std::collections::HashSet;

pub fn run(file_name: &str) -> std::io::Result<()> {
    let file = File::open(file_name)
        .expect("File not found");
    let reader = BufReader::new(file);
    let line = reader.lines().next().unwrap()?;

    let mut packet = VecDeque::new();

    let start_of_packet_size = 4;
    let start_of_message_size = 14;
    let is_part_one = false;

    let (skip_amount, offset) = if is_part_one {
        (start_of_packet_size - 1, start_of_packet_size)
    } else {
        (start_of_message_size - 1, start_of_message_size)
    };

    line.chars()    
        .into_iter()
        .take(skip_amount)
        .for_each(|x| packet.push_front(x as u32 - 'a' as u32));

    'outer: for (i, ch) in line.chars().skip(skip_amount).enumerate() {
        packet.push_front(ch as u32 - 'a' as u32);

        // Check for duplicates
        let mut set = HashSet::new();

        for v in &packet {
            if set.contains(v) {
                // Contains duplicates drop last element and continue
                packet.pop_back();
                continue 'outer;
            }
            set.insert(v);
        }

        // Set contains no duplicates
        println!("{}", i + offset);
        break;
        
    }


    Ok(())
}