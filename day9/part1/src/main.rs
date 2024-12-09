use std::{io::{self, BufRead}, fs::File};

fn main() -> io::Result<()> {
    let file = File::open("../input.in")?;
    let reader = io::BufReader::new(file);
    
    let mut diskmap: Vec<char> = vec![];
    for line in reader.lines() {
        diskmap = line.unwrap().chars().collect();
    }
    
    let mut memory = Vec::<i32>::new(); 
    let mut curr_id = 0;
    for i in (0..diskmap.len()).step_by(2) {
        let blk_sz = diskmap[i].to_digit(10).unwrap();
        for _ in 0..blk_sz {
            memory.push(curr_id);
        }
        if i + 1 < diskmap.len(){
            let free_sz = diskmap[i+1].to_digit(10).unwrap();
            for _ in 0..free_sz {
                memory.push(-1);
            }
        }
        curr_id += 1;
    }
    // println!("Memory: {:?}", memory);

    // compress
    let mut free_ptr = 0;
    let mut back_ptr = memory.len()-1;
    
    while back_ptr >= free_ptr {
        // println!("{}, {}", free_ptr, back_ptr);
        if memory[back_ptr] < 0 {
            back_ptr -= 1;
            continue;
        }
        if memory[free_ptr] >= 0 {
            free_ptr += 1;
            continue;
        }
        memory[free_ptr] = memory[back_ptr];
        memory[back_ptr] = -1;
    }

    // calculate checksum
    let mut checksum: i64 = 0;
    for (i, val) in memory.iter().enumerate() {
        if val < &0 {
            break;
        }
        checksum += i as i64 * (*val as i64);
    }

    // println!("Result: {:?}", memory);
    println!("Checksum: {}", checksum);

    Ok(())
}