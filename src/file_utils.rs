use std::env;
use std::fs;

pub fn get_input(day: i32) -> String {
    let working_dir = env::current_dir().unwrap();
    let file_path = working_dir
        .join("inputs")
        .join(format!("day{:02}.txt", day));
    
    let file = fs::read_to_string(file_path);
    file.expect("Error loading input file").trim().to_string()
}