use std::fs;
use std::collections::VecDeque;

fn main() {
    let file_name = "data/data.txt";
    let data = read_data(file_name);
    println!("Part 1:");
    find_non_repeat_seq(4, &data);
    println!("Part 2:");
    find_non_repeat_seq(14, &data);
}

fn read_data(file_name: &str) -> Vec<char> {
    let contents = fs::read_to_string(file_name).expect("Should have been able to read the file");
    let split = contents.lines();
    let lines = split.collect::<Vec<&str>>();

    return lines[0].chars().collect::<Vec<char>>();
}

fn find_non_repeat_seq(length: i32, data: &Vec<char>) {
    let mut char_list: VecDeque<char> = VecDeque::new();
    
    let mut count = 0;

    for character in data {
        if char_list.contains(character) {
            let mut front_char = char_list.pop_front();
            while front_char.unwrap() != *character {
                front_char = char_list.pop_front();
            }
        }

        char_list.push_back(*character);
        count += 1;

        
        
        if char_list.len() as i32 == length {
            break;
        }
    }
    println!("Count is: {}", count);
}
