use std::{collections::{HashSet, HashMap}, str::Chars, hash::Hash};

// Part 1

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

// Part 2

pub fn find_badges_and_sum_priorities(rucksacks: Vec<String>) -> u32 {

    const SLICE_LENGTH: usize = 3;

    let mut i: usize = 0;
    let mut total: u32 = 0;
    loop {
        let mut map: HashMap<char, u32> = HashMap::new();
        let rucksack_group = &rucksacks[i..i + SLICE_LENGTH];
        for rucksack in rucksack_group {
            let mut seen: HashSet<char> = HashSet::new();
            for item_type in rucksack.chars() {
                if seen.contains(&item_type) {
                    continue;
                }
                seen.insert(item_type);
                let next_count = match map.get(&item_type) {
                    Some(current_count) => current_count + 1,
                    None => 1
                };
                map.insert(item_type, next_count);
            }
        }
        i += SLICE_LENGTH;
        total += item_types_to_sum_of_priorities(
            map
                .into_iter()
                .filter(|(_, value)| *value == 3)
                .map(|(key, _)| key)
                .collect()
        );
        if i >= rucksacks.len() {
            break;
        }
    }
    total
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

    fn input_to_vector_str(input: String) -> Vec<String> {
        input
            .split("\n")
            .map(|rucksack| String::from(rucksack))
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

    #[test]
    fn it_should_find_sum_of_badges_with_priority_for_puzzle_input() {
        let input = get_input_from_file("./data/day_3_puzzle_input.txt");
        let reorganise_rucksacks = input_to_vector_str(input);

        assert_eq!(find_badges_and_sum_priorities(reorganise_rucksacks), 2881);
    }
}