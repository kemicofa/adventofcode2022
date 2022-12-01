pub fn calorie_counting(elves: Vec<Vec<i32>>) -> i32 {
    let mut most_calories = 0;
    for elf in elves {
        let mut elf_calories = 0;
        for calorie in elf {
            elf_calories += calorie;
        }
        if elf_calories > most_calories {
            most_calories = elf_calories;
        }
    }
    most_calories
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;
    #[test]
    fn it_can_find_most_calories_for_example_puzzle_input() {
        let contents = fs::read_to_string("./data/day_1_calorie_counting_example_puzzle_input.txt")
            .expect("Should have been able to read the file");

        let elves: Vec<Vec<i32>> = contents
            .split("\n\n")
            .map(|calories| calories
                .split('\n')
                .map(|calorie_string| calorie_string.parse::<i32>().unwrap())
                .collect()
            )
            .collect();

        assert_eq!(calorie_counting(elves), 24000);
    }

    #[test]
    fn it_can_find_most_calories_for_puzzle_input() {
        let contents = fs::read_to_string("./data/day_1_calorie_counting_puzzle_input.txt")
            .expect("Should have been able to read the file");

        let elves: Vec<Vec<i32>> = contents
            .split("\n\n")
            .map(|calories| calories
                .split('\n')
                .map(|calorie_string| calorie_string.parse::<i32>().unwrap())
                .collect()
            )
            .collect();

        assert_eq!(calorie_counting(elves), 71124);
    }
}