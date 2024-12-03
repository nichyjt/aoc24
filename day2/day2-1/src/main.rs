use std::{fs::File, io::{self, BufRead}};

fn main() -> io::Result<()> {
    let res = File::open("../input.in");
    // Results need to be unwrapped. Prefer to use ? operator in the future
    let file = res.unwrap_or_else(|err| {
        eprintln!("Error loading file: {}", err);
        std::process::exit(1);
    });

    let reader = io::BufReader::new(file);
    let mut num_safe_reports = 0;
    
    for line in reader.lines() {
        let report_str = line?;
        // handle each report string
        let mut prev: i32 = -1; // invariant: all inputs are >= 0 so this works
        let mut prev_diff = 0; // track the polarity
        let mut is_ok = true;
        // within a report, find the report strings
        let strs = report_str.split(" ");
        for str in strs {
            let curr = str.parse::<i32>().unwrap();
            if prev == -1 {
                prev = curr;
            } else {
                let curr_diff = curr - prev;
                // magnitude check 
                if curr_diff == 0 || curr_diff.abs() > 3 {
                    is_ok = false;
                    break;
                }
                // do a polarity check 
                if prev_diff != 0 {
                    if curr_diff * prev_diff < 0 {
                        // polarity here failed
                        is_ok = false;
                        break;
                    }
                }
                prev_diff = curr_diff;
                prev = curr;
            }

        }
        if is_ok {
            num_safe_reports += 1;
        }
    }
    println!("Safe reports: {}", num_safe_reports);

    Ok(())
}
