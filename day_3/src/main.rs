use std::fs;
use std::collections::HashSet;

fn main() {
    part_1();
    part_2();
}

fn part_1() {
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

    println!("Sum of Priorities (Part 1): {}", sum);
}

fn part_2() {
    let contents = fs::read_to_string("data/data.txt").expect("Should have been able to read the file");
    let split = contents.lines();
    let lines = split.collect::<Vec<&str>>();

    let mut sum = 0;
    let to_zero = u32::from('a');

    let mut set_vector: Vec<HashSet<char>> = Vec::new();
    set_vector.push(HashSet::new());
    set_vector.push(HashSet::new());
    set_vector.push(HashSet::new());

    for (i, line) in lines.iter().enumerate() {
        
        if i % 3 == 0  && i != 0{

            for x in set_vector[0].intersection(&set_vector[1]) {
                if set_vector[2].contains(&x) {
                    if x.is_uppercase() {
                        sum += u32::from(x.to_ascii_lowercase()) - to_zero + 27;
                    } else {
                        sum += u32::from(x.to_ascii_lowercase()) - to_zero + 1;
            
                    }
                }
            }
            
            for item in set_vector.iter_mut() {
                item.clear();
            }
        }

        for x in line.chars() {
            set_vector[i % 3].insert(x);
        }
    }

    for x in set_vector[0].intersection(&set_vector[1]) {
        if set_vector[2].contains(&x) {
            if x.is_uppercase() {
                sum += u32::from(x.to_ascii_lowercase()) - to_zero + 27;
            } else {
                sum += u32::from(x.to_ascii_lowercase()) - to_zero + 1;
    
            }
        }
    }

    println!("Sum of Priorities (Part 2): {}", sum);
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