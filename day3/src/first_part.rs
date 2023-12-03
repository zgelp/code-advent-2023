#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_parsing() {

        let test_cases = vec![
            "467..114..",
            "...*......",
            "..35..633.",
            "......#...",
            "617*......",
            ".....+.58.",
            "..592.....",
            "......755.",
            "...$.*....",
            ".664.598.."
        ].iter().map(|s| s.to_string()).collect::<Vec<String>>();

        let part_numbers = get_part_numbers(test_cases);
        assert_eq!(part_numbers, [467, 35, 633, 617, 592, 755, 664, 598]);
    }
}

fn get_part_numbers(grid: Vec<String>) -> Vec<u32> {
    let mut part_numbers = Vec::new();
    let grid: Vec<Vec<char>> = grid.iter().map(|s| s.chars().collect()).collect();

    for (index, row) in grid.iter().enumerate() {
        let mut iter_count = 0;
        while iter_count < row.len() {
            if row[iter_count].is_ascii_digit() {
                let mut number = String::new();
                let mut k = iter_count;

                // Collecting the full multi-digit number
                while k < row.len() && row[k].is_ascii_digit() {
                    number.push(row[k]);
                    k += 1;
                }

                // Check if the first or last digit of the number is adjacent to any sign
                if is_adjacent_to_sign(&grid, index, iter_count, k - 1) {
                    part_numbers.push(number.parse::<u32>().unwrap());
                }

                // Skip past the current number - more than 1 digit
                iter_count = k;
            } else {
                iter_count += 1;
            }
        }
    }

    part_numbers
}

fn is_adjacent_to_sign(grid: &[Vec<char>], row_idx: usize, start_idx: usize, end_idx: usize) -> bool {
    // Brute forced until i got all signs
    let signs = ['+', '*', '$', '#', '=', '@', '&', '%', '/', '-'];

    // Check only the first and last digit for position (also horizontaly)
    let check_indices = [start_idx, end_idx];

    for &i in &check_indices {
        let positions = [
            (row_idx.checked_sub(1), Some(i)), // Above
            ((row_idx + 1 < grid.len()).then_some(row_idx + 1), Some(i)), // Below
            (Some(row_idx), i.checked_sub(1)), // Left
            (Some(row_idx), (i + 1 < grid[0].len()).then_some(i + 1)), // Right

            // Diagonal positions // If 4-digit number it is a bug because of check_indices
            (row_idx.checked_sub(1), i.checked_sub(1)), // Top-left
            (row_idx.checked_sub(1), (i + 1 < grid[0].len()).then_some(i + 1)), // Top-right
            ((row_idx + 1 < grid.len()).then_some(row_idx + 1), i.checked_sub(1)), // Bottom-left
            ((row_idx + 1 < grid.len()).then_some(row_idx + 1), (i + 1 < grid[0].len()).then_some(i + 1)), // Bottom-right
        ];

        for (x, y) in positions {
            if let (Some(x), Some(y)) = (x, y) {
                if signs.contains(&grid[x][y]) {
                    return true;
                }
            }
        }
    }

    false
}

fn sum_of_products(products: Vec<u32>) -> u32 {
    products.iter().sum()
}


pub fn first_part_solution(game_strings: Vec<String>) -> u32 {
    let valid_game_ids = get_part_numbers(game_strings);
    sum_of_products(valid_game_ids)
}




