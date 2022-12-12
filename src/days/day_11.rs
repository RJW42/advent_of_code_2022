use std::fs;
use std::io::BufReader;
use std::io::prelude::*;

use crate::d11::OperationType::*;
use crate::d11::Operand::*;


struct Monkey {
    items: Vec<u64>,
    items_inspected: u64,
    operation: Operation,
    test_amount: u64,
    true_monkey: u64,
    false_monkey: u64,
}

struct Operation {
    v1: Operand,
    v2: Operand,
    opp: OperationType,
}

enum OperationType {
    Add,
    Sub,
    Mul,
    Div,
}

enum Operand {
    Old,
    Val(u64),
}



pub fn run(file_name: &str) -> std::io::Result<()> {
    let mut monkeys = parse_monkeys(file_name)?;
    let rounds = 10000;
    let decrease_amount = 1;
    let mut distress = 1;

    for m in &monkeys {
        distress *= m.test_amount;
    }

    if decrease_amount != 1 {
        distress = 1;
    }

    for round_i in 0..rounds {
        for monkey_i in 0..monkeys.len() {
            // Plays with items 
            monkeys[monkey_i].play_with_items(decrease_amount);
            let true_m = monkeys[monkey_i].true_monkey;
            let false_m = monkeys[monkey_i].false_monkey;
            let div = monkeys[monkey_i].test_amount;


            let items = monkeys[monkey_i].items.clone();
            monkeys[monkey_i].items.clear();

            // Moves items to other monkey
            for item in items {
                if item % div == 0 {
                    monkeys[true_m as usize].items.push(item % distress);
                } else {
                    monkeys[false_m as usize].items.push(item % distress);
                }
            }
        }

        println!("After round: {}", round_i);
        for (i, monkey) in monkeys.iter().enumerate() {
            println!(
                "Monkey {} inspected {} items: {:?}", 
                i, monkey.items_inspected, monkey.items);
        }
        println!();
    }

    let mut top_1 = 0;
    let mut top_2 = 0;

    for (i, monkey) in monkeys.iter().enumerate() {
        let items = monkey.items_inspected;
        println!("Monkey {} inspected {} items", i, items);

        if items > top_1 {
            top_2 = top_1;
            top_1 = items;
        } else if items > top_2 {
            top_2 = items;
        }
    }

    println!("T1: {}, T2: {}, S: {}", top_1, top_2, top_1 * top_2);

    Ok(())
}

/* Problem */


/* Debug */
impl Monkey {
    fn print(&self) {
        self.print_items();
        self.operation.print();
        self.print_condition();
    }

    fn print_items(&self) {
        println!("items: {:?}", self.items);
    }

    fn print_condition(&self) {
        println!(
            "if div by {} then {} else {}", 
            self.test_amount, self.true_monkey,
            self.false_monkey
        );
    }

    fn play_with_items(&mut self, decrease_amount: u64) {
        self.items_inspected += self.items.len() as u64;

        for item in self.items.iter_mut() {
            *item = self.operation.perform_opp(*item) / decrease_amount;
        }
    }
}

impl Operation {
    fn print(&self) {
        print!("opp: ");
        self.v1.print();
        print!(" ");
        self.opp.print();
        print!(" ");
        self.v2.print();
        println!();
    }

    fn perform_opp(&self, old_value: u64) -> u64 {
        let v1 = self.v1.get_val(old_value);
        let v2 = self.v2.get_val(old_value);

        match self.opp {
            Add => v1 + v2,
            Sub => v1 - v2,
            Mul => v1 * v2,
            Div => v1 / v2,
        }    
    }
}

impl OperationType {
    fn print(&self) {
        match self {
            Add => print!("+"),
            Sub => print!("-"),
            Mul => print!("*"),
            Div => print!("/"),
        }
    }
}

impl Operand {
    fn print(&self) {
        match self {
            Val(v) => print!("{}", v),
            Old => print!("old"),
        }
    }

    fn get_val(&self, old_value: u64) -> u64 {
        match self {
            Val(v) => *v,
            Old => old_value,
        }
    }
}


/* Parsing */
fn parse_monkeys(file_name: &str) -> std::io::Result<Vec<Monkey>> {
    let file = fs::File::open(file_name)
        .expect("File not found");
    let reader = BufReader::new(file);
    
    let mut line_i = 0;
    let mut lines = Vec::new();
    let mut monkeys = Vec::new();

    for l in reader.lines() {
        let line = l?;

        if line.len() == 0 {
            continue;
        }

        lines.push(line);
        line_i += 1;

        if line_i < 6 {
            continue;
        }

        if let Some(monkey) = parse_monkey(&lines) {
            println!("Monkey: ");
            monkey.print();
            monkeys.push(monkey);
            println!();
        }

        lines.clear();
        line_i = 0;
    }

    Ok(monkeys)
}


fn parse_monkey(lines: &Vec<String>) -> Option<Monkey> {
    Some(Monkey {
        items: parse_items(&lines[1])?,
        items_inspected: 0,
        operation: parse_operation(&lines[2])?,
        test_amount: parse_num(&lines[3], 21)?,
        true_monkey: parse_num(&lines[4], 29)?,
        false_monkey: parse_num(&lines[5], 30)?
    })
}


fn parse_items(line: &str) -> Option<Vec<u64>> {
    //   Starting items: a, b, c, ...
    let mut items = Vec::new();
    let mut curr_item = 0;

    for ch in line.chars().skip(18) {
        match ch {
            '0'..='9' => {
                curr_item = curr_item * 10 + (ch as u64 - '0' as u64);
            },
            ',' => {
                items.push(curr_item);
                curr_item = 0;
            },
            ' ' => (),
            _ => return None,
        }
    }

    items.push(curr_item);

    Some(items)
}

fn parse_operation(line: &str) -> Option<Operation> {
    //   Operation: new = (v1) (opp) (v2)
    let chars = line.chars().collect::<Vec<char>>();
    let mut chars_iter = chars.iter().skip(19);

    let v1 = parse_operand(&mut chars_iter)?;
    let opp = parse_opperation_type(&mut chars_iter)?;
    let v2 = parse_operand(&mut chars_iter)?;

    Some(Operation{
        v1: v1,
        opp: opp,
        v2: v2,
    })
}


fn parse_opperation_type<'a, I>(chars: &mut I) -> Option<OperationType> 
where I: Iterator<Item = &'a char> {
    let output = match chars.next()? {
        '+' => Some(Add),
        '-' => Some(Sub),
        '*' => Some(Mul),
        '/' => Some(Div),
        _ => None,
    }?;

    chars.next();

    Some(output)
}

fn parse_operand<'a, I>(chars: &mut I) -> Option<Operand> 
where I: Iterator<Item = &'a char> {
    let ch = chars.next()?;

    if *ch == 'o' {
        // Old value
        chars.next()?;
        chars.next()?;
        chars.next();

        return Some(Old);
    }

    // Numeric value 
    let mut val = (*ch as u64) - ('0' as u64);

    while let Some(ch) = chars.next() {
        match ch {
            '0'..='9' => val = val * 10 + (*ch as u64) - ('0' as u64),
            _ => break,
        }
    }

    Some(Val(val))
}


fn parse_num(line: &str, start_pos: u64) -> Option<u64>   {
    let mut val = 0;

    for ch in line.chars().skip(start_pos.try_into().unwrap()) {
        match ch {
            '0'..='9' => val = val * 10 + (ch as u64) - ('0' as u64),
            _ => break,
        }
    }

    Some(val)
}
