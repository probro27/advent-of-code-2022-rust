use crate::Solution;

pub fn solve(input: String) -> Solution {
    let mut inventory: Vec<i32> = input.trim_end()
                                .split("\n\n")
                                .map(|item | item.split("\n")
                                .fold(0, |acc, element| acc + element.parse::<i32>().unwrap()))
                                .collect::<Vec<i32>>();
    inventory.sort_unstable_by(|a, b| b.cmp(a));

    Solution {
        first: inventory[0].to_string(),
        second: inventory[0..3].iter().sum::<i32>().to_string()
    }
}