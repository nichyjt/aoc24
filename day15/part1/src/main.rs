use std::{io::{BufReader, self, BufRead}, fs::File};

// const WALL: char = '#';
const EMPTY: char = '.';
const BOX:char = 'O';
const ROBOT:char = '@';

fn is_valid(row: usize, col: usize, max_rows: usize, max_cols: usize) -> bool{
    return row < max_rows && col < max_cols
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
    while let Some(Ok(line)) = lines.next() {
        if line.trim().is_empty() {
            break;
        }
        let bot_found = line.find(ROBOT);
        if let Some(index) = bot_found {
            curr_row = warehouse.len();
            curr_col = index;
        }
        warehouse.push(line.chars().collect());
    }
    println!("Bot at {},{}", curr_row, curr_col);
    let MAX_ROWS = warehouse.len();
    let MAX_COLS = warehouse[0].len();

    let mut moves: Vec<char> = Vec::new();
    for line in lines {
        moves.extend(line?.trim().chars());
    }


    for mov in moves {
        if mov == '<' {
            let mut c = curr_col - 1;
            while is_valid(curr_row, c, MAX_ROWS, MAX_COLS) && warehouse[curr_row][c] == BOX {
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
            while is_valid(curr_row, c, MAX_ROWS, MAX_COLS) && warehouse[curr_row][c] == BOX {
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
        } else if mov == '^' {
            let mut r = curr_row - 1;
            while is_valid(r, curr_col, MAX_ROWS, MAX_COLS) && warehouse[r][curr_col] == BOX {
                r -= 1;
            }
            if warehouse[r][curr_col] == EMPTY {
                while r < curr_row {
                    warehouse[r][curr_col] = warehouse[r + 1][curr_col];
                    r += 1;
                }
                warehouse[curr_row][curr_col] = EMPTY;
                curr_row -= 1;
            }
        } else if mov == 'v' {
            let mut r = curr_row + 1;
            while is_valid(r, curr_col, MAX_ROWS, MAX_COLS) && warehouse[r][curr_col] == BOX {
                r += 1;
            }
            if warehouse[r][curr_col] == EMPTY {
                while r > curr_row {
                    warehouse[r][curr_col] = warehouse[r - 1][curr_col];
                    r -= 1;
                }
                warehouse[curr_row][curr_col] = EMPTY;
                curr_row += 1;
            }
        }
    }
    
    let mut score = 0;
    let mut r = 0;
    for row in warehouse {
        let mut c = 0;
        for val in row {
            print!("{}",val);
            if val == BOX {
                score += 100 * r + c;
            }
            c += 1;
        }
        println!();
        r += 1;
    }
    println!("Score: {}", score);
    Ok(())
}
