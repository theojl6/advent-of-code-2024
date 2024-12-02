use std::fs;
fn main() {
    let content = fs::read_to_string("input.txt").expect("Cannot read file");
    let mut safe_reports = 0;
    for line in content.lines() {
        let report: Vec<i32> = line
            .split_ascii_whitespace()
            .map(|n| n.parse().expect("Invalid number"))
            .collect();
        let mut is_increasing = false;
        let mut is_valid = true;
        for i in 1..report.len() {
            if !is_valid {
                continue;
            }
            let prev = report[i - 1];
            let curr = report[i];
            let diff = prev.abs_diff(curr);
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
        }
    }
    println!("The number of safe reports is: {}", safe_reports);
}
