#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_concat_and_sum() {
        let test_cases = vec![
            vec!["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"],
        ];

        let expected_sum = 142;

        for case in test_cases {
            let concat_results = crate::concat_inputs(case.clone().into_iter().map(String::from).collect());
            let sum = first_part_solution(concat_results);
            assert_eq!(sum, expected_sum, "Failed on test case: {:?}", case);
        }
    }
}

pub fn concat_inputs(input: Vec<String>) -> Vec<u32> {
    // Parse and concat numbers from string
    input.iter().map(|s| {
        let digits: Vec<char> = s.chars().filter(|c| c.is_ascii_digit()).collect();
        let num_str = match digits.len() {
            1 => digits[0].to_string().repeat(2),
            _ => format!("{}{}", digits[0], digits[digits.len() - 1]),
        };
        num_str.parse::<u32>().unwrap_or(0)
    }).collect()
}

pub fn first_part_solution(input: Vec<u32>) -> u32 {
    input.iter().sum()
}