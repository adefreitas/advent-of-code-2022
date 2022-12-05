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

    let moves_separator = Regex::new(r"(move\s)|(\sfrom\s)|(\sto\s)").unwrap();

    // Will be [0] = amount, [1] = origin, [2] = destination
    let moves: Vec<Vec<usize>> = split_content[1]
        .split("\n")
        .into_iter()
        .map(|row| {
            return moves_separator
                .split(row)
                .filter(|thing| -> bool { thing.len() > 0 })
                .map(|thing| -> usize {
                    return thing.parse::<usize>().unwrap();
                })
                .collect();
        })
        .collect();

    for moe in moves {
        // println!("{:?}", moe);
        for _i in 0..moe[0] {
            let val = rotated_state[moe[1] - 1].pop();
            if val.is_some() {
                rotated_state[moe[2] - 1].push(val.unwrap());
            }
        }
    }

    println!("{:?}", rotated_state);

    rotated_state
        .iter()
        .for_each(|item| print!("{:?}", item.last().unwrap()));
}
