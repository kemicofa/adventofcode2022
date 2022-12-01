pub fn calorie_counting(elves: Vec<Vec<i32>>) -> i32 {
    elves
        .iter()
        .map(|elf| elf.iter().fold(0, |acc, calorie| acc + calorie ))
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

    fn input_to_vector_2d_i32(input: String) -> Vec<Vec<i32>> {
        input
            .split("\n\n")
            .map(|row| row
                .split('\n')
                .map(|item| item.parse::<i32>().unwrap())
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
        let input = get_input_from_file("./data/day_1_calorie_counting_example_puzzle_input.txt");
        let elves = input_to_vector_2d_i32(input);

        assert_eq!(calorie_counting(elves), 24000);
    }

    #[test]
    fn it_can_find_most_calories_for_puzzle_input() {
        let input = get_input_from_file("./data/day_1_calorie_counting_puzzle_input.txt");
        let elves = input_to_vector_2d_i32(input);

        assert_eq!(calorie_counting(elves), 71124);
    }
}