use std::{fs::File, io::{self, BufRead, BufReader}};

const ADV: i64 = 0;
const BXL: i64 = 1;
const BST: i64 = 2;
const JNZ: i64 = 3;
const BXC: i64 = 4;
const OUT: i64 = 5;
const BDV: i64 = 6;
const CDV: i64 = 7;

fn get_operand(operand: i64, ra: i64, rb: i64, rc: i64) -> i64 {
    if operand <= 3 {
        return operand;
    } else if operand == 4 {
        return ra;
    } else if operand == 5 {
        return rb;
    } else if operand == 6 {
        return rc;
    }
    // operand > 6
    assert!(false); // throw an error
    return operand;
}

fn main() -> io::Result<()> {
    let res = File::open("../input.in");
    let file = res.unwrap_or_else(|err| {
        eprintln!("Error loading file: {}", err);
        std::process::exit(1);
    });

    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    let mut s= lines.next().unwrap()?;
    let mut res: Vec<&str> = s.split(": ").collect();
    let mut reg_a = res[1].parse::<i64>().unwrap();
    s = lines.next().unwrap()?;
    res = s.split(": ").collect();
    let mut reg_b = res[1].parse::<i64>().unwrap();
    
    s = lines.next().unwrap()?;
    res = s.split(": ").collect();
    let mut reg_c = res[1].parse::<i64>().unwrap();
    
    lines.next(); // process empty feed
    let program_string = &lines.next().unwrap()?[9..];
    let parts = program_string.split(",");
    let mut program = Vec::<i64>::new();
    for part in parts {
        let v = part.parse::<i64>().unwrap();
        program.push(v);
    }

    // Now, simulate the program
    println!("Simulating... ra={}, rb={}, rc={}", reg_a, reg_b, reg_c);
    let mut ip: i64 = 0; // instruction pointer
    let mut print_buf= Vec::<i64>::new();

    while ip >= 0 && ip+1 < program.len() as i64 {
        let opcode = program[ip as usize];
        let literal = program[(ip + 1) as usize];
        // println!("OP: {},{}", opcode, literal);
        let combo = get_operand(literal, reg_a, reg_b, reg_c);
        // perform the operation
        match opcode {
            ADV => reg_a = reg_a / (1<<combo),
            BXL => reg_b = reg_b^literal,
            BST => reg_b = combo % 8,
            JNZ => {
                if reg_a != 0 {
                    ip = literal - 2;
                }
            },
            BXC => reg_b = reg_b ^ reg_c,
            OUT => print_buf.push(combo % 8),
            BDV => reg_b = reg_a / (1<<combo),
            CDV => reg_c = reg_a / (1<<combo),
            _ => panic!("invalid opcode")
        }
        ip += 2;
    }

    println!();
    println!("Output: ");
    for(i, val) in print_buf.iter().enumerate() {
        print!("{}", val);
        if i < print_buf.len() - 1 {
            print!(",");
        }
    }
    println!();

    Ok(())
}