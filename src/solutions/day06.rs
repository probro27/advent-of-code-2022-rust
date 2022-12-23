use std::collections::HashSet;

use crate::Solution;

fn check_if_unique(chars: Vec<char>) -> bool{
    let chars_set: HashSet<char> = HashSet::from_iter(chars.iter().cloned());
    chars_set.len() == chars.len()
}

pub fn solve(input: String) -> Solution {
    let chars_till_package = input.chars().collect::<Vec<char>>().windows(4).position(|element| check_if_unique(Vec::from(element))).unwrap();
    let chars_till_message = input.chars().collect::<Vec<char>>().windows(14).position(|element| check_if_unique(Vec::from(element))).unwrap();
    Solution { 
        first: (chars_till_package + 4).to_string(), 
        second: (chars_till_message + 14).to_string()
    }
}