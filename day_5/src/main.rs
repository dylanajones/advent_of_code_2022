use std::fs;
use std::collections::VecDeque;

fn main() {
    let file_name = "data/data.txt";
    let mut stacks = read_stack_data(file_name);
    println!("Starting Stacks:");
    print_stacks(&stacks);
    // stacks = process_stack_moves(stacks, file_name);
    stacks = process_stack_moves_2(stacks, file_name);
    println!("Ending Stacks:");
    print_stacks(&stacks);
}

fn read_stack_data(file_name: &str) -> Vec<VecDeque<char>>{
    let contents = fs::read_to_string(file_name).expect("Should have been able to read the file");
    let split = contents.lines();
    let lines = split.collect::<Vec<&str>>();

    let mut stacks : Vec<VecDeque<char>> = Vec::new();

    let num_stacks = ((lines[0].len() as f32) / 4.0).round() as i32;

    for _i in 0..num_stacks {
        let empty_stack : VecDeque<char> = VecDeque::new();
        stacks.push(empty_stack);
    }

    for line in lines {
        if line.chars().nth(1).unwrap() != '1' {
            for idx in (1..line.len()).step_by(4) {
                if line.chars().nth(idx).unwrap() != ' ' {
                    stacks[idx / 4].push_back(line.chars().nth(idx).unwrap().clone());
                }
            }        
        } else {
            break;
        }
    }

    return stacks
}

fn process_stack_moves(mut stacks: Vec<VecDeque<char>>, file_name: &str) ->  Vec<VecDeque<char>>{
    let contents = fs::read_to_string(file_name).expect("Should have been able to read the file");
    let split = contents.lines();
    let lines = split.collect::<Vec<&str>>();

    for line in lines {
        if line.is_empty() {
            continue;
        } else if line.chars().nth(0).unwrap() != 'm' {
            continue;
        } else {
            let moves = line.split(" ").collect::<Vec<&str>>();
            let num_move = moves[1].parse::<i32>().unwrap();
            let from_stack = moves[3].parse::<i32>().unwrap() - 1;
            let to_stack = moves[5].parse::<i32>().unwrap() - 1;

            for _i in 0..num_move {
                let item_to_add = stacks[from_stack as usize].pop_front().unwrap();
                stacks[to_stack as usize].push_front(item_to_add);
            }
        }
    }
    
    return stacks
}

fn process_stack_moves_2(mut stacks: Vec<VecDeque<char>>, file_name: &str) ->  Vec<VecDeque<char>>{
    let contents = fs::read_to_string(file_name).expect("Should have been able to read the file");
    let split = contents.lines();
    let lines = split.collect::<Vec<&str>>();

    for line in lines {
        if line.is_empty() {
            continue;
        } else if line.chars().nth(0).unwrap() != 'm' {
            continue;
        } else {
            let moves = line.split(" ").collect::<Vec<&str>>();
            let num_move = moves[1].parse::<i32>().unwrap();
            let from_stack = moves[3].parse::<i32>().unwrap() - 1;
            let to_stack = moves[5].parse::<i32>().unwrap() - 1;

            let mut temp_queue: VecDeque<char> = VecDeque::new();

            for _i in 0..num_move {
                let item_to_add = stacks[from_stack as usize].pop_front().unwrap();
                temp_queue.push_front(item_to_add);
            }

            while !temp_queue.is_empty() {
                stacks[to_stack as usize].push_front(temp_queue.pop_front().unwrap());
            }
        }
    }
    
    return stacks
}

fn print_stacks(stacks: &Vec<VecDeque<char>>) {
    for stack in stacks {
        let mut item_list = String::from("");
        for item in stack {
            item_list.push_str(&item.to_string());
        }

        println!("{}", item_list);
    }
}