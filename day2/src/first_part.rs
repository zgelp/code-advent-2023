use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct GameData {
    pub game_id: u32,
    pub color_counts: Vec<HashMap<String, u32>>,
}

struct PredefinedCounts {
    red: u32,
    green: u32,
    blue: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_parsing() {

        let test_cases = vec![
            "Game 97: 3 green, 11 red, 1 blue; 3 green, 13 red, 4 blue; 1 green, 3 blue, 12 red; 4 green, 10 red; 4 blue, 10 green, 12 red",
            "Game 98: 6 blue, 12 red; 17 red, 1 green, 11 blue; 13 blue, 9 red; 9 red, 6 blue, 2 green",
            "Game 99: 15 green, 1 blue, 11 red; 12 green, 12 blue, 14 red; 12 green, 10 blue, 1 red",
            "Game 100: 1 green, 11 red, 4 blue; 4 green, 1 red; 9 red, 2 blue; 5 blue, 11 red, 9 green",
        ].iter().map(|s| s.to_string()).collect::<Vec<String>>();

        let parsed_games = parse_games(test_cases);
        assert_eq!(parsed_games.len(), 4);
    }

    #[test]
    fn test_filter_valid_games() {
        let games = vec![
            GameData {
                game_id: 97,
                color_counts: vec![
                    [("green".to_string(), 13), ("red".to_string(), 12), ("blue".to_string(), 14)].iter().cloned().collect(),
                    [("green".to_string(), 13), ("red".to_string(), 10)].iter().cloned().collect(),
                ],
            },
            GameData {
                game_id: 98,
                color_counts: vec![
                    [("green".to_string(), 14), ("red".to_string(), 12)].iter().cloned().collect(),
                ],
            },
            GameData {
                game_id: 99,
                color_counts: vec![
                    [("green".to_string(), 11), ("red".to_string(), 11), ("blue".to_string(), 13)].iter().cloned().collect(),
                ],
            },
        ];

        let valid_game_ids = filter_valid_games(games);
        assert_eq!(valid_game_ids, vec![97, 99]);
    }

    #[test]
    fn test_first_part_solution() {
        let game_strings = vec![
            "Game 97: 3 green, 11 red, 1 blue; 3 green, 13 red, 4 blue; 1 green, 3 blue, 12 red; 4 green, 10 red; 4 blue, 10 green, 12 red",
            "Game 98: 6 blue, 12 red; 17 red, 1 green, 11 blue; 13 blue, 9 red; 9 red, 6 blue, 2 green",
            "Game 99: 15 green, 1 blue, 11 red; 12 green, 12 blue, 14 red; 12 green, 10 blue, 1 red",
            "Game 100: 1 green, 11 red, 4 blue; 4 green, 1 red; 9 red, 2 blue; 5 blue, 11 red, 9 green",
        ].iter().map(|s| s.to_string()).collect::<Vec<String>>();
        let sum = first_part_solution(game_strings);
        assert_eq!(sum, 100, "The sum of valid game IDs should be 100");
    }
}


pub(crate) fn parse_games(game_strings: Vec<String>) -> Vec<GameData> {
    game_strings.into_iter().map(|game_str| {
        let game_str = game_str.as_str();
        let parts: Vec<&str> = game_str.split(": ").collect();
        let game_id_str = parts[0];
        let game_id = game_id_str.split_whitespace().nth(1).unwrap().parse::<u32>().unwrap();
        let color_counts_str = parts[1];
        let color_sets = color_counts_str.split("; ").collect::<Vec<&str>>();
        let color_counts = color_sets.into_iter().map(|set| {
            set.split(", ").map(|color_count| {
                let parts: Vec<&str> = color_count.split_whitespace().collect();
                let count = parts[0].parse::<u32>().unwrap();
                let color = parts[1].to_string();
                (color, count)
            }).collect::<HashMap<String, u32>>()
        }).collect();

        GameData { game_id, color_counts }
    }).collect()
}


fn filter_valid_games(games: Vec<GameData>) -> Vec<u32> {
    let predefined = PredefinedCounts { red: 12, green: 13, blue: 14 };
    games.into_iter()
        .filter(|game| {
            game.color_counts.iter().all(|color_count| {
                *color_count.get("red").unwrap_or(&0) <= predefined.red &&
                    *color_count.get("green").unwrap_or(&0) <= predefined.green &&
                    *color_count.get("blue").unwrap_or(&0) <= predefined.blue
            })
        })
        .map(|game| game.game_id)
        .collect()
}


pub fn first_part_solution(game_strings: Vec<String>) -> u32 {
    let games = parse_games(game_strings);

    let valid_game_ids = filter_valid_games(games);
    valid_game_ids.into_iter().sum()
}
