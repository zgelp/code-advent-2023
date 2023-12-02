use std::collections::HashMap;

#[derive(PartialEq)]
enum NumberPosition {
    First,
    Last,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_and_sum() {
        let test_cases = vec![
            vec!["two1nine", "eightwothree", "abcone2threexyz", "xtwone3four", "4nineeightseven2", "zoneight234", "7pqrstsixteen"],
        ];

        let expected_sum: u32 = 281;

        for case in test_cases {
            let concat_numbers = concat_numbers(case.clone().into_iter().map(String::from).collect());
            let sum = second_part_solution(concat_numbers);
            assert_eq!(sum, expected_sum, "Failed on test case: {:?}", case);
        }
    }
}

pub fn concat_numbers(input: Vec<String>) -> Vec<u32> {
    let number_map: HashMap<&str, u32> = [
        ("one", 1), ("two", 2), ("three", 3), ("four", 4),
        ("five", 5), ("six", 6), ("seven", 7), ("eight", 8),
        ("nine", 9),
    ].iter().cloned().collect();

    input.iter().map(|s| {
        let first_number = find_number(s, &number_map, NumberPosition::First);
        let last_number = find_number(s, &number_map, NumberPosition::Last);
        first_number * 10 + last_number
    }).collect()
}

// Function to find a number in a string based on the specified position (first or last).
fn find_number(s: &str, number_map: &HashMap<&str, u32>, position: NumberPosition) -> u32 {
    let mut found_number: Option<(usize, u32)> = None;

    // Closure to update the found_number variable
    let update_found_number = |found_number: &mut Option<(usize, u32)>, index, value| {
        match position {
            // If looking for the first number, update if this is the earliest found number.
            NumberPosition::First if found_number.map_or(true, |(pos, _)| index < pos) => {
                *found_number = Some((index, value));
            },
            // If looking for the last number, update if this is the latest found number.
            NumberPosition::Last if found_number.map_or(true, |(pos, _)| index > pos) => {
                *found_number = Some((index, value));
            },
            _ => {}
        }
    };

    // Iterate over each character in the string along with its index.
    for (index, c) in s.chars().enumerate() {
        if c.is_alphabetic() {
            for (word, &value) in number_map {
                if s[index..].starts_with(word) {
                    // Update found number with new index
                    update_found_number(&mut found_number, index, value);
                    break;
                }
            }
        } else if let Some(digit) = c.to_digit(10) {
            // If the character is a digit, update closure
            update_found_number(&mut found_number, index, digit);
        }
    }

    found_number.map_or(0, |(_, num)| num)
}


pub fn second_part_solution(input: Vec<u32>) -> u32 {
    input.iter().sum()
}
