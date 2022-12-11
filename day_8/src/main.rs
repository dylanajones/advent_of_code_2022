use ndarray::Array2;
use std::fs;

fn main() {
    let file_name = "data/data.txt";
    let contents = fs::read_to_string(file_name).expect("Should have been able to read the file");
    let split = contents.lines();
    let lines = split.collect::<Vec<&str>>();

    let num_rows = lines.len();
    let num_cols = lines[0].len();

    let mut height_map = Array2::zeros((num_rows, num_cols));
    
    for (i, line) in lines.iter().enumerate() {
        let chars = line.chars().collect::<Vec<char>>();
        for (j, s_char) in chars.iter().enumerate() {
            height_map[[i,j]] = s_char.to_digit(10).unwrap();
        }
    }

    let mut i = 0;
    let mut j = 0;
    let mut sum_viz = 0;
    let mut add = 0;

    let mut viz_up = true;
    let mut viz_down = true;
    let mut viz_left = true;
    let mut viz_right = true;

    let mut up_count = 0;
    let mut down_count = 0;
    let mut right_count = 0;
    let mut left_count = 0;

    let mut score = 0;

    for row in height_map.rows() {
        for element in row {
            add = 0;
            
            up_count = i;
            for x in (0..i).rev() {
                if height_map[[x,j]] >= *element {
                    viz_up = false;
                    up_count = i - x;
                    break;
                }
            }

            down_count = num_rows - i - 1;
            for x in i+1..num_rows {
                if height_map[[x,j]] >= *element {
                    viz_down = false;
                    down_count = x - i;
                    break;
                }
            }

            left_count = j;
            for x in (0..j).rev() {
                if height_map[[i,x]] >= *element {
                    viz_left = false;
                    left_count = j - x;
                    break;
                }
            }

            right_count = num_cols - j - 1;
            for x in j+1..num_cols {
                if height_map[[i,x]] >= *element {
                    viz_right = false;
                    right_count = x - j;
                    break;
                }
            }
            
            if viz_up || viz_down || viz_left || viz_right {
                add = 1;
            }

            // println!("Row: {}, Col: {}, Up Count: {}, Down Count: {}, Left Count: {}, Right Count: {}", i,j,up_count,down_count,left_count,right_count);

            let temp_score = up_count * down_count * left_count * right_count;

            if temp_score > score {
                score = temp_score;
            }
            
            viz_up = true;
            viz_down = true;
            viz_left = true;
            viz_right = true;

            up_count = 0;
            down_count = 0;
            right_count = 0;
            left_count = 0;
            
            
            sum_viz += add;
            j += 1;
        }
        j = 0;
        i += 1;
    }
    
    println!("Sum vis: {}", sum_viz);
    println!("Max score: {}", score);
}
