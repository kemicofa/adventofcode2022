use std::{collections::HashSet, str::Chars};

const UPPERCASE_ASCII_OFFSET: u32 = 38;
const LOWERCASE_ASCII_OFFSET: u32 = 96;

fn item_types_to_sum_of_priorities(item_types: Vec<char>) -> u32 {
    item_types
        .into_iter()
        .map(|item_type| {
            item_type as u32 - match item_type.is_uppercase() {
                true => UPPERCASE_ASCII_OFFSET,
                false => LOWERCASE_ASCII_OFFSET
            }
        })
        .sum()
}

fn create_set_from_chars(chars: Chars<'_>) -> HashSet<char> {
    let mut set:HashSet<char> = HashSet::new();
    chars.for_each(|item_type| {
        set.insert(item_type);
    });
    set
}

pub fn reorganise_rucksack(rucksacks: Vec<(String, String)>) -> u32 {
    let mut duplicate_item_types: Vec<char> = Vec::new();
    for (compartment1, compartment2) in rucksacks {
        let rucksack_set = create_set_from_chars(compartment1.chars());
        for item_type in compartment2.chars() {
            if rucksack_set.contains(&item_type) {
                duplicate_item_types.push(item_type);
                break;
            }
        }
    }
    item_types_to_sum_of_priorities(duplicate_item_types)
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
        assert_eq!(reorganise_rucksack(reorganise_rucksacks), 157);
    }

    #[test]
    fn it_can_use_the_reorganise_rucksack_to_calculate_rps_score_for_puzzle_input() {
        let input = get_input_from_file("./data/day_3_puzzle_input.txt");
        let reorganise_rucksacks = input_to_vector_tuple_str(input);

        assert_eq!(reorganise_rucksack(reorganise_rucksacks), 7980);
    }
}