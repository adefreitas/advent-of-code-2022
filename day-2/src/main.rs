use std::{collections::HashMap, fs};

fn main() {
    let contents = fs::read_to_string("../input.txt").expect("Input read");

    let mut scoring_map: HashMap<&str, u32> = HashMap::new();
    let round_win_score: u32 = 6;
    let round_draw_score: u32 = 3;

    // Initialise scores
    // Rock -> A/X -> 1 point
    scoring_map.insert("A", 1);
    scoring_map.insert("X", 1);
    // Paper -> B/Y -> 2 points
    scoring_map.insert("B", 2);
    scoring_map.insert("Y", 2);
    // Scissors -> C/Z -> 3 points
    scoring_map.insert("C", 3);
    scoring_map.insert("Z", 3);

    let rounds: Vec<Vec<&str>> = contents
        .split("\n")
        .map(|round| round.split(" ").collect())
        .collect();

    println!("Loaded strategy {rounds:?} and scores {scoring_map:?}");

    let mut elf_score = 0;
    let mut adversary_score = 0;

    // Begining of part 1
    /*
    for round in rounds.clone() {
        let adversary_move = round[0];
        let elf_move = round[1];

        let adversary_move_score = scoring_map.get(adversary_move).copied().unwrap_or(0);
        let elf_move_score = scoring_map.get(elf_move).copied().unwrap_or(0);

        println!("Moves are {adversary_move} and {elf_move}");

        adversary_score += adversary_move_score;
        elf_score += elf_move_score;

        if is_winning_move(elf_move, adversary_move) {
            elf_score += round_win_score;
            println!("Winning move, scores elf:{elf_score} adversary: {adversary_score}");
        } else if !is_tie(elf_move, adversary_move) {
            adversary_score += round_win_score;
            println!("Losing move, scores elf:{elf_score} adversary: {adversary_score}");
        } else {
            adversary_score += round_draw_score;
            elf_score += round_draw_score;
            println!("Tie move, scores elf:{elf_score} adversary: {adversary_score}");
        }
    }

    println!("The final scores are elf: {elf_score} adversary: {adversary_score}"); */

    fn is_winning_move(elf_move: &str, adversary_move: &str) -> bool {
        return elf_move == "X" && adversary_move == "C"
            || elf_move == "Z" && adversary_move == "B"
            || elf_move == "Y" && adversary_move == "A";
    }

    fn is_tie(elf_move: &str, adversary_move: &str) -> bool {
        return elf_move == "X" && adversary_move == "A"
            || elf_move == "Y" && adversary_move == "B"
            || elf_move == "Z" && adversary_move == "C";
    }

    // End of part 1

    // Beginning of part 2

    //  moves map
    let mut losing_moves_map: HashMap<&str, &str> = HashMap::new();
    losing_moves_map.insert("A", "Z");
    losing_moves_map.insert("B", "X");
    losing_moves_map.insert("C", "Y");

    let mut winning_moves_map: HashMap<&str, &str> = HashMap::new();
    winning_moves_map.insert("A", "Y");
    winning_moves_map.insert("B", "Z");
    winning_moves_map.insert("C", "X");

    let mut tying_moves_map: HashMap<&str, &str> = HashMap::new();
    tying_moves_map.insert("A", "X");
    tying_moves_map.insert("B", "Y");
    tying_moves_map.insert("C", "Z");

    for round in rounds.clone() {
        let adversary_move = round[0];
        let elf_action = round[1];
        let elf_move: &str;

        if elf_action == "X" {
            elf_move = losing_moves_map.get(adversary_move).copied().unwrap()
        } else if elf_action == "Y" {
            elf_move = tying_moves_map.get(adversary_move).copied().unwrap();
        } else {
            elf_move = winning_moves_map.get(adversary_move).copied().unwrap();
        }

        let adversary_move_score = scoring_map.get(adversary_move).copied().unwrap_or(0);
        let elf_move_score = scoring_map.get(elf_move).copied().unwrap_or(0);

        println!("Moves are {adversary_move} and {elf_move}");

        adversary_score += adversary_move_score;
        elf_score += elf_move_score;

        if is_winning_move(elf_move, adversary_move) {
            elf_score += round_win_score;
            println!("Winning move, scores elf:{elf_score} adversary: {adversary_score}");
        } else if !is_tie(elf_move, adversary_move) {
            adversary_score += round_win_score;
            println!("Losing move, scores elf:{elf_score} adversary: {adversary_score}");
        } else {
            adversary_score += round_draw_score;
            elf_score += round_draw_score;
            println!("Tie move, scores elf:{elf_score} adversary: {adversary_score}");
        }
    }

    println!("The final scores are elf: {elf_score} adversary: {adversary_score}");
    // End of part 2
}
