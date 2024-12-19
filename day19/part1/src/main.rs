use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let res = File::open("../input.in");
    // Results need to be unwrapped. Prefer to use ? operator in the future
    let file = res.unwrap_or_else(|err| {
        eprintln!("Error loading file: {}", err);
        std::process::exit(1);
    });

    let reader = io::BufReader::new(file);
    let mut patterns= Vec::<String>::new();
    let mut targets = Vec::<String>::new();
    for (i, line) in reader.lines().enumerate() {
        if i == 0 {
            let str = line?;
            let parts = str.split(", ");
            for p in parts {
                patterns.push(p.to_string());
            }
        }else {
            let str = line?;
            if str.len() <= 0 {
                continue;
            }
            targets.push(str);
        }
    }
    // build string with parts. try a DFS
    let mut num_ok = 0;
    for target in targets {
        let is_valid = search(&patterns, &target, 0);
        if is_valid {
            num_ok += 1;
            // println!("{} OK", target);
        }else{
            // println!("{} NOK", target);
        }
    }
    println!("Result: {}", num_ok);

    Ok(())
}

// try building a string with parts
fn search(patterns: &Vec::<String>, target: &String, index: usize) -> bool {
    if index == target.len() {
        return true;
    }
    let mut is_valid = false;

    for pattern in patterns {
        let len = pattern.len();
        if index + len > target.len() {
            continue;
        }
        if target[index..index+len] == *pattern {
            is_valid |= search(patterns, &target, index+len);
            if is_valid {
                break;
            }
        } 
    }
    return is_valid
}