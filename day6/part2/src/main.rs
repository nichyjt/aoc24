use std::collections::HashSet;
use std::{fs::File, io};
use std::io::BufRead;

fn main() -> io::Result<()> {
    let file = File::open("../input.in")?;
    let reader = io::BufReader::new(file);
    
    let mut maze = Vec::<Vec<char>>::new();
    let mut obstacles = HashSet::<(i32,i32)>::new();
    let mut g_row:i32 = 0;
    let mut g_col:i32 = 0;
    for line in reader.lines() {
        let line = line?;
        let mut row = Vec::new();
    
        for (col, c) in line.chars().enumerate() {
            row.push(c);
            if c == '^' {
                g_row = maze.len() as i32;
                g_col = col as i32;
            } else if c == '#' {
                obstacles.insert((maze.len() as i32, col as i32));
            }
        }
    
        maze.push(row);
    }
    

    let original_r = g_row;
    let original_c = g_col;

    let mut state = 0;
    let pos: [(i32, i32); 4] = [(-1,0),(0,1),(1,0),(0,-1)];
    const OBSTACLE: char = '#';
    const VISITED: char = 'X';

    let mut visited = HashSet::<(i32,i32)>::new();
    // Run the simulation
    loop {
        if maze[g_row as usize][g_col as usize] != VISITED {
            maze[g_row as usize][g_col as usize] = VISITED;
            visited.insert((g_row, g_col));
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

    // now we re-run the simulation
    let mut result = 0;

    for (block_row, block_col) in visited {
        let mut g_row = original_r;
        let mut g_col = original_c;

        // Simulate again with the new obstacle
        obstacles.insert((block_row, block_col));

        // Initialize new_visited to track states and directions
        let mut new_visited = vec![
            vec![HashSet::<i32>::new(); maze[0].len()];
            maze.len()
        ];

        let mut state = 0;
        let mut cycle_found = false;

        loop {
            // Track the current position and state
            new_visited[g_row as usize][g_col as usize].insert(state as i32);

            // Get delta for the current direction
            let (dr, dc) = pos[state];
            let r = g_row + dr;
            let c = g_col + dc;

            // check if the guard has left the maze
            if r < 0 || r >= maze.len() as i32 || c < 0 || c >= maze[r as usize].len() as i32 {
                // guard has left - blocking failed. 
                break;
            }
            // if obstacle then change state
            if obstacles.contains(&(r, c)) {
                state = (state + 1) % pos.len();
                continue;
            }
            // check for a cycle
            if new_visited[r as usize][c as usize].contains(&(state as i32)) {
                cycle_found = true;
                break;
            }
            // move
            g_row = r;
            g_col = c;
        }

        if cycle_found {
            result += 1;
        }

        // remove the added obstacle 
        obstacles.remove(&(block_row, block_col));
    }

    
    println!("Result: {}", result);


    Ok(())
}