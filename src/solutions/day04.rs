use crate::Solution;

fn contains_whole(pair1: Vec<&str>, pair2: Vec<&str>) -> i32 {
    let pair1_elements = (pair1[0].parse::<i32>().unwrap_or_else(|error| panic!("{}", error)), pair1[1].parse::<i32>().unwrap_or_else(|error| panic!("{}", error)));
    let pair2_elements = (pair2[0].parse::<i32>().unwrap_or_else(|error| panic!("{}", error)), pair2[1].parse::<i32>().unwrap_or_else(|error| panic!("{}", error)));
    if (pair1_elements.0 <= pair2_elements.0 && pair1_elements.1 >= pair2_elements.1) || (pair1_elements.0 >= pair2_elements.0 && pair2_elements.1 >= pair1_elements.1) {
        1
    } else {
        0
    }
}

fn contains_overlap(pair1: Vec<&str>, pair2: Vec<&str>) -> i32 {
    let pair1_elements = (pair1[0].parse::<i32>().unwrap_or_else(|error| panic!("{}", error)), pair1[1].parse::<i32>().unwrap_or_else(|error| panic!("{}", error)));
    let pair2_elements = (pair2[0].parse::<i32>().unwrap_or_else(|error| panic!("{}", error)), pair2[1].parse::<i32>().unwrap_or_else(|error| panic!("{}", error)));
    if (pair1_elements.0 <= pair2_elements.1 && pair1_elements.1 >= pair2_elements.1) || (pair1_elements.1 >= pair2_elements.0 && pair2_elements.1 >= pair1_elements.1) {
        1
    }
    else if (pair1_elements.0 <= pair2_elements.0 && pair1_elements.1 >= pair2_elements.1) || (pair1_elements.0 >= pair2_elements.0 && pair2_elements.1 >= pair1_elements.1) {
        1
    } else {
        0
    }
}

pub fn solve(input: String) -> Solution {
    let inputs_containing_whole: i32 = input.split("\n")
            .map(|pair| pair.split(",").collect::<Vec<&str>>())
            .map(|element| (element[0].split("-").collect::<Vec<&str>>(), element[1].split("-").collect::<Vec<&str>>()))
            .map(|element| contains_whole(element.0, element.1))
            .sum();
    let inputs_containing_overlap: i32 = input.split("\n")
            .map(|pair| pair.split(",").collect::<Vec<&str>>())
            .map(|element| (element[0].split("-").collect::<Vec<&str>>(), element[1].split("-").collect::<Vec<&str>>()))
            .map(|element| contains_overlap(element.0, element.1))
            .sum();

    Solution {
        first: inputs_containing_whole.to_string(),
        second: inputs_containing_overlap.to_string()
    }
}