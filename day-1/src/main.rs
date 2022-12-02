use std::fs;

fn main() {
    let contents = fs::read_to_string("../input.txt").expect("Input read");

    let mut elf_backpacks: Vec<u32> = contents
        .split("\n\n")
        .map(|backpack| {
            backpack
                .split("\n")
                .map(|meal| meal.parse::<u32>().unwrap())
                .fold(0, |acc, x| acc + x)
        })
        .collect();

    let mut calories_elf_highest_calories: u32 = 0;
    let mut index_elf_highest_calories: u32 = 0;
    let mut elf_index = 0;

    println!("Elf backpacks:\n{elf_backpacks:?}");

    for elf_calories in elf_backpacks.iter() {
        elf_index = elf_index + 1;
        if *elf_calories > calories_elf_highest_calories {
            calories_elf_highest_calories = *elf_calories;
            index_elf_highest_calories = elf_index
        }
    }

    println!("Elf with most calories:\n{index_elf_highest_calories}. calories carried {calories_elf_highest_calories}");

    elf_backpacks.sort();
    elf_backpacks.reverse();

    println!("Sorted elf backpacks:\n{elf_backpacks:?}");
    let calories_top_3_elves = elf_backpacks[0] + elf_backpacks[1] + elf_backpacks[2];

    println!("Calories carried by the top 3 elves {calories_top_3_elves:?}")
}
