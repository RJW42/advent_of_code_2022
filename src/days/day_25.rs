use std::fs;
use std::io::BufReader;
use std::io::prelude::*;


pub fn run(file_name: &str) -> std::io::Result<()> {
    let file = fs::File::open(file_name)?;
    let reader = BufReader::new(file);


    let mut sum = 0;

    for l in reader.lines() {
        let number = parse_snafu_number(&l?);
        sum += number;

        println!("{}", number);
    }

    println!("p1: {}", sum);
    println!("p1 snafu: {}", into_snafu_number(sum).unwrap());

    Ok(())
}


fn parse_snafu_number(line: &str) -> i64 {
    let mut output = 0;

    for ch in line.chars() {
        output = output * 5 + parse_snafu_digit(ch).unwrap();
    }

    output
}

fn parse_snafu_digit(
    ch: char
) -> Option<i64> {
    match ch {
        '0' => Some(0),
        '1' => Some(1),
        '2' => Some(2),
        '-' => Some(-1),
        '=' => Some(-2),
        _ => None,
    }
}

fn into_snafu_number(
    num: i64, 
) -> Option<String> {
    let mut string = String::with_capacity(50); 
    let mut n = num;

    while n > 0 {
        let digit = n % 5;
        n = n / 5;

        let (ch, carry) = into_snafu_digit(digit)?;

        string.insert(0, ch);

        n += carry;
    }

    Some(string)
}

fn into_snafu_digit(
    num: i64
) -> Option<(char, i64)> {
    match num {
        0 => Some(('0', 0)),
        1 => Some(('1', 0)),
        2 => Some(('2', 0)),
        3 => Some(('=', 1)),
        4 => Some(('-', 1)),
        _ => return None,
    }
}