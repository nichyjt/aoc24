use std::{io::{BufReader, self, BufRead}, fs::File, collections::{HashSet, VecDeque}};

const WALL: char = '#';
const EMPTY: char = '.';
const BOX:char = 'O';
const BOXLEFT:char = '[';
const BOXRIGHT: char = ']';
const ROBOT:char = '@';

fn is_valid(row: usize, col: usize, max_rows: usize, max_cols: usize) -> bool{
    return row < max_rows && col < max_cols
}

fn print_warehouse(warehouse: &Vec<Vec::<char>>) {
    for row in warehouse {
        for val in row {
            print!("{}",val);
        }
        println!();
    }
}

fn main() -> io::Result<()> {
    let res = File::open("../input.in");
    let file = res.unwrap_or_else(|err| {
        eprintln!("Error loading file: {}", err);
        std::process::exit(1);
    });

    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    let mut curr_row = 0;
    let mut curr_col = 0;
    let mut warehouse: Vec<Vec<char>> = Vec::new();

    let mut _r = 0;
    while let Some(Ok(line)) = lines.next() {
        let mut _c = 0;
        let mut row = Vec::<char>::new();
        if line.trim().is_empty() {
            break;
        }
        for c in line.chars() {
            if c == BOX {
                row.push(BOXLEFT);
                row.push(BOXRIGHT);
            }else if c == ROBOT {
                curr_col = _c;
                curr_row = _r;
                row.push(ROBOT);
                row.push(EMPTY);
            }else{
                row.push(c);
                row.push(c);
            }
            _c += 2;
        }
        warehouse.push(row);
        _r += 1;
    }
    println!("Bot at {},{}", curr_row, curr_col);
    print_warehouse(&warehouse);
    
    let MAX_ROWS = warehouse.len();
    let MAX_COLS = warehouse[0].len();

    let mut moves: Vec<char> = Vec::new();
    for line in lines {
        moves.extend(line?.trim().chars());
    }
    let mut tick = 1;
    for mov in moves {
        if mov == '<' {
            let mut c = curr_col - 1;
            while is_valid(curr_row, c, MAX_ROWS, MAX_COLS) && 
            (warehouse[curr_row][c] == BOXLEFT || warehouse[curr_row][c] == BOXRIGHT) {
                c -= 1;
            }
            if warehouse[curr_row][c] == EMPTY {
                while c < curr_col {
                    warehouse[curr_row][c] = warehouse[curr_row][c + 1];
                    c += 1;
                }
                warehouse[curr_row][curr_col] = EMPTY;
                curr_col -= 1;
            }
        } else if mov == '>' {
            let mut c = curr_col + 1;
            while is_valid(curr_row, c, MAX_ROWS, MAX_COLS) && 
            (warehouse[curr_row][c] == BOXLEFT || warehouse[curr_row][c] == BOXRIGHT)  {
                c += 1;
            }
            if warehouse[curr_row][c] == EMPTY {
                while c > curr_col {
                    warehouse[curr_row][c] = warehouse[curr_row][c - 1];
                    c -= 1;
                }
                warehouse[curr_row][curr_col] = EMPTY;
                curr_col += 1;
            }
        } else if mov == '^' || mov == 'v' {

            let mut to_move = Vec::<(usize,usize)>::new();
            let mut queue = VecDeque::<(usize,usize)>::new();
            queue.push_back((curr_row,curr_col));
            let mut should_exe = true;
            while !queue.is_empty() {
                let (r,c) = queue.pop_back().unwrap();
                if warehouse[r][c] == WALL {
                    should_exe = false;
                    break;
                }else if warehouse[r][c] != EMPTY {
                    to_move.push((r,c));
                    let mut nr = r;
                    if mov == '^'{
                        nr -= 1;
                    } else{
                        nr += 1;
                    }
                    let nc = c;
                    queue.push_back((nr,nc));
                    if warehouse[nr][nc] == BOXLEFT {
                        queue.push_back((nr,nc+1));
                    }
                    if warehouse[nr][nc] == BOXRIGHT {
                        queue.push_back((nr,nc-1));
                    }
                }
            } //while
            if should_exe {

                let mut set = HashSet::<(usize,usize)>::new();
                to_move.reverse();
                for (r,c) in to_move {
                    println!("{},{}", r, c);
                    if !set.contains(&(r,c)) {
                        set.insert((r,c));
                        if mov == '^' {
                            let temp = warehouse[r][c];
                            warehouse[r][c] = warehouse[r-1][c];
                            warehouse[r-1][c] = temp;
                        }else{
                            let temp = warehouse[r][c];
                            warehouse[r][c] = warehouse[r+1][c];
                            warehouse[r+1][c] = temp;
                        }
                    }
                }
                if mov == '^' {
                    curr_row -= 1;
                }else{
                    curr_row +=1;
                }
            }
        }
        
        println!("{}: {}", tick, mov);
        tick += 1;
        print_warehouse(&warehouse)
    }
    println!("==========================");
    let mut score = 0;
    let mut _r = 0;
    for row in &warehouse {
        for val in row {
            print!("{}",val);
        }
        println!();
    }
    for row in warehouse {
        let mut _c = 0;
        for val in row {
            if val == BOXLEFT {
                score += 100 * _r + _c;
            }
           _c += 1;
        }
        _r += 1;
    }
    println!("Score: {}", score);
    Ok(())
}

