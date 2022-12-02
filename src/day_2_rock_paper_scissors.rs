const DRAW: i32 = 3;
const WIN: i32 = 6;
const LOSS: i32 = 0;

fn rps_action_to_value(action: char) -> i32 {
    match action {
        'A' | 'X' => 1, // rock
        'B' | 'Y' => 2, // paper
        'C' | 'Z' => 3, // scissors
        _ => panic!("Unknown action detected!")
    }
}

fn calculate_round_score(round_value_1: i32, round_value_2: i32) -> i32 {
    let diff = round_value_2 - round_value_1;
    round_value_2 + match diff {
        0 => DRAW,
        1 | -2 => WIN,
        _ => LOSS
    }
}

pub fn solve_rock_paper_scissors(strategy_guide: Vec<Vec<char>>) -> i32 {
    strategy_guide
        .into_iter()
        .map(|round| calculate_round_score(rps_action_to_value(round[0]), rps_action_to_value(round[1])))
        .fold(0, |acc, round_score| acc + round_score)
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

    fn input_to_vector_2d_char(input: String) -> Vec<Vec<char>> {
        input
            .split("\n")
            .map(|row| row
                .split_whitespace()
                .map(|item| item.parse::<char>().unwrap())
                .collect()
            )
            .collect()
    }

    fn get_input_from_file(path: &str) -> String {
        fs::read_to_string(path)
            .expect("Should have been able to read the file")
    }

    #[test]
    fn it_can_find_most_calories_for_example_puzzle_input() {
        let input = get_input_from_file("./data/day_2_rock_paper_scissors_example_puzzle_input.txt");
        let strategy_guide = input_to_vector_2d_char(input);

        assert_eq!(solve_rock_paper_scissors(strategy_guide), 15);
    }

    #[test]
    fn it_can_find_most_calories_for_puzzle_input() {
        let input = get_input_from_file("./data/day_2_rock_paper_scissors_puzzle_input.txt");
        let strategy_guide = input_to_vector_2d_char(input);

        assert_eq!(solve_rock_paper_scissors(strategy_guide), 15572);
    }
}