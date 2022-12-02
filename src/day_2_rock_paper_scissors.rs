const DRAW: i32 = 3;
const WIN: i32 = 6;

pub fn rps_action_to_integer(action: char) -> i32 {
    match action {
        'A' | 'X' => 1, // rock
        'B' | 'Y' => 2, // paper
        'C' | 'Z' => 3, // scissors
        _ => panic!("Unknown action detected!")
    }
}

pub fn calculate_round_score(round: Vec<i32>) -> i32 {
    let diff = round[1] - round[0];
    match diff {
        0 => round[1] + DRAW,
        1 | -2 => round[1] + WIN,
        _ => round[1]
    }
}

pub fn solve_rock_paper_scissors(strategy_guide: Vec<Vec<char>>) -> i32 {
    strategy_guide
        .into_iter()
        .map(|round| vec![rps_action_to_integer(round[0]), rps_action_to_integer(round[1])])
        .map(|round| calculate_round_score(round))
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