use crate::Solution;
use std::collections::HashSet;

fn find_common_element(stack1_str: String, stack2_str: String) -> String {
    let stack1 = stack1_str.split("").map(|element| String::from(element)).collect::<Vec<String>>();
    let stack2 = stack2_str.split("").map(|element| String::from(element)).collect::<Vec<String>>();
    let stack1_set: HashSet<String> = HashSet::from_iter(stack1.iter().cloned());
    let stack2_set: HashSet<String> = HashSet::from_iter(stack2.iter().cloned());
    let result_list = stack1_set.intersection(&stack2_set).filter(|&element| (*element) != "").collect::<Vec<&String>>();
    let result = result_list[0];
    return (*result).clone();
}

fn find_common_element_3(stack1_str: String, stack2_str: String, stack3_str: String) -> String {
    let stack1 = stack1_str.split("").map(|element| String::from(element)).collect::<Vec<String>>();
    let stack2 = stack2_str.split("").map(|element| String::from(element)).collect::<Vec<String>>();
    let stack3 = stack3_str.split("").map(|element| String::from(element)).collect::<Vec<String>>();
    let stack1_set: HashSet<String> = HashSet::from_iter(stack1.iter().cloned());
    let stack2_set: HashSet<String> = HashSet::from_iter(stack2.iter().cloned());
    let stack3_set: HashSet<String> = HashSet::from_iter(stack3.iter().cloned());
    let result_list_unfiltered = &(&stack1_set & &stack2_set) & &stack3_set;
    let result_list = result_list_unfiltered.iter().filter(|&element| (*element) != "").collect::<Vec<&String>>();
    let result = result_list[0];
    return (*result).clone();
}

fn find_value(element: String) -> u32 {
    let character = element.chars().next().expect("String is empty");
    if character.is_uppercase() {
        character as u32 - 64 + 26
    }
    else {
        character as u32 - 96
    }
}

pub fn solve(input: String) -> Solution {
    let sum_common_elements: u32 = 
        input.split("\n")
            .map(|element| String::from(element))
            .map(|element| 
                (element[..element.len() / 2].to_string().clone(), element[element.len() / 2..].to_string().clone())
            )
            .map(|element| find_common_element(element.0.clone(), element.1.clone()))
            .map(|element| find_value(element))
            .sum();
    let sum_common_stack_elements: u32 = 
        input.split("\n")
            .map(|element| String::from(element))
            .collect::<Vec<String>>()
            .chunks(3)
            .map(|w| find_common_element_3(w[0].clone(), w[1].clone(), w[2].clone()))
            .map(|element| find_value(element))
            .sum();

    Solution {
        first: sum_common_elements.to_string(),
        second: sum_common_stack_elements.to_string()
    }
}