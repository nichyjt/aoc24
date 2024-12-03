use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let res = File::open("input.in");
    // Results need to be unwrapped. Prefer to use ? operator in the future
    let file = res.unwrap_or_else(|err| {
        eprintln!("Error loading file: {}", err);
        std::process::exit(1);
    });

    let reader = io::BufReader::new(file);
    let mut left_arr: Vec<i32> = Vec::new();
    let mut right_arr: Vec<i32> = Vec::new();
    
    // Practice unwrapping into vectors
    for line in reader.lines() {
        let str = line?;
        // println!("{}", str);
        let parts: Vec<&str> = str.split("   ").collect();
        let left = parts[0].parse::<i32>().unwrap(); // unhandled unwrap
        let right = parts[1].parse::<i32>().unwrap();
        left_arr.push(left);
        right_arr.push(right);
    }

    // Create the similarity map
    let mut right_value_mapper: HashMap<i32, i32> = HashMap::new();
    for n in right_arr {
        // dereference what is returned by or_insert to access and modify the value 
        *(right_value_mapper.entry(n).or_insert(0)) += 1;
    }

    // Process the similarities
    let mut result:i64 = 0;
    for n in left_arr {
        // disgusting
        if right_value_mapper.contains_key(&n) {
            result += (n as i64) * (*right_value_mapper.get(&n).unwrap() as i64);
        }
    }
    println!("Similarity Score: {}", result);
    // Absolute insanity
    Ok(())
}