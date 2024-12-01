use std::{collections::HashMap, fs};
fn main() {
    let contents = fs::read_to_string("part1.txt").expect("File read error");
    let mut left = Vec::<i32>::new();
    let mut right_map = HashMap::<i32, i32>::new();
    for line in contents.lines() {
        let mut numbers = line.split_ascii_whitespace();
        left.push(numbers.next().unwrap().parse().expect("Invalid number"));
        right_map
            .entry(numbers.next().unwrap().parse().expect("Invalid number"))
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }
    let mut similarity_score = 0;
    for i in 0..left.len() {
        similarity_score += right_map.get(&left[i]).unwrap_or(&0) * left[i];
    }
    println!("The result is: {}", similarity_score);
}
