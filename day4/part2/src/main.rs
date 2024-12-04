use std::{io::{self, BufRead}, fs::File};

fn main() -> io::Result<()> {
    let file = File::open("../input.in")?;
    let reader = io::BufReader::new(file);
    
    let mut string_input = Vec::<Vec<char>>::new();
    for line in reader.lines() {
        string_input.push(line?.chars().collect());
    }

    // create a 3x3 window and search for MAS, SAM in X
    let mut num_xmas = 0;
    for i in 0..=string_input.len() - 3 {
        for j in 0..=string_input[i].len() - 3 {
            let mut window = Vec::<Vec<char>>::new();
            for k in 0..3 {
                window.push(string_input[i+k][j..j+3].to_vec());
            }
            // search for MAS / SAM
            // brute force matching because i am lazy
            if (window[0][0] == 'M' && window[1][1] == 'A' && window[2][2] == 'S') || 
               (window[0][0] == 'S' && window[1][1] == 'A' && window[2][2] == 'M') {
                if (window[2][0] == 'M' && window[1][1] == 'A' && window[0][2] == 'S') || 
                   (window[2][0] == 'S' && window[1][1] == 'A' && window[0][2] == 'M') {
                   num_xmas += 1;
               }
            }
        }
    }
    
    println!("Number of X-MAS: {}", num_xmas);

    Ok(())
}


