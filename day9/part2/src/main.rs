use std::{io::{self, BufRead}, fs::File};

fn main() -> io::Result<()> {
    let file = File::open("../input.in")?;
    let reader = io::BufReader::new(file);
    
    let mut diskmap: Vec<char> = vec![];
    for line in reader.lines() {
        diskmap = line.unwrap().chars().collect();
    }
    
    let mut memory = Vec::<i32>::new(); 
    // Map each file id via its index to its (memory index, size)
    let mut id_sz = Vec::<(i32, i32)>::new();
    // Map free block indexes and their sizes
    let mut free_sz_map = Vec::<(i32, i32)>::new();
    
    let mut curr_id = 0;
    
    for i in (0..diskmap.len()).step_by(2) {
        let blk_sz = diskmap[i].to_digit(10).unwrap();
        id_sz.push((memory.len() as i32, blk_sz as i32));
        for _ in 0..blk_sz {
            memory.push(curr_id);
        }
        if i + 1 < diskmap.len(){
            let free_sz = diskmap[i+1].to_digit(10).unwrap();
            free_sz_map.push((memory.len() as i32, free_sz as i32));
            for _ in 0..free_sz {
                memory.push(-1);
            }
        }
        curr_id += 1;
    }
    // println!("Memory: {:?}", memory);
    // println!("File Mappings: {:?}", id_sz);
    // println!("Free Mappings: {:?}", free_sz_map);

    // compress
    for (file_id, (file_idx, file_size)) in id_sz.iter().rev().enumerate() {
        let file_id = id_sz.len() - file_id - 1;
        // check if free space exists
        for (free_id, (free_idx, free_sz)) in free_sz_map.iter().enumerate() {
            if free_sz < file_size || file_idx < free_idx {
                continue;
            }
            // enough size exists. transfer the file over
            // println!("Transferring {} to {}", file_id, free_idx);
            let mut writer = *free_idx;
            for _ in 0..*file_size {
                memory[writer as usize] = file_id as i32;
                writer += 1;
            }
            let new_free_idx = writer;

            writer = *file_idx;
            for _ in 0..*file_size {
                memory[writer as usize] = -1;
                writer += 1;
            }
            // update the free_id
            free_sz_map[free_id] = (new_free_idx, free_sz - file_size);
            // println!("Memory: {:?}", memory);
            break;
        }
    }


    // calculate checksum
    let mut checksum: i64 = 0;
    for (i, val) in memory.iter().enumerate() {
        if val < &0 {
            continue;
        }
        checksum += i as i64 * (*val as i64);
    }
    // println!("Result: {:?}", memory);
    println!("Checksum: {}", checksum);

    Ok(())
}
