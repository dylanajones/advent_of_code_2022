use std::fs;

fn main() {
    let contents = fs::read_to_string("data/data.txt").expect("Should have been able to read the file");

    let split = contents.lines();
    
    let lines = split.collect::<Vec<&str>>();

    let mut max_cals = 0.0;
    let mut cur_cals = 0.0;
    let mut max_id = 0;
    let mut cur_id = 0;

    // Loop through the lines and add up calories
    for line in lines {
        if !line.is_empty(){
            let cals = line.parse::<f32>().unwrap();
            cur_cals += cals;
        } else {
            if cur_cals > max_cals {
                max_cals = cur_cals;
                max_id = cur_id;
            }

            cur_cals = 0.0;
            cur_id += 1;
        }
    }

    // Make sure we don't miss the last elf
    if cur_cals > max_cals {
        max_cals = cur_cals;
        max_id = cur_id;
    }

    println!("{}", max_id);
    println!("{}", max_cals);
}
