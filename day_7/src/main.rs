use std::fs;

mod tree;
pub use crate::tree::tree_impl::Tree;
pub use crate::tree::tree_impl::Node;

fn main() {
    let file_name = "data/data.txt";
    let contents = fs::read_to_string(file_name).expect("Should have been able to read the file");
    let split = contents.lines();
    let lines = split.collect::<Vec<&str>>();

    let mut tree: Tree<String> = Tree::default();
    tree.add("/".to_string(), None);

    let mut current_folder: Option<usize> = None;
    
    for line in lines {
        let input = line.split(" ").collect::<Vec<&str>>();
        
        if input[0] == "$"
        {
            if input[1] == "cd" {
                if current_folder.is_some() {
                    if input[2] == ".." {
                        current_folder = tree.get_parent(current_folder.unwrap())
                    } else {
                        current_folder = tree.find_child(current_folder.unwrap(), input[2].to_string())
                    }
                } else {
                    current_folder = Some(0);
                }
            }
        } else if input[0] == "dir" {
            tree.add(input[1].to_string(), current_folder)
        } else {
            tree.add_file(input[1].to_string(), current_folder.unwrap(), input[0].parse::<i32>().unwrap())
        }
    }

    let mut sum = 0;
    
    // Part 1
    for idx in 0..tree.size() {
        if tree.is_folder(idx) {
            let f_size = tree.compute_size(idx);
            if f_size <= 100000 {
                sum += f_size;
            }
        }
    }

    println!("Small Files Sum: {}", sum);

    // Part 2
    let total_space = 70000000;
    let needed_space = 30000000;
    let used_space = tree.compute_size(0);
    let unused_space = total_space - used_space;

    let mut delete_size = total_space;

    for idx in 0..tree.size() {
        if tree.is_folder(idx) {
            let f_size = tree.compute_size(idx);
            if unused_space + f_size >= needed_space && f_size < delete_size {
                delete_size = f_size;
            }
        }
    }

    println!("Delete Size: {}", delete_size);
}