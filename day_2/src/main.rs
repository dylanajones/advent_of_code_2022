use std::fs;

struct Round {
    opp_choice: char,
    my_choice: char,
}

fn main() {
    let rounds = process_input();

    let mut total_score_part_1 = 0;
    let mut total_score_part_2 = 0;

    for round in rounds {
        total_score_part_1 += score_round_part_1(&round);
        total_score_part_2 += score_round_part_2(&round);
    }

    println!("Part 1: {}", total_score_part_1);
    println!("Part 2: {}", total_score_part_2);
}

fn process_input() -> Vec<Round> {
    let mut rounds: Vec<Round> = Vec::new();

    let contents = fs::read_to_string("data/data.txt").expect("Should have been able to read the file");
    let split = contents.lines();
    let lines = split.collect::<Vec<&str>>();

    for line in lines {
        let round = Round {
            opp_choice : line.chars().nth(0).unwrap(),
            my_choice : line.chars().nth(2).unwrap(),
        };
        rounds.push(round);
    }
    
    return rounds;
}

fn score_play(my_play: char) -> i32 {
    let mut score = 0;

    if my_play == 'X' {
        score = 1;
    } else if my_play == 'Y' {
        score = 2;
    } else if my_play == 'Z' {
        score = 3;
    }

    return score;
}

fn score_round_part_1(round: &Round) -> i32 {
    let mut score = 0;

    if round.my_choice == 'X' {
        if round.opp_choice == 'A' {
            score = 3;
        } else if round.opp_choice == 'B' {
            score = 0;
        } else if round.opp_choice == 'C' {
            score = 6;
        }
    } else if round.my_choice == 'Y' {
        if round.opp_choice == 'A' {
            score = 6;
        } else if round.opp_choice == 'B' {
            score = 3;
        } else if round.opp_choice == 'C' {
            score = 0;
        }
    } else if round.my_choice == 'Z' {
        if round.opp_choice == 'A' {
            score = 0;
        } else if round.opp_choice == 'B' {
            score = 6;
        } else if round.opp_choice == 'C' {
            score = 3;
        }
    }
    
    return score + score_play(round.my_choice);
}

fn score_round_part_2(round: &Round) -> i32 {
    let mut score = 0;
    let mut play = 'X';

    if round.my_choice == 'X' {
        score = 0;
        if round.opp_choice == 'A' {
            play = 'Z';
        } else if round.opp_choice == 'B' {
            play = 'X';
        } else if round.opp_choice == 'C' {
            play = 'Y';
        }
    } else if round.my_choice == 'Y' {
        score = 3;
        if round.opp_choice == 'A' {
            play = 'X';
        } else if round.opp_choice == 'B' {
            play = 'Y';
        } else if round.opp_choice == 'C' {
            play = 'Z';
        }
    } else if round.my_choice == 'Z' {
        score = 6;
        if round.opp_choice == 'A' {
            play = 'Y';
        } else if round.opp_choice == 'B' {
            play = 'Z';
        } else if round.opp_choice == 'C' {
            play = 'X';
        }
    }
    
    return score + score_play(play);
}