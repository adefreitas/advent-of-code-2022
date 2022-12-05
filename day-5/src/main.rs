extern crate regex;
use regex::Regex;
use std::fs;

fn main() {
    let contents = fs::read_to_string("../input.txt").expect("Input read");

    let split_content: Vec<&str> = contents.split("\n\n").collect();

    let separator = Regex::new(r"(\[[A-Z]\]\s{0,1})|(\s{4}|\s{3}$)").unwrap();

    let initial_state: Vec<Vec<char>> = split_content[0]
        .split("\n")
        .into_iter()
        .map(|row| {
            return separator
                .find_iter(row)
                .map(|matched| matched.as_str().chars().nth(1).unwrap())
                .collect();
        })
        .filter(|row: &Vec<char>| row.len() > 0)
        .collect();

    let mut rotated_state: Vec<Vec<char>> = vec![vec![]; initial_state[0].len()];

    let length = initial_state.len();
    for i in (0..length).rev() {
        for j in 0..initial_state[i].len() {
            if !initial_state[i][j].is_whitespace() {
                rotated_state[j].push(initial_state[i][j]);
            }
        }
    }

    println!("{:?}", initial_state);
    println!("{:?}", rotated_state);
}
