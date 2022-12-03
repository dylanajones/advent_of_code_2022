use std::fs;
use std::collections::HashSet;

fn main() {
    let contents = fs::read_to_string("data/data.txt").expect("Should have been able to read the file");
    let split = contents.lines();
    let lines = split.collect::<Vec<&str>>();

    let mut sum = 0;
    let to_zero = u32::from('a');

    for line in lines {
        let item = find_same_char(line);
        if item.is_uppercase() {
            sum += u32::from(item.to_ascii_lowercase()) - to_zero + 27;
        } else {
            sum += u32::from(item) - to_zero + 1;

        }
    } 

    println!("Sum of Priorities: {}", sum);
}

fn find_same_char(line: &str) -> char {
    let mut items = HashSet::new();

    let middle_idx = (line.len() / 2) - 1;

    let first_half = &line[0..middle_idx+1];
    let second_half = &line[middle_idx+1..];

    for item in first_half.chars() {
        items.insert(item);
    }

    for item in second_half.chars() {
        if items.contains(&item) {
            return item;
        }
    }

    return 'a';
}