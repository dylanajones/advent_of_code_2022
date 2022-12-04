use std::fs;

struct Elf {
    start: i32,
    end: i32,
}

fn main() {
    let elf_pairs = read_data();

    part_1(&elf_pairs);
    part_2(&elf_pairs);
}

fn part_1(elves : &Vec<(Elf,Elf)>) {
    let mut count = 0;
    for pair in elves {
        if (pair.0.start <= pair.1.start && pair.0.end >= pair.1.end) || (pair.1.start <= pair.0.start && pair.1.end >= pair.0.end) {
            count += 1;
        }
    }

    println!("Count of fully overlapping pairs is {}", count);
}

fn part_2(elves : &Vec<(Elf,Elf)>) {
    let mut count = 0;
    for pair in elves {
        if (pair.0.start <= pair.1.start && pair.0.end >= pair.1.end) || 
           (pair.0.start <= pair.1.start && pair.0.end <= pair.1.end && pair.0.end >= pair.1.start) ||
           (pair.1.start <= pair.0.start && pair.1.end >= pair.0.end) ||
           (pair.1.start <= pair.0.start && pair.1.end <= pair.0.end && pair.1.end >= pair.0.start) {
            count += 1;
        }
    }

    println!("Count of overlapping pairs is {}", count);
}

fn read_data() -> Vec<(Elf,Elf)> {
    let contents = fs::read_to_string("data/data.txt").expect("Should have been able to read the file");
    let split = contents.lines();
    let lines = split.collect::<Vec<&str>>();

    let mut elf_pairs: Vec<(Elf,Elf)> = Vec::new();
    
    for line in lines {
        let elfs = line.split(",").collect::<Vec<&str>>();
        let sections_1 = elfs[0].split("-").collect::<Vec<&str>>();
        let sections_2 = elfs[1].split("-").collect::<Vec<&str>>();

        let elf_1 = Elf {
            start: sections_1[0].parse().unwrap(),
            end: sections_1[1].parse().unwrap(),
        };
        let elf_2 = Elf {
            start: sections_2[0].parse().unwrap(),
            end: sections_2[1].parse().unwrap(),
        };
        elf_pairs.push((elf_1, elf_2));
    }

    return elf_pairs;
}
