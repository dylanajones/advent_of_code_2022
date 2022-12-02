use std::fs;

struct Round {
    opp_choice: char,
    my_choice: char,
}

fn main() {
    let rounds = process_input();

    let mut total_score = 0;

    for round in rounds {
        total_score += score_round(round);
    }

    println!("{}", total_score);
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

fn score_play(round: Round) -> i32 {
    let mut score = 0;

    if round.my_choice == 'X' {
        score = 1;
    } else if round.my_choice == 'Y' {
        score = 2;
    } else if round.my_choice == 'Z' {
        score = 3;
    }

    return score;
}

fn score_round(round: Round) -> i32 {
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
    
    return score + score_play(round);
}