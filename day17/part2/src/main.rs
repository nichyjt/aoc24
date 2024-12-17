use std::{fs::File, io::{self, BufRead, BufReader}};

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
    let _ = res[1].parse::<i64>().unwrap();
    s = lines.next().unwrap()?;
    res = s.split(": ").collect();
    let _ = res[1].parse::<i64>().unwrap();
    
    s = lines.next().unwrap()?;
    res = s.split(": ").collect();
    let _ = res[1].parse::<i64>().unwrap();
    
    lines.next(); // process empty feed
    let program_string = &lines.next().unwrap()?[9..];
    let parts = program_string.split(",");
    let mut program = Vec::<i64>::new();
    for part in parts {
        let v = part.parse::<i64>().unwrap();
        program.push(v);
    }

    // Disassemble the program (sigh...)
    // while a != 0
    // b = a % 8
    // b = b ^ 1
    // c = a / (1<<b)
    // b = b ^ 5
    // a = a / 8
    // b = b ^ c
    // res.push(b%8)
    
    // Notice that the program runs for log_8(a) rounds.
    // So, we should search for values of a such that it iterates 16 times
    // let a_lower = i64::pow(8, 15);
    // let a_higher = i64::pow(8, 16);
    // limit search to |8^16-8^15| elems << i64 range

    // this is still too large. 
    // further notice that b depends on last 3 bits of a
    // and that we really only care about the last 3 bits of b
    
    // we search these to find the one that gives us the correct result
    let res = search(0, 0, &program);
    if res >= 0 {
        println!("Result: {}", res);
    }
    

    // not 236810900258
    // not 20567765534425
    Ok(())
}

fn search(round: i32, curr_a: i64, program: &Vec<i64>) -> i64 {
    if round == 16 {
        return curr_a;
    }
    println!("r={}, curr={},goal={}", round, curr_a, program[program.len()-1-round as usize]);
    for _a in 0..8 {
        let a = (curr_a << 3) + _a;
        let mut b: i64;
        let c: i64;
        b = a % 8;
        b ^= 1;
        c = a / (1 << b);
        b ^= 5;
        b ^= c;
        let res = b % 8;
        if program[program.len()-1-round as usize] == res {
            let res = search(round + 1, a, program);
            if res >= 0 { 
               return res;
            }
        }           
    }
    return -1;
}