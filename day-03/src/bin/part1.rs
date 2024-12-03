use std::fs;
fn main() {
    let content = fs::read_to_string("input.txt").expect("Cannot read file");
    let chars: Vec<char> = content.chars().collect();
    let mut i = 0;
    let mut result = 0;
    while i < chars.len() {
        if let Some((x, y)) = walk(&mut i, &chars) {
            println!("Found x: {}, y: {}", x, y);
            result += x * y;
        } else {
            i += 1;
        }
    }
    println!("The result of multiplications is: {}", result);
}

fn walk(i: &mut usize, chars: &Vec<char>) -> Option<(i32, i32)> {
    let mut x = 0;
    let mut y = 0;
    for c in ['m', 'u', 'l', '(', 'x', ',', 'y', ')'] {
        if c == chars[*i] {
            *i += 1;
        } else if c == 'x' && chars[*i].is_digit(10) {
            let mut x_str = vec![];
            while chars[*i].is_digit(10) {
                x_str.push(chars[*i]);
                *i += 1;
            }
            x = x_str.iter().collect::<String>().parse().unwrap();
        } else if c == 'y' && chars[*i].is_digit(10) {
            let mut y_str = vec![];
            while chars[*i].is_digit(10) {
                y_str.push(chars[*i]);
                *i += 1;
            }
            y = y_str.iter().collect::<String>().parse().unwrap();
        } else {
            return None;
        }
    }
    return Some((x, y));
}
