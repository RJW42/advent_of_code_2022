use std::fs;
use std::io::BufReader;
use std::io::prelude::*;

use crate::d7::TerminalLine::*;
use crate::d7::CDArgs::*;
use std::collections::HashMap;

enum TerminalLine {
    CD(CDArgs),
    LS,
    FS(File),
    DR(Dir),
}

enum CDArgs {
    GoRoot,
    GoBack,
    GoFile(String),
}

struct File {
    name: String,
    size: u32,
}

struct Dir {
    name: String,
    files: HashMap<String, File>,
    dirs: HashMap<String, u32>,
    parent_id: u32,
    size: u32
}


pub fn run(file_name: &str) -> std::io::Result<()> {
    let file = fs::File::open(file_name)
        .expect("File not found");
    let reader = BufReader::new(file);

    let mut dirs: HashMap<u32, Dir> = HashMap::new();
    let mut max_id = 0;

    // Add root dir 
    dirs.insert(0, Dir {
        name: "\\".to_string(),
        files: HashMap::new(),
        dirs: HashMap::new(),
        parent_id: 0,
        size: 0
    });

    let mut curr_dir_id = 0;

    /* Build file system */
    for l in reader.lines().skip(1) { /* Can skip first as is always cd / */
        let command = parse_terminal_line(&l?).unwrap();
        
        match command {
            LS => () /* Nothing needed to be done */ ,
            CD(arg) => curr_dir_id = get_new_curr_dir(arg, curr_dir_id, &dirs), 
            FS(file) => add_file(dirs.get_mut(&curr_dir_id).unwrap(), file),
            DR(mut dir) => {
                max_id = max_id + 1;
                dir.parent_id = curr_dir_id;
                dirs.get_mut(&curr_dir_id).unwrap().dirs.insert(
                    dir.name.clone(), max_id
                );
                dirs.insert(max_id, dir);
            },
        };
    }

    /* Calculate the fs sizes */
    let unused_space = 70000000 - populate_fs_size(&mut dirs, 0);
    let space_needed = 30000000 - unused_space;
    print_fs(&dirs, 0, 0);

    println!("unsued: {}", unused_space);
    println!("part one: {}", part_one(&dirs, 0));
    println!("part two: {}", part_two(&dirs, space_needed));

    Ok(())
}

/* Solution */
fn print_fs(dirs: &HashMap<u32, Dir>, curr_dir: u32, curr_depth: u32) {
    // Print current
    let dir = dirs.get(&curr_dir).unwrap();

    for _ in 0..curr_depth {print!("  "); }
    println!("- {} (dir, size={})", dir.name, dir.size);

    // Print dirs
    for (_, dir_id) in &dirs.get(&curr_dir).unwrap().dirs {
        print_fs(&dirs, *dir_id, curr_depth + 1);
    }

    // Print files 
    for (_, file) in &dirs.get(&curr_dir).unwrap().files {
        for _ in 0..(curr_depth + 1) {print!("  "); }
        println!("- {} (file, size={})", file.name, file.size);
    }
}

fn populate_fs_size(dirs: &mut HashMap<u32, Dir>, curr_dir: u32) -> u32 {
    let mut size = 0;

    // get size of all files 
    for (_, file) in &dirs.get(&curr_dir).unwrap().files {
        size += file.size;
    }

    // Get all sub dirs 
    let mut sub_dirs = Vec::new();

    for (_, id) in &dirs.get(&curr_dir).unwrap().dirs {
        sub_dirs.push(*id);
    }

    for k in sub_dirs {
        size += populate_fs_size(dirs, k);
    }

    // Save this size 
    dirs.get_mut(&curr_dir).unwrap().size = size;
    size
}

fn part_one(dirs: &HashMap<u32, Dir>, curr_dir: u32) -> u32 {
    let mut sum = 0;

    if dirs.get(&curr_dir).unwrap().size <= 100000 {
        sum += dirs.get(&curr_dir).unwrap().size;
    }

    for (_, id) in &dirs.get(&curr_dir).unwrap().dirs {
        sum += part_one(dirs, *id);
    }


    sum 
}

fn part_two(dirs: &HashMap<u32, Dir>, space_needed: u32) -> u32 {
    let mut curr_best = dirs.get(&0).unwrap();

    for (_, dir) in dirs {
        if dir.size < space_needed {
            continue;
        }

        if dir.size < curr_best.size {
            curr_best = dir;
        }
    }

    curr_best.size
}



/* FS Buildnig */
fn add_file(curr_dir: &mut Dir, file: File) {
    curr_dir.files.insert(file.name.clone(), file); 
}

fn get_new_curr_dir(
        cd_arg: CDArgs, curr_dir_id: u32, 
        dirs: &HashMap<u32, Dir>
) -> u32 {
    match cd_arg {
        GoRoot => 
            0,
        GoBack =>
            dirs.get(&curr_dir_id).unwrap().parent_id,
        GoFile(name) => 
            dirs.get(&curr_dir_id).unwrap().dirs.get(&name).unwrap().clone()       
    }
}

/* Parsing */
fn parse_terminal_line(line: &str) -> Option<TerminalLine> {
    match line.chars().next() {
        Some('$') => parse_terminal_command(line),
        Some('0'..='9') => parse_terminal_file(line),
        Some('d') => parse_terminal_dir(line),
        _ => None,
    }
}


fn parse_terminal_file(line: &str) -> Option<TerminalLine> { 
    let mut size = 0;
    let mut name = String::new();

    for ch in line.chars() {
        match ch {
            '0'..='9' => size = (size * 10) + (ch as u32 - '0' as u32),
            ' ' => (), 
            _ => name.push(ch),
        }
    }

    Some(FS(File {
        size: size,
        name: name
    }))
}


fn parse_terminal_dir(line: &str) -> Option<TerminalLine> {
    let mut name = String::new();

    for ch in line.chars().skip(4) {
        name.push(ch);
    }

    Some(DR(Dir {
        name: name,
        files: HashMap::new(),
        dirs: HashMap::new(),
        parent_id: 0,
        size: 0
    }))
}


fn parse_terminal_command(line: &str) -> Option<TerminalLine> {
    match line.chars().nth(2) {
        Some('l') => return Some(LS),
        Some('c') => (),
        _ => return None
    };

    match line.chars().nth(5) {
        Some('.') => return Some(CD(GoBack)),
        Some('/') => return Some(CD(GoRoot)),
        _ => ()
    };

    let mut name = String::new();

    for ch in line.chars().skip(5) {
        name.push(ch);
    }

    Some(CD(GoFile(name)))
}


impl TerminalLine {
    fn print(self: &TerminalLine) {
        match self {
            CD(arg) => print_cd(arg),
            LS => println!("$ ls"),
            FS(file) => println!("{} {}", file.size, file.name),
            DR(dir) => println!("dir {}", dir.name),
        }
    }
}

fn print_cd(arg: &CDArgs) {
    match arg {
        GoRoot => println!("$ cd /"),
        GoFile(name) => println!("$ cd {}", name),
        GoBack => println!("$ cd .."),
    }
}