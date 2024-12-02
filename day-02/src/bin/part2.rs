use std::fs;
fn main() {
    let content = fs::read_to_string("input.txt").expect("Cannot read file");
    let mut safe_reports = 0;
    for line in content.lines() {
        let report: Vec<i32> = line
            .split_ascii_whitespace()
            .map(|n| n.parse().expect("Invalid number"))
            .collect();
        let mut valid_report = false;
        for r in 0..report.len() {
            if valid_report {
                continue;
            }
            let tolerated_report: Vec<&i32> = if r < report.len() - 1 {
                report[..r]
                    .into_iter()
                    .chain(report[r + 1..].into_iter())
                    .collect()
            } else {
                report[..r].into_iter().collect()
            };
            let mut is_increasing = false;
            let mut is_valid = true;
            for i in 1..tolerated_report.len() {
                if !is_valid {
                    continue;
                }
                let prev = tolerated_report[i - 1];
                let curr = tolerated_report[i];
                let diff = prev.abs_diff(*curr);
                if diff < 1 || diff > 3 {
                    is_valid = false;
                }
                if i == 1 {
                    is_increasing = curr - prev > 0;
                } else {
                    match is_increasing {
                        true => {
                            if curr - prev < 0 {
                                is_valid = false;
                            }
                        }
                        false => {
                            if curr - prev > 0 {
                                is_valid = false;
                            }
                        }
                    }
                }
            }
            if is_valid {
                safe_reports += 1;
                valid_report = true;
            }
        }
    }
    println!("The number of safe reports is: {}", safe_reports);
}
