use std::fs;

fn main() {
    let contents = fs::read_to_string("./input.txt").expect("Input read");
    let trees: Vec<Vec<i32>> = contents
        .split("\n")
        .map(|row| {
            row.chars()
                .map(|char| char.to_digit(10).unwrap_or(0) as i32)
                .collect()
        })
        .collect();
    // println!("contents {:?}", contents);

    println!("Width {:?}", trees.len());
    println!("Height {:?}", trees[0].len());
    let mut visibles: u32 = ((trees.len() as u32) * 2) + (((trees[0].len() - 2) as u32) * 2);

    println!("Total trees {:?}", visibles);
    // Begining of part 1
    for y in 1..trees.len() - 1 {
        // println!("Y is {:?}", y);
        for x in 1..trees[0].len() - 1 {
            let tree = trees[y][x];
            let mut left_max = -1;
            let mut top_max = -1;
            let mut right_max = -1;
            let mut bottom_max = -1;
            for i in 0..x {
                left_max = left_max.max(trees[y][i]);
            }

            for i in 0..y {
                top_max = top_max.max(trees[i][x]);
            }

            for i in x + 1..trees.len() {
                right_max = right_max.max(trees[y][i]);
            }

            for i in y + 1..trees.len() {
                bottom_max = bottom_max.max(trees[i][x]);
            }

            if tree > left_max || tree > bottom_max || tree > right_max || tree > top_max {
                // println!("Visible tree is Y:{:} X:{:} ", y, x);
                visibles += 1;
            }
        }
    }
    // End of part 1

    // Beginning of part 2
    // let y = 3;
    // let x = 2;
    let mut max_score = 0;
    for y in 1..trees.len() - 1 {
        for x in 1..trees[0].len() - 1 {
            let tree = trees[y][x];
            let mut left_score = 0;
            let mut top_score = 0;
            let mut right_score = 0;
            let mut bottom_score = 0;
            for i in (0..x).rev() {
                left_score += 1;
                if trees[y][i] >= tree {
                    break;
                }
            }

            for i in (0..y).rev() {
                top_score += 1;
                if trees[i][x] >= tree {
                    break;
                }
            }

            for i in x + 1..trees.len() {
                right_score += 1;
                if trees[y][i] >= tree {
                    break;
                }
            }

            for i in y + 1..trees.len() {
                bottom_score += 1;
                if trees[i][x] >= tree {
                    break;
                }
            }
            max_score = max_score.max(left_score * top_score * right_score * bottom_score);
        }
    }
    // End of part 2
    println!("Visible trees {:?}", visibles);
    println!("Max score {:?}", max_score);
}
