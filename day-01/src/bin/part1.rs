use std::fs;
fn main() {
    let contents = fs::read_to_string("part1.txt").expect("File read error");
    let mut col1 = Vec::<i32>::new();
    let mut col2 = Vec::<i32>::new();
    for line in contents.lines() {
        let mut numbers = line.split_ascii_whitespace();
        col1.push(numbers.next().unwrap().parse().expect("Invalid number"));
        col2.push(numbers.next().unwrap().parse().expect("Invalid number"));
    }
    col1.sort();
    col2.sort();
    let mut total_distance = 0;
    for i in 0..col1.len() {
        total_distance += col1[i].abs_diff(col2[i]);
    }
    println!("The result is: {}", total_distance);
}
