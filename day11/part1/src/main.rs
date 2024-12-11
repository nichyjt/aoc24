use std::{io::{self, BufReader, BufRead}, fs::File};

fn main() -> io::Result<()> {
    let res = File::open("../input.in");
    let file = res.unwrap_or_else(|err| {
        eprintln!("Error loading file: {}", err);
        std::process::exit(1);
    });

    let reader = BufReader::new(file);
    let mut vals: Vec<String> = Vec::new();
    for line in reader.lines() {
        for word in line?.split(" ") {
            vals.push(word.to_string());
        }
    }

    // process 25 times
    // println!("Initial: {:?}", vals);
    
    for _i in 1..=25 {
        let mut new_list = Vec::<String>::new();
        for val in vals.iter_mut() {
            if val == "0" {
                new_list.push("1".to_string());
            }else if val.len() % 2 == 0 {
                let half_len = val.len() / 2;
                new_list.push(val[..half_len].to_string().parse::<i64>().unwrap().to_string());
                new_list.push(val[half_len..].to_string().parse::<i64>().unwrap().to_string());
            }else{
                let mut num: i64 = val.parse().unwrap();
                num *= 2024;
                new_list.push(num.to_string());
            }
        }
        vals = new_list;
        // println!("{}: {:?}", _i, vals);
    }
    println!("Num Stones: {}", vals.len());
    Ok(())
}