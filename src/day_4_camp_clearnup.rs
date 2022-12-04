fn is_contained(assignment1: (u32, u32), assignment2: (u32, u32)) -> bool {
    let (lower_bound_1, upper_bound_1) = assignment1;
    let (lower_bound_2, upper_bound_2) = assignment2;

    lower_bound_1 >= lower_bound_2 && upper_bound_1 <= upper_bound_2
}

fn is_partially_contained(assignment1: (u32, u32), assignment2: (u32, u32)) -> bool {
    let (lower_bound_1, upper_bound_1) = assignment1;
    let (lower_bound_2, upper_bound_2) = assignment2;

    (lower_bound_1 >= lower_bound_2 && lower_bound_1 <= upper_bound_2) || 
    (upper_bound_1 >= lower_bound_2 && upper_bound_1 <= upper_bound_2)
}

pub fn camp_cleanup(assignment_pairs: Vec<((u32, u32), (u32, u32))>) -> u32 {
    let mut count = 0;
    for (assignment1, assignment2) in assignment_pairs {
        if is_contained(assignment1, assignment2) || is_contained(assignment2, assignment1) {
            count += 1;
        }
    }
    count
}

pub fn camp_cleanup_part_2(assignment_pairs: Vec<((u32, u32), (u32, u32))>) -> u32 {
    let mut count = 0;
    for (assignment1, assignment2) in assignment_pairs {
        if is_partially_contained(assignment1, assignment2) || is_partially_contained(assignment2, assignment1) {
            count += 1;
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

    fn input_to_assignment_pairs(input: String) -> Vec<((u32, u32), (u32, u32))> {
        input
            .split("\n")
            .map(|row| {
                let elf_assignment_pair: Vec<(u32, u32)> = row
                .split(",")
                .map(|range|{ 
                    let assignment: Vec<&str> = range.split("-").collect();
                    (assignment[0].parse::<u32>().unwrap(), assignment[1].parse::<u32>().unwrap())
                })
                .collect();
                (elf_assignment_pair[0], elf_assignment_pair[1])
            })
            .collect()
    }

    fn get_input_from_file(path: &str) -> String {
        fs::read_to_string(path)
            .expect("Should have been able to read the file")
    }

    #[test]
    fn it_should_be_able_to_count_fully_contained_assignments_for_example_puzzle_input() {
        let input = get_input_from_file("./data/day_4_example_puzzle_input.txt");
        let assinment_pairs = input_to_assignment_pairs(input);
        assert_eq!(camp_cleanup(assinment_pairs), 2);
    }

    #[test]
    fn it_should_be_able_to_count_fully_contained_assignments_for_puzzle_input() {
        let input = get_input_from_file("./data/day_4_puzzle_input.txt");
        let assinment_pairs = input_to_assignment_pairs(input);
        assert_eq!(camp_cleanup(assinment_pairs), 431);
    }

    #[test]
    fn it_should_be_able_to_count_partially_contained_assignments_for_puzzle_input() {
        let input = get_input_from_file("./data/day_4_puzzle_input.txt");
        let assinment_pairs = input_to_assignment_pairs(input);
        assert_eq!(camp_cleanup_part_2(assinment_pairs), 823);
    }
}