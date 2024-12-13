use std::{io::{BufReader, self, BufRead}, fs::File};

struct Machine {
    ax: i64,
    ay: i64,
    bx: i64,
    by: i64,
    px: i64,
    py: i64
}

fn main() -> io::Result<()> {
    let res = File::open("../input.in");
    let file = res.unwrap_or_else(|err| {
        eprintln!("Error loading file: {}", err);
        std::process::exit(1);
    });

    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    let mut machines: Vec<Machine> = Vec::new();
    while let (Some(btn_a), Some(btn_b), Some(prize)) = (lines.next(), lines.next(), lines.next()) {
        let btn_a = btn_a?;
        let btn_b = btn_b?;
        let prize = prize?;
        let mut a_parts = btn_a[10..].split(",");
        let mut b_parts = btn_b[10..].split(",");
        let mut p_parts = prize[7..].split(",");
        let ax = a_parts
            .next()
            .and_then(|part| part.split("+").nth(1))
            .and_then(|num| num.parse::<i64>().ok()).unwrap();
        let ay = a_parts
            .next()
            .and_then(|part| part.split("+").nth(1))
            .and_then(|num| num.parse::<i64>().ok()).unwrap();
        let bx = b_parts
            .next()
            .and_then(|part| part.split("+").nth(1))
            .and_then(|num| num.parse::<i64>().ok()).unwrap();
        let by = b_parts
            .next()
            .and_then(|part| part.split("+").nth(1))
            .and_then(|num| num.parse::<i64>().ok()).unwrap();
        let px = p_parts
            .next()
            .and_then(|part| part.split("=").nth(1))
            .and_then(|num| num.parse::<i64>().ok()).unwrap();
        let py = p_parts
            .next()
            .and_then(|part| part.split("=").nth(1))
            .and_then(|num| num.parse::<i64>().ok()).unwrap();
        let machine = Machine {
            ax, ay, bx, by, px, py
        };
        machines.push(machine);
        lines.next();
    }

    let mut sum = 0;
    let mut num_valid = 0;

    for m in machines {
        let det = m.ax * m.by - m.bx * m.ay;

        if det == 0 {
            continue;
        }

        let na_num = m.by * (m.px + 10000000000000) - m.bx * (m.py + 10000000000000);
        let nb_num = -m.ay * (m.px+ 10000000000000) + m.ax * (m.py + 10000000000000);

        if na_num % det != 0 || nb_num % det != 0 {
            // println!("no integer solution");
            continue;
        }

        let na = na_num / det;
        let nb = nb_num / det;
        if na < 0 || nb < 0 { // || na > 100 || nb > 100 {
            continue;
        }

        println!("na = {}, nb = {}", na, nb);
        sum += 3 * na + nb;
        num_valid += 1;
    }
    
    println!("Result: {}, {}", sum, num_valid);
    

    Ok(())
}

