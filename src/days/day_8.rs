use std::fs;
use std::io::BufReader;
use std::io::prelude::*;

pub fn run(file_name: &str) -> std::io::Result<()> {
    let file = fs::File::open(file_name)
        .expect("File not found");
    let reader = BufReader::new(file);

    let matrix = parse_matrix(reader)?;
    let mut is_visible = empty_matrix(matrix.len() as u32, 0);
    let mut scenic_scores = empty_matrix(matrix.len() as u32, (0, 0, 0, 0));

    part_one(&matrix, &mut is_visible);
    part_two(&matrix, &mut scenic_scores);



    // print matrix and get number visible 
    let mut visible = 0;

    for i in 0..is_visible.len() {
        for j in 0..is_visible[0].len() { 
            print!("{}", is_visible[i][j]);
            if is_visible[i][j] > 0 {
                visible += 1;
            }
        }
        println!();
    }

    println!();


    let mut max_score = 0;

    for i in 0..scenic_scores.len() {
        for j in 0..scenic_scores[0].len() { 
            let val = scenic_scores[i][j];
            let score = val.0 * val.1 * val.2 * val.3;

            print!("({} {} {} {})", val.0, val.1, val.2, val.3);
            if score > max_score {
                max_score = score;
            }
        }
        println!();
    }

    println!("visible: {}", visible);
    println!("max score: {}", max_score);

    Ok(())
}



fn part_one(matrix: &Vec<Vec<u32>>, is_visible: &mut Vec<Vec<u32>>) {
    for row_i in 0..matrix.len() {
        let row = &matrix[row_i];
        let len = row.len();
        let mut high = row[0];
        
        // left to right check 
        is_visible[row_i][0] += 1;
        for col_i in 1..len {
            let height = row[col_i];
            if height > high {
                high = height;
                is_visible[row_i][col_i] += 1;
            }
        }

        // right to left check
        is_visible[row_i][len - 1] += 1;
        high = row[len - 1];

        for i in 1..(len + 1) {
            let col_i = len - i; 
            let height = row[col_i];
            if height > high {
                high = height;
                is_visible[row_i][col_i] += 1;
            }
        }
    }

    // Check cols 
    for col_i in 0..matrix.len() {
        let len = matrix[0].len();
        let mut high = matrix[0][col_i];
        
        // left to right check 
        is_visible[0][col_i] += 1;
        for row_i in 1..len {
            let height = matrix[row_i][col_i];
            if height > high {
                high = height;
                is_visible[row_i][col_i] += 1;
            }
        }

        // right to left check
        is_visible[len - 1][col_i] += 1;

        for i in 1..(len + 1) {
            let row_i = len - i; 
            let height = matrix[row_i][col_i];
            if height > high {
                high = height;
                is_visible[row_i][col_i] += 1;
            }
        }
    }
}


fn part_two(matrix: &Vec<Vec<u32>>, scenic_scores: &mut Vec<Vec<(u32, u32, u32, u32)>>) {
    let len = matrix.len();

    for row_i in 0..len {
        for col_i in 0..len {
            let val = matrix[row_i][col_i]; 
            let score = &mut scenic_scores[row_i][col_i];

            // Check left 
            for row_i in (0..row_i).rev() {
                score.0 += 1;
                if matrix[row_i][col_i] >= val { break; }
            }

            // Check right 
            for row_i in (row_i + 1)..len {
                score.1 += 1;
                if matrix[row_i][col_i] >= val { break; }
            }

            // Check up 
            for col_i in (0..col_i).rev() {
                score.2 += 1;
                if matrix[row_i][col_i] >= val { break; }
            }

            // Check down
            for col_i in (col_i + 1)..len {
                score.3 += 1;
                if matrix[row_i][col_i] >= val { break; }
            }
        }
    }
}


fn empty_matrix<T: std::clone::Clone + Copy>(size: u32, init_val: T) -> Vec<Vec<T>> {
    let mut out = Vec::new();

    for _ in 0..size {
        let mut line = Vec::with_capacity(size.try_into().unwrap());
        line.resize(size.try_into().unwrap(), init_val);
        out.push(line);
    }

    out
}


fn parse_matrix<R>(reader: BufReader<R>) -> std::io::Result<Vec<Vec<u32>>> where R: std::io::Read {
    let mut out = Vec::new();

    for l in reader.lines() {
        let line = l?;
        out.push(parse_matrix_line(&line));
    }

    Ok(out)
}


fn parse_matrix_line(line: &str) -> Vec<u32> {
    let mut out = Vec::new();

    for ch in line.chars() {
        out.push(ch as u32 - '0' as u32);
    }

    return out;
}