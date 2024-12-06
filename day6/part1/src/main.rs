use std::{fs::File, io};
use std::io::BufRead;

fn main() -> io::Result<()> {
    let file = File::open("../input.in")?;
    let reader = io::BufReader::new(file);
    
    let mut maze = Vec::<Vec<char>>::new();
    let mut g_row:i32 = 0;
    let mut g_col:i32 = 0;
    for line in reader.lines() {
        let line = line?;
        let pos = line.chars().position(|c| c == '^');
        maze.push(line.chars().collect());
        if let Some(p) = pos {
            g_col = p as i32;
            g_row = (maze.len() as i32) - 1;
        }
    }

    let mut state = 0;
    let pos: [(i32, i32); 4] = [(-1,0),(0,1),(1,0),(0,-1)];
    const OBSTACLE: char = '#';
    const VISITED: char = 'X';
    let mut visited = 0;
    // Run the simulation
    loop {
        if maze[g_row as usize][g_col as usize] != VISITED {
            maze[g_row as usize][g_col as usize] = VISITED;
            visited += 1;
        }
        // get delta
        let (dr, dc) = pos[state];
        let r = g_row + dr;
        let c = g_col + dc;
        if r < 0 || r >= maze.len() as i32 || c < 0 || c >= maze[r as usize].len() as i32{
            // guard has left the maze. goodbye!
            break;
        } 
        // guard has not left the maze. check validity
        if maze[r as usize][c as usize] == OBSTACLE {
            state = (state + 1) % pos.len();
            continue;
        }
        // move 
        g_row = r;
        g_col = c;
    }
    
    // for row in maze {
    //     for c in row {
    //         print!("{}", c);
    //     }
    //     println!();
    // }
    println!("Result: {}", visited);

    Ok(())
}