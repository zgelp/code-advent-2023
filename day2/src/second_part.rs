use std::collections::HashMap;
use crate::first_part::GameData;
#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;


    #[test]
    fn test_max_color_combinations() {
        let games = vec![
            GameData {
                game_id: 1,
                color_counts: vec![
                    [("blue".to_string(), 3), ("red".to_string(), 4)].iter().cloned().collect(),
                    [("red".to_string(), 1), ("green".to_string(), 2), ("blue".to_string(), 6)].iter().cloned().collect(),
                    [("green".to_string(), 2)].iter().cloned().collect(),
                ],
            },
            GameData {
                game_id: 2,
                color_counts: vec![
                    [("blue".to_string(), 1), ("green".to_string(), 2)].iter().cloned().collect(),
                    [("green".to_string(), 3), ("blue".to_string(), 4), ("red".to_string(), 1)].iter().cloned().collect(),
                    [("green".to_string(), 1), ("blue".to_string(), 1)].iter().cloned().collect(),
                ],
            },
        ];

        let max_combinations = filter_valid_games(games);
        assert_eq!(max_combinations[0].color_counts[0].get("red"), Some(&4));
        assert_eq!(max_combinations[0].color_counts[0].get("green"), Some(&2));
        assert_eq!(max_combinations[0].color_counts[0].get("blue"), Some(&6));
        assert_eq!(max_combinations[1].color_counts[0].get("red"), Some(&1));
        assert_eq!(max_combinations[1].color_counts[0].get("green"), Some(&3));
        assert_eq!(max_combinations[1].color_counts[0].get("blue"), Some(&4));
    }

    #[test]
    fn test_multiply_color_counts() {
        let games = vec![
            GameData {
                game_id: 1,
                color_counts: vec![
                    {
                        let mut map = HashMap::new();
                        map.insert("red".to_string(), 1);
                        map.insert("green".to_string(), 2);
                        map.insert("blue".to_string(), 3);
                        map
                    },
                ],
            },
            GameData {
                game_id: 2,
                color_counts: vec![
                    {
                        let mut map = HashMap::new();
                        map.insert("red".to_string(), 1);
                        map.insert("green".to_string(), 1);
                        map.insert("blue".to_string(), 1);
                        map
                    },
                ],
            },
            GameData {
                game_id: 3,
                color_counts: vec![
                    {
                        let mut map = HashMap::new();
                        map.insert("red".to_string(), 2);
                        map.insert("green".to_string(), 3);
                        map.insert("blue".to_string(), 4);
                        map
                    },
                ],
            },
        ];

        let products = multiply_color_counts(games);
        assert_eq!(products, vec![6, 1, 24]);
    }

    #[test]
    fn test_sum_of_products() {
        let products = vec![6, 1, 24];
        let total_sum = sum_of_products(products);
        assert_eq!(total_sum, 31);
    }
}

fn filter_valid_games(games: Vec<GameData>) -> Vec<GameData> {
    games.into_iter().map(|game| {
        let mut max_counts: HashMap<String, u32> = HashMap::new();

        for color_count in &game.color_counts {
            for (color, &count) in color_count {
                // This is shid
                max_counts.entry(color.clone())
                    .and_modify(|max_count| *max_count = (*max_count).max(count))
                    .or_insert(count);
            }
        }

        GameData {
            game_id: game.game_id,
            color_counts: vec![max_counts],
        }
    }).collect()
}


fn multiply_color_counts(games: Vec<GameData>) -> Vec<u32> {
    games.iter().map(|game| {
        let color_count = &game.color_counts[0];
        let red = color_count.get("red").unwrap();
        let green = color_count.get("green").unwrap();
        let blue = color_count.get("blue").unwrap();

        red * green * blue
    }).collect()
}

fn sum_of_products(products: Vec<u32>) -> u32 {
    products.iter().sum()
}


pub fn second_part_solution(game_strings: Vec<String>) -> u32 {
    let games = crate::first_part::parse_games(game_strings);

    let valid_game_ids = multiply_color_counts(filter_valid_games(games));
    sum_of_products(valid_game_ids)
}


