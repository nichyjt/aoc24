use std::{
    io::{self, BufReader, BufRead},
    fs::File,
    collections::HashMap,
};

const LIMIT: i64 = 75;

fn main() -> io::Result<()> {
    let file = File::open("../input.in")
        .unwrap_or_else(|err| {
            eprintln!("Error loading file: {}", err);
            std::process::exit(1);
        });
    
    let reader = BufReader::new(file);
    let mut vals: Vec<usize> = Vec::new();
    
    for line in reader.lines() {
        for word in line?.split_whitespace() {
            if let Ok(num) = word.parse() {
                vals.push(num);
            }
        }
    }
    
    vals.sort_by(|a, b| b.cmp(a));
    
    let mut result = 0;
    let mut lut: HashMap<(i64, i64), i64> = HashMap::new();
    
    for curr_val in vals {
        // println!("OK");
        
        if curr_val == 0 {
            search(&mut lut, 1, 1);
            let next_count = lut.get(&(1, 1)).cloned().unwrap_or(0);
            result += next_count;
        } else {
            let curr_str = curr_val.to_string();
            
            if curr_str.len() % 2 == 0 {
                let half_len: i64 = curr_str.len() as i64/ 2;
                let first: i64 = curr_str[..half_len as usize].parse().unwrap();
                let second: i64 = curr_str[half_len as usize..].parse().unwrap();
                
                search(&mut lut, first, 1);
                search(&mut lut, second, 1);
                
                let first_count = lut.get(&(first, 1)).cloned().unwrap_or(0);
                let second_count = lut.get(&(second, 1)).cloned().unwrap_or(0);
                
                result += first_count + second_count;
            } else {
                let transformed_val = curr_val * 2024;
                
                search(&mut lut, transformed_val as i64, 1);
                
                let transformed_count = lut
                    .get(&(transformed_val as i64, 1))
                    .cloned()
                    .unwrap_or(0);
                
                result += transformed_count;
            }
        }
    }
    
    println!("Result: {}", result);
    Ok(())
}

fn search(lut: &mut HashMap<(i64, i64), i64>, curr_val: i64, curr_blink: i64) {
    // Base case: Stop recursion at LIMIT
    if curr_blink == LIMIT {
        lut.insert((curr_val, curr_blink), 1);
        return;
    }

    // has been computed
    if lut.contains_key(&(curr_val, curr_blink)) {
        return;
    }

    let mut count = 0;

    if curr_val == 0 {
        search(lut, 1, curr_blink + 1);
        count += lut.get(&(1, curr_blink + 1)).cloned().unwrap_or(0);
    } else {
        let curr_str = curr_val.to_string();

        if curr_str.len() % 2 == 0 {
            let half_len = curr_str.len() / 2;
            let first: i64 = curr_str[..half_len].parse().unwrap();
            let second: i64 = curr_str[half_len..].parse().unwrap();

            search(lut, first, curr_blink + 1);
            search(lut, second, curr_blink + 1);

            count += lut.get(&(first, curr_blink + 1)).cloned().unwrap_or(0);
            count += lut.get(&(second, curr_blink + 1)).cloned().unwrap_or(0);
        } else {
            let transformed_val = curr_val * 2024;
            search(lut, transformed_val, curr_blink + 1);
            count += lut.get(&(transformed_val, curr_blink + 1)).cloned().unwrap_or(0);
        }
    }

    lut.insert((curr_val, curr_blink), count);
}