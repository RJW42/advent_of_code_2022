use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

struct SupplyStacks {
    no_of_stacks: usize,
    max_stack_height: usize,
    store: Vec<Vec<char>>,
}


pub fn run(file_name: &str) -> std::io::Result<()> {
    let file = File::open(file_name)
        .expect("File not found");
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    let mut stacks = parse_supply_stacks(&mut lines)?;

    stacks.print();

    for l in lines {
        let line = l?;
        let (amount, from, to) = parse_command(&line);

        let mut tmp = Vec::new();

        for _ in 0..amount {
            let val = stacks.store[from as usize].pop().unwrap();
            tmp.push(val);
        }

        for _ in 0..amount {
            let val = tmp.pop().unwrap();
            stacks.store[to as usize].push(val);
        }

        stacks.max_stack_height = 0;

        for i in 0..stacks.no_of_stacks {
            let len = stacks.store[i].len();
            if len > stacks.max_stack_height {
                stacks.max_stack_height = len;
            }
        }

        stacks.print();
    }

    for i in 0..stacks.no_of_stacks {
        print!("{}", stacks.store[i].last().unwrap())
    }
    println!();

    Ok(())
}


fn parse_command(line: &str) -> (u32, u32, u32) {
    let mut chars = line.chars();

    advance_to_next_space(&mut chars);
    let amount = get_number(&mut chars).unwrap();
    advance_to_next_space(&mut chars);
    let from = get_number(&mut chars).unwrap();
    advance_to_next_space(&mut chars);
    let to = get_number(&mut chars).unwrap();

    (amount, from - 1, to - 1)
}


fn advance_to_next_space(chars: &mut std::str::Chars) {
    while let Some(ch) = chars.next() {
        if ch == ' ' {
            return;
        }
    }
}


fn get_number(chars: &mut std::str::Chars) -> Result<u32, &'static str> {
    let mut res = 0;

    while let Some(ch) = chars.next() {
        match ch {
            '0'..='9' => res = (res * 10) + (ch as u32 - '0' as u32),
            _ => break,
        }
    }

    Ok(res)
}


fn parse_supply_stacks<B: std::io::BufRead>(
    lines: &mut std::io::Lines<B>
) -> std::io::Result<SupplyStacks> {
    // Read first line to get number of stacks
    let mut line = lines.next().unwrap()?;
    let no_of_stacks = (line.len() + 1) / 4;

    // Parse each line into the stacks
    let mut max_stack_height = 0;
    let mut store = Vec::new();

    for _ in 0..no_of_stacks {
        store.push(Vec::new());
    }

    'outer: loop {
        // Parse this line
        let mut column = 0;

        let mut chars = line.chars();
        while let Some(ch) = chars.next() {
            match ch {
                ' ' => match chars.next() {
                    Some('1'..='9') => {
                        // Reached end of header
                        lines.next();
                        break 'outer;
                    },
                    _ => {
                        // Spacing between stack elements
                        column += 1;
                        chars.next();
                        chars.next();
                    }
                },
                '[' => {
                    // Parse this sack element
                    let val = chars.next().unwrap();
                    store[column].push(val);
                    column += 1;
                    chars.next();
                    chars.next();
                },
                _ => (),
            }
        }        


        // Go to next line
        max_stack_height += 1;
        line = lines.next().unwrap()?;
    }


    for i in 0..no_of_stacks {
        store[i].reverse();
    }

    Ok(SupplyStacks {
        no_of_stacks: no_of_stacks, 
        max_stack_height: max_stack_height, 
        store: store
    })
}


impl SupplyStacks {
    fn print(self: &SupplyStacks) {
        // Print content 
        for row_i in 0..self.max_stack_height {
            for stack_i in 0..self.no_of_stacks {
                let stack = &self.store[stack_i];
                let diff = self.max_stack_height - stack.len();

                if row_i < diff {
                    print!("    ");
                    continue;
                }

                print!("[{}] ", stack[stack.len() - (row_i - diff + 1)]);
            }
            println!();
        }

        // Print footer
        for i in 0..self.no_of_stacks {
            print!(" {} ", i + 1);
        }
        println!();
    }
}