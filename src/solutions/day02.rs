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

fn calculate_score(opponent_move: String, player_move: String) -> i32 {
    let pick_score: HashMap<String, i32> = generate_pick_score_hashmap();
    let diff = pick_score[&player_move] - pick_score[&opponent_move];
    if diff % 3 == 0 {
        pick_score[&player_move] + 3
    } else if diff % 3 == 1 {
        pick_score[&player_move] + 6
    } else {
        pick_score[&player_move]
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
    Solution {
        first: total_score.to_string(),
        second: String::from("Not started")
    }
}