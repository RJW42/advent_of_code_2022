use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;


pub fn run(file_name: &str) -> std::io::Result<()> {
    let file = File::open(file_name)
        .expect("File not found");
    let reader = BufReader::new(file);

    let mut score = 0;

    for l in reader.lines() {
        let line = l?;
        /* Part One
        let opp = line.chars().nth(0).unwrap() as i32 - 'A' as i32;
        let rsp = line.chars().nth(2).unwrap() as i32 - 'X' as i32;

        let res = if opp < rsp { opp - rsp + 3} else {opp - rsp};

        // 0 == draw
        // 1 == loss
        // 2 == win 

        score += rsp + 1; // Add peice bonus
        score += ((res + 1) % 2) * (3 + (res / 2) * 3); // Add win/draw bonus 
        */
        let opp = line.chars().nth(0).unwrap() as i32 - 'A' as i32;
        let res = line.chars().nth(2).unwrap() as i32 - 'X' as i32;

        // 0 == loss
        // 1 == draw
        // 2 == win

        let rsp = (opp + res + 2) % 3; // ((opp + (res - 1) + 3) % 3)

        score += rsp + 1;
        score += 3 * res;
    }

    println!("score: {}", score);

    Ok(())
}