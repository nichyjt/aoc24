use std::{fs::File, io::{self, BufRead}};

fn main() -> io::Result<()> {
    let res = File::open("../input.in");
    // Results need to be unwrapped. Prefer to use ? operator in the future
    let file = res.unwrap_or_else(|err| {
        eprintln!("Error loading file: {}", err);
        std::process::exit(1);
    });

    let reader = io::BufReader::new(file);
    let mut reports = Vec::<Vec::<i32>>::new();

    for line in reader.lines() {
        reports.resize(reports.len() + 1, Vec::<i32>::new());
        let report_str = line?;
        let index = reports.len() - 1;
        report_str.split(" ").for_each(|s| {
            reports[index].push(s.parse::<i32>().unwrap());
        });      
    }

    // Solve the edit one problem
    let mut num_safe_reports = 0;
    for report in reports {
        for (i, _) in report.iter().enumerate() {
            // Choose one index to skip
            let mut is_ok = true;
            let mut prev_diff = 0;
            let mut prev = -1;
            for (j, curr) in report.iter().enumerate() {
                if i != j {
                    if prev == -1 {
                        prev = *curr;
                    } else{
                        let curr_diff = curr - prev;

                        if curr_diff == 0 || curr_diff.abs() > 3 {
                            is_ok = false;
                            break;
                        }

                        if prev_diff != 0 {
                            if curr_diff * prev_diff < 0 {
                                // polarity here failed
                                is_ok = false;
                                break;
                            }
                        }

                        prev_diff = curr_diff;
                        prev = *curr;
                    }
                }
            }

            if is_ok {
                num_safe_reports += 1;
                break;
            }
        }

    }
    println!("Safe reports: {}", num_safe_reports);
    Ok(())
}

