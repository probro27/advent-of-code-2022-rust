use std::collections::HashMap;

use crate::Solution;

fn generate_pick_score_hashmap() -> HashMap<String, i32> {
    let mut pick_score: HashMap<String, i32> = HashMap::new();
    pick_score.insert("X".to_string(), 1);
    pick_score.insert("Y".to_string(), 2);
    pick_score.insert("Z".to_string(), 3);
    pick_score.insert("A".to_string(), 1);
    pick_score.insert("B".to_string(), 2);
    pick_score.insert("C".to_string(), 3);
    pick_score
}

fn generate_move_hashmap() -> HashMap<i32, String> {
    let mut move_map: HashMap<i32, String> = HashMap::new();
    move_map.insert(1, "X".to_string());
    move_map.insert(2, "Y".to_string());
    move_map.insert(3, "Z".to_string());
    move_map
}

fn calculate_score(opponent_move: String, player_move: String) -> i32 {
    let pick_score: HashMap<String, i32> = generate_pick_score_hashmap();
    let diff = pick_score[&player_move] - pick_score[&opponent_move];
    if diff.rem_euclid(3) == 0 {
        pick_score[&player_move] + 3
    } else if diff.rem_euclid(3) == 1 {
        pick_score[&player_move] + 6
    } else if diff.rem_euclid(3) == 2 {
        pick_score[&player_move]
    } else {
        let value = format!("This shouldn't happen {} % {} = {}", diff, 3, diff.rem_euclid(3));
        panic!("{}", &value[..])
    }
}

fn calculate_score_2(opponent_move: String, result: String) -> i32 {
    let pick_score = generate_pick_score_hashmap();
    let move_map = generate_move_hashmap();
    if pick_score[&result] == 1 {
        let mut player_move: i32 = (2 + (pick_score[&opponent_move] % 3)) % 3;
        if player_move == 0 {
            player_move = 3;
        }
        pick_score[&move_map[&player_move]]
    } else if pick_score[&result] == 2 {
        let mut player_move: i32 = pick_score[&opponent_move] % 3;
        if player_move == 0 {
            player_move = 3;
        }
        pick_score[&move_map[&player_move]] + 3
    } else {
        let mut player_move: i32 = (1 + (pick_score[&opponent_move] % 3)) % 3;
        if player_move == 0 {
            player_move = 3;
        }
        pick_score[&move_map[&player_move]] + 6 
    }
}

pub fn solve(input: String) -> Solution {
    let total_score = input
        .trim_end()
        .split("\n")
        .map(|pair| {
            let moves = (*pair).split(" ")
                    .map(|element| String::from(element))
                    .collect::<Vec<String>>();
            calculate_score(moves[0].clone(), moves[1].clone())
        })
        .fold(0, |sum, acc| sum + acc);
    let total_score_2 = input
        .trim_end()
        .split("\n")
        .map(|pair| {
            let moves = (*pair).split(" ")
                    .map(|element| String::from(element))
                    .collect::<Vec<String>>();
            calculate_score_2(moves[0].clone(), moves[1].clone())
        })
        .fold(0, |sum, acc| sum + acc);
    Solution {
        first: total_score.to_string(),
        second: total_score_2.to_string()
    }
}