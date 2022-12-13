use std::fs;
use std::io::BufReader;
use std::io::prelude::*;

use std::iter::Peekable;

use std::cmp::Ordering;

use crate::d13::PacketElement::*;
use crate::d13::Comparison::*;



struct Packet {
    elements: Vec<PacketElement>,    
}

#[derive(Clone)]
enum PacketElement {
    Num(u32),
    Lst(Vec<PacketElement>),
}

enum Comparison {
    Continue,
    OutOfOrder,
    InOrder
}


pub fn run(file_name: &str) -> std::io::Result<()> {
    let packets = match parse_packets(file_name) {
        Some(packets) => packets?,
        _ => panic!(),
    };

    let mut score = 0;
    let mut not_sure_count = 0;
    let mut list = Vec::new();

    for (i, (left, right)) in packets.iter().enumerate() {
        left.print();
        right.print();

        let left_as_element = Lst(left.elements.clone());
        let right_as_element = Lst(right.elements.clone());
        
        match compare_elements(&left_as_element, &right_as_element, 0, true) {
            InOrder => { 
                println!("In Order: {}", i + 1);
                score += i + 1;
            },
            OutOfOrder =>println!("Out of Order"),
            Continue => {
                println!("Not Sure");
                not_sure_count += 1;
            }
        };
        println!();

        list.push(left_as_element);
        list.push(right_as_element);
    }

    println!("score: {}, not sure count: {}", score, not_sure_count);

    // Add decoder packets
    list.push(Lst(vec![Lst(vec![Num(2)])]));
    list.push(Lst(vec![Lst(vec![Num(6)])]));

    list.sort_by(|a, b| match compare_elements(&a, &b, 0, false) {
        InOrder => Ordering::Less,
        OutOfOrder => Ordering::Greater,
        Continue => Ordering::Equal,
    });

    let mut part_2_score = 1;

    for (i, packet) in list.iter().enumerate() {
        packet.print();
        println!();

        part_2_score *= match packet {
            Lst(inner) => match &inner[..] {
                [Lst(second_inner)] => match &second_inner[..] {
                    [Num(2)] | [Num(6)] => (i + 1),
                    _ => 1,
                },
                _ => 1,
            },
            _ => 1,
        };
    }

    println!("part 2 score: {}", part_2_score);

    Ok(())
}

/* Functionality */

fn compare_elements(
        left: &PacketElement, right: &PacketElement, 
        depth: u32, debug: bool
) -> Comparison {
    if debug {
        print!("{: <1$} - compare ", "", depth as usize);
        left.print();
        print!(" vs ");
        right.print();
        println!();
    }

    match (left, right) {
        (Num(lv), Num(rv)) => 
            compare_numbers(*lv, *rv),
        (Lst(ll), Lst(rl)) => 
            compare_lists(ll, rl, depth, debug),
        (Num(lv), _) => 
            compare_elements(
                &to_list(*lv, depth + 1, debug), right, depth + 1, debug
            ),
        (_, Num(rv)) => 
            compare_elements(
                left, &to_list(*rv, depth + 1, debug), depth + 1, debug
            ),
    }
}

fn to_list(val: u32, depth: u32, debug: bool) -> PacketElement {
    let mut list = Vec::new();

    if debug {
        println!("{: <1$} - Mixed types; convert val to list", "", depth as usize);
    }

    list.push(Num(val));

    Lst(list)
}

fn compare_numbers(left: u32, right: u32) -> Comparison{
    if left < right {
        InOrder
    } else if left > right  {
        OutOfOrder
    } else {
        Continue
    }
}

fn compare_lists(
        left: &Vec<PacketElement>, right: &Vec<PacketElement>, 
        depth: u32, debug: bool
) -> Comparison {
    for (left_child, right_child) in left.iter().zip(right.iter()) {
        match compare_elements(left_child, right_child, depth + 1, debug) {
            Continue => (),
            stop @ _ => return stop,
        };
    }

    if left.len() < right.len() {
        InOrder 
    } else if left.len() > right.len() {
        OutOfOrder
    } else {
        Continue
    }
}


/* Debug */

impl Packet {
    fn print(&self) {
       let element = Lst(self.elements.clone());
       element.print();
       println!();
    }
}

impl PacketElement {
    fn print(&self) {
        match self {
            Num(v) => print!("{}", v),
            Lst(l) => {
                print!("[");
                for element in l {
                    element.print();
                    print!(",");
                }
                print!("]")
            }
        }
    }
}

/* Parsing */

fn parse_packets(file_name: &str) -> Option<std::io::Result<Vec<(Packet, Packet)>>> {
    let file = fs::File::open(file_name)
        .expect("File not found");
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    let mut output = Vec::new();

    loop {
        let l1 = lines.next()?.ok()?;
        let l2 = lines.next()?.ok()?;
        
        let p1 = parse_packet(&l1)?;
        let p2 = parse_packet(&l2)?;

        output.push((p1, p2));

        if let None = lines.next() {
            break;
        }
    }

    Some(Ok(output))
}


fn parse_packet(line: &str) -> Option<Packet> {
    let chars = line.chars().collect::<Vec<char>>();
    let mut chars_iter = chars.iter().peekable();
    let body = parse_packet_lst(&mut chars_iter)?; 

    if let Lst(elements) = body {
        Some(Packet {
            elements: elements, 
        })
    } else {
        None
    }
}


fn parse_packet_element<'a, I>(chars: &mut Peekable<I>) -> Option<PacketElement> 
where I: Iterator<Item = &'a char> {
    parse_packet_lst(chars)
        .or(parse_packet_num(chars))   
}


fn parse_packet_num<'a, I>(chars: &mut Peekable<I>) -> Option<PacketElement> 
where I: Iterator<Item = &'a char> {
    let mut output = 0;

    match chars.peek()? {
        '0'..='9' => (),
        _ => return None,
    };

    loop {
        match chars.peek()? {
            ch @ '0'..='9' => {
                output = output * 10 + (**ch as u32) - ('0' as u32);
                chars.next();
            }
            ',' => break,
            ']' => break,
            _ => return None,
        };
    }

    Some(Num(output))
}

fn parse_packet_lst<'a, I>(chars: &mut Peekable<I>) -> Option<PacketElement> 
where I: Iterator<Item = &'a char> {
    if **chars.peek()? != '[' {
        return None;
    }

    chars.next();

    let mut output = Vec::new();

    while let Some(element) = parse_packet_element(chars) {
        output.push(element);
        if let Some(ch) = chars.peek() {
            if **ch == ',' {
                chars.next();
            }
        }
    }    

    if *chars.next()? != ']' {
        return None;
    }

    Some(Lst(output))
}



