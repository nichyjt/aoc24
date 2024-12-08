use std::{fs::File, io::{self, BufRead}, collections::{HashMap, HashSet}};

fn main() -> io::Result<()> {
    let file = File::open("../input.in")?;
    let reader = io::BufReader::new(file);
    
    let mut space = Vec::<Vec<char>>::new();
    let mut antenna_map = HashMap::<char, Vec::<(i32,i32)>>::new();

    let mut row = 0;
    for line in reader.lines() {
        let line = line?;
        space.push(line.chars().collect());
        let mut col = 0;
        for char in line.chars() {
            if char != '.' {
                if let Some(antennas) = antenna_map.get_mut(&char) {
                    antennas.push((row, col));
                } else {
                    antenna_map.insert(char, vec![(row, col)]);
                }
            }
            col += 1;
        }
        row += 1;
    }
    let mut antinodes = HashSet::<(i32,i32)>::new();
    // do a pair-wise antenna search 
    for antenna_name in antenna_map.keys() {
        if let Some(points) = antenna_map.get(antenna_name) {
            for &(r1, c1) in points.iter() {
                for &(r2, c2) in points.iter() {
                    if (r1, c1) != (r2, c2) {
                        let dr = r2 - r1;
                        let dc = c2 - c1;

                        // antinode candidates
                        let candidates = [
                            (r1 - dr, c1 - dc), // node opposite to (r1, c1)
                            (r2 + dr, c2 + dc), // node opposite to (r2, c2)
                        ];

                        for &(ar, ac) in &candidates {
                            let d1 = (ar - r1).abs() + (ac - c1).abs();
                            let d2 = (ar - r2).abs() + (ac - c2).abs();

                            if d1 == 2 * d2 || d2 == 2 * d1 {
                                if ar < 0 || ar >= space.len() as i32 || ac < 0 || ac >= space[0].len() as i32 {
                                    continue;
                                }
                                antinodes.insert((ar, ac));
                            }
                        }
                    }
                }
            }
        }
    }
    println!("Result: {}",antinodes.len());    


    Ok(())
}