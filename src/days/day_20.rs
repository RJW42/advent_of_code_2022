use std::fs;
use std::io::BufReader;
use std::io::prelude::*;

#[derive(Debug, Default, Clone, Copy)]
struct Node {
    value: i64,
    next: usize,
    prev: usize,
}

impl Node {
    fn new(value: i64) -> Self {
        Self {
            value,
            ..Default::default()
        }
    }
}

pub fn run(file_name: &str) -> std::io::Result<()> {
    let decryption_key = 811589153;
    let numbers = parse_nums(file_name)?;
    let (mut nodes, zero_index) = build_list(&numbers);

    mix(&mut nodes, false);

    println!("p1: {}", calculate_awnser(&nodes, zero_index));

    let (mut nodes, zero_index) = build_list(&numbers);

    p2_mix(&mut nodes, decryption_key, false);

    println!("p2: {}", calculate_awnser(&nodes, zero_index));

    
    Ok(())
}


fn p2_mix(nodes: &mut Vec<Node>, decryption_key: i64, debug: bool ) {
    for index in 0..nodes.len() {
        nodes[index].value *= decryption_key;
    }

    for _ in 0..10 {
        mix(nodes, false);
    }
}

fn mix(nodes: &mut Vec<Node>, debug: bool) {
    for index in 0..nodes.len() {
        if debug {
            print_list(nodes);
        }
        let current = nodes[index];
        let shift = modulo(current.value, nodes.len() - 1);

        if shift == 0 {
            continue;
        }

        // Remove current from list 
        nodes[current.prev].next = current.next;
        nodes[current.next].prev = current.prev;

        // Get the element at the fhist position
        let mut node = current.next;
        for _ in 1..shift {
            node = nodes[node].next;
        }

        let next = nodes[node].next;
        let prev = node;

        // Re insert current into the list 
        nodes[next].prev = index;
        nodes[prev].next = index;

        nodes[index].next = next;
        nodes[index].prev = prev;
    }
    if debug {
        print_list(nodes);
    }
}

fn print_list(nodes: &Vec<Node>) {
    let mut index = 0;
    loop {
        let node = nodes[index];
        print!("{}, ", node.value);
        index = node.next;
        if index == 0 {
            break;
        }
    }
    println!();
}

fn calculate_awnser(nodes: &Vec<Node>, zero_index: usize) -> i64 {
    let mut index = zero_index;
    let mut awnser = 0;

    for iteration in 1..3001 {
        index = nodes[index].next;
        if iteration % 1000 == 0 {
            awnser += nodes[index].value;
        }
    }

    awnser
}


fn modulo(n: i64, modulo: usize) -> usize {
    (((n % modulo as i64) + modulo as i64) % (modulo as i64)) as usize
}

fn build_list(numbers: &Vec<i64>) -> (Vec<Node>, usize) {
    let mut nodes = Vec::with_capacity(numbers.len());
    let mut zero_index = 0;

    for (index, value) in numbers.iter().copied().enumerate() {
        nodes.push(
            Node::new(value)
        );

        if value == 0 {
            zero_index = index;
        }
    }

    for index in 0..nodes.len() {
        nodes[index].next = (index + 1) % numbers.len();
        nodes[index].prev = index.checked_sub(1).unwrap_or(numbers.len() - 1);
    }

    (nodes, zero_index)
}


/* Parsing */
fn parse_nums(file_name: &str) -> std::io::Result<Vec<i64>> {
    let file = fs::File::open(file_name)?;
    let reader = BufReader::new(file);
    let mut output = Vec::new();

    for line in reader.lines() {
        output.push(
            parse_num(&line?).unwrap()
        );
    }

    Ok(output)
}


fn parse_num(line: &str) -> Option<i64>  {
    let mut chars = line.chars().into_iter().peekable();
    let mut output = None;
    let mut output_val = 0;
    let mut sign = 1;

    loop {
        match chars.peek() {
            Some('-') => {
                sign = -1;
                chars.next();
            },
            ch @ Some('0'..='9') => {
                output_val = output_val * 10 + (
                    *ch.unwrap() as i64 - '0' as i64
                );
                output = Some(output_val);
                chars.next();
            },
            _ => break,
        };
    }

    output.map(|v| v * sign)
}