use crate::Solution;

pub fn solve(input: String) -> Solution {
    let num_crates: usize = 9; // sample data (3) test data (9)
    let mut stacks: Vec<Vec<String>> = Vec::new();
    let input_parts = input.split("\n\n").map(|element| String::from(element)).collect::<Vec<String>>();
    let stack_input = input_parts[0].lines().collect::<Vec<&str>>();
    let stack_input_lined = &stack_input[0..stack_input.len() - 1];
    let move_input = &input_parts[1].lines().collect::<Vec<&str>>();
    for stack in 0..num_crates {
        stacks.push(Vec::new());
        for &l in stack_input_lined {
            let c = l.chars().nth((4 * stack) + 1).unwrap();
            if c != ' ' {
                stacks[stack].push(String::from(c));
            }
        }
    }
    let mut stacks_reversed = stacks.iter().map(|stack| stack.iter().rev().collect::<Vec<&String>>()).collect::<Vec<Vec<&String>>>();
    let mut stacks_reversed_2 = stacks_reversed.clone();
    for &line in move_input {
        let words = line.split(" ").collect::<Vec<&str>>();
        let mut number_of_crates = words[1].parse::<usize>().expect("msg");
        let start_stack = words[3].parse::<usize>().expect("");
        let end_stack = words[5].parse::<usize>().expect("");
        while number_of_crates != 0 {
            let length_of_start_stack = stacks_reversed[start_stack - 1].len();
            let item = stacks_reversed[start_stack - 1].remove(length_of_start_stack - 1);
            stacks_reversed[end_stack - 1].push(item);
            number_of_crates -= 1;
        }
    }
    
    for &line in move_input {
        let words = line.split(" ").collect::<Vec<&str>>();
        let mut number_of_crates = words[1].parse::<usize>().expect("msg");
        let start_stack = words[3].parse::<usize>().expect("");
        let end_stack = words[5].parse::<usize>().expect("");
        while number_of_crates != 0 {
            let length_of_start_stack = stacks_reversed_2[start_stack - 1].len();
            let item = stacks_reversed_2[start_stack - 1].remove(length_of_start_stack - number_of_crates);
            stacks_reversed_2[end_stack - 1].push(item);
            number_of_crates -= 1;
        }
    }

    
    Solution {
        first: stacks_reversed.iter().map(|element| element.last().unwrap().clone().to_string()).collect(),
        second: stacks_reversed_2.iter().map(|element| element.last().unwrap().clone().to_string()).collect()
    }
}
