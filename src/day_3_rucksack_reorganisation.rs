use std::{collections::HashSet};

pub fn reogarnise_rucksack(rucksacks: Vec<(String, String)>) -> u32 {
    let mut duplicate_item_types: Vec<char> = Vec::new();
    for (compartment1, compartment2) in rucksacks {
        let mut rucksack_set:HashSet<char> = HashSet::new();

        compartment1.chars().for_each(|item_type| {
            rucksack_set.insert(item_type);
        });

        for item_type in compartment2.chars() {
            if rucksack_set.contains(&item_type) {
                duplicate_item_types.push(item_type);
                break;
            }
        }
    }
    duplicate_item_types
        .into_iter()
        .map(|item_type| {
            item_type as u32 - match item_type.is_uppercase() {
                true => 38,
                false => 96
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

    fn input_to_vector_tuple_str(input: String) -> Vec<(String, String)> {
        input
            .split("\n")
            .map(|rucksack| rucksack.split_at(rucksack.len()/2))
            .map(|(compartment1, compartment2)| (String::from(compartment1), String::from(compartment2)))
            .collect()
    }

    fn get_input_from_file(path: &str) -> String {
        fs::read_to_string(path)
            .expect("Should have been able to read the file")
    }

    #[test]
    fn it_can_use_the_reorganise_rucksack_to_calculate_rps_score_for_example_puzzle_input() {
        let input = get_input_from_file("./data/day_3_example_puzzle_input.txt");
        let reorganise_rucksacks = input_to_vector_tuple_str(input);
        assert_eq!(reogarnise_rucksack(reorganise_rucksacks), 157);
    }

    #[test]
    fn it_can_use_the_reorganise_rucksack_to_calculate_rps_score_for_puzzle_input() {
        let input = get_input_from_file("./data/day_3_puzzle_input.txt");
        let reorganise_rucksacks = input_to_vector_tuple_str(input);

        assert_eq!(reogarnise_rucksack(reorganise_rucksacks), 7980);
    }
}