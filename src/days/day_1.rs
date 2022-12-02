use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use priority_queue::PriorityQueue;
use std::cmp::Reverse;

pub fn run(file_name: &str) -> std::io::Result<()> {
    let file = File::open(file_name)
        .expect("File not found");
    let reader = BufReader::new(file);

    let mut maxs: PriorityQueue<i32, Reverse<i32>> = PriorityQueue::new();
    let mut curr = 0;

    for l in reader.lines() {
        let line = l?;

        if line.len() == 0 {
            maxs.push(curr, Reverse(curr));
            if maxs.len() > 3 {
                maxs.pop();
            }
            curr = 0;
            continue;
        }

        curr += line.parse::<i32>().unwrap();
    }

    let mut sum = 0;

    for (item, _) in maxs.into_sorted_iter() {
        println!("{}", item);
        sum += item;
    }

    println!("Sum: {}", sum);

    Ok(())
}