use std::{io::{self, BufRead}, fs::File};

fn main() -> io::Result<()> {
    let file = File::open("../input.in")?;
    let reader = io::BufReader::new(file);
    
    let mut string_input = Vec::<Vec<char>>::new();
    for line in reader.lines() {
        string_input.push(line?.chars().collect());
    }

    let pattern = "XMAS";
    let mut num_xmas = 0;
    // LEFT search
    for row in &string_input {
        for i in 0..row.len() - 3 {
            let window: String = row[i..i+4].iter().collect();
            if window == pattern {
                num_xmas += 1;
            }
        }
    }
    // RIGHT search
    for row in &string_input {
        for i in 3..row.len() {
            let mut window = String::new();
            for k in 0..4 {
                window.push(row[i-k]);
            }
            if window == pattern {
                num_xmas += 1;
            }
        }
    }
    // vertical search
    for i in 0..string_input.len() {
        for j in 0..string_input[i].len() {
            // up
            if i >= 3 {
                let mut window: String = String::new();
                for k in 0..4 {
                    window.push(string_input[i-k][j]);
                }
                if window == pattern {
                    num_xmas += 1;
                }
            }
            // down
            if i <= string_input.len() - 4 {
                let mut window = String::new();
                for k in 0..4 {
                    window.push(string_input[i+k][j]);
                }
                if window == pattern {
                    num_xmas += 1;
                }
            }
        }
    }
    // diagonal search
    for i in 0..string_input.len() {
        for j in 0..string_input[i].len() {
            // up right
            if i >= 3 && j <= string_input[i].len() - 4 {
                let mut window = String::new();
                for k in 0..4 {
                    window.push(string_input[i - k][j + k]);
                }
                if window == pattern {
                    num_xmas += 1;
                }
            }
            // up left
            if i >= 3 && j >= 3 {
                let mut window = String::new();
                for k in 0..4 {
                    window.push(string_input[i - k][j - k]);
                }
                if window == pattern {
                    num_xmas += 1;
                }
            }
            // down right
            if i <= string_input.len() - 4 && j <= string_input[i].len() - 4 {
                let mut window = String::new();
                for k in 0..4 {
                    window.push(string_input[i + k][j + k]);
                }
                if window == pattern {
                    num_xmas += 1;
                }
            }
            // down left
            if i <= string_input.len() - 4 && j >= 3 {
                let mut window = String::new();
                for k in 0..4 {
                    window.push(string_input[i + k][j - k]);
                }
                if window == pattern {
                    num_xmas += 1;
                }
            }
        }
    }

    println!("Number of XMAS: {}", num_xmas);

    Ok(())
}

