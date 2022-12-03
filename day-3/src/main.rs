/*

One Elf has the important job of loading all of the rucksacks with supplies for the jungle journey. Unfortunately, that Elf didn't quite follow the packing instructions, and so a few items now need to be rearranged.

Each rucksack has two large compartments. All items of a given type are meant to go into exactly one of the two compartments. The Elf that did the packing failed to follow this rule for exactly one item type per rucksack.

The Elves have made a list of all of the items currently in each rucksack (your puzzle input), but they need your help finding the errors. Every item type is identified by a single lowercase or uppercase letter (that is, a and A refer to different types of items).

The list of items for each rucksack is given as characters all on a single line. A given rucksack always has the same number of items in each of its two compartments, so the first half of the characters represent items in the first compartment, while the second half of the characters represent items in the second compartment.

For example, suppose you have the following list of contents from six rucksacks:

vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw

The first rucksack contains the items vJrwpWtwJgWrhcsFMMfFFhFp, which means its first compartment contains the items vJrwpWtwJgWr, while the second compartment contains the items hcsFMMfFFhFp. The only item type that appears in both compartments is lowercase p.
The second rucksack's compartments contain jqHRNqRjqzjGDLGL and rsFMfFZSrLrFZsSL. The only item type that appears in both compartments is uppercase L.
The third rucksack's compartments contain PmmdzqPrV and vPwwTWBwg; the only common item type is uppercase P.
The fourth rucksack's compartments only share item type v.
The fifth rucksack's compartments only share item type t.
The sixth rucksack's compartments only share item type s.

To help prioritize item rearrangement, every item type can be converted to a priority:

    Lowercase item types a through z have priorities 1 through 26.
    Uppercase item types A through Z have priorities 27 through 52.

In the above example, the priority of the item type that appears in both compartments of each rucksack is 16 (p), 38 (L), 42 (P), 22 (v), 20 (t), and 19 (s); the sum of these is 157.

Find the item type that appears in both compartments of each rucksack. What is the sum of the priorities of those item types?
*/

use std::fs;

fn main() {
    let contents = fs::read_to_string("../input.txt").expect("Input read");
    // Beginning of part 1
    // Process the input
    let total_prio = contents.split("\n").fold(0, |acc, backpack| -> u32 {
        let mut repeated_item_index: usize = 0;
        let mut i = 0;
        let mut j = backpack.chars().count() / 2;
        
        // Find the repeated char
        while i != j {
            if backpack.chars().nth(i).unwrap() == backpack.chars().nth(j).unwrap() {
                repeated_item_index = i;
                break;
            }
            j += 1;
            if j == backpack.chars().count() {
                j = backpack.chars().count() / 2;
                i += 1;
            }
        }
        // Get the ascii value of the char for some magic
        let ascii_value = backpack.chars().nth(repeated_item_index).unwrap() as u32;

        // Substract the value needed to normalise the value
        let priority: u32 = if ascii_value >= 65 && ascii_value <= 90 {
            ascii_value - 38
        } else {
           ascii_value - 96
        };

        println!(
            "repeated item is {:?} and ascii value is {:?} and priority value is {:?}, accumulator has {acc} for {backpack}",
            backpack.chars().nth(repeated_item_index).unwrap(),
            ascii_value,
            priority

        );
        return acc + priority;
    });

    println!("Total priority of items is {total_prio:?}");

    // End of part 1

    // Begining of part 2

    // Process the input
    let backpacks: Vec<&str> = contents.split("\n").collect();

    let mut backprios: u32 = 0;

    for index in 0..(backpacks.len() / 3) {
        let mut ascii_value: u32 = 0;
        let mut backpack1: Vec<char> = backpacks.get(index * 3).unwrap().chars().collect();
        backpack1.dedup();
        backpack1.sort();
        let mut backpack2: Vec<char> = backpacks.get((index * 3) + 1).unwrap().chars().collect();
        backpack2.dedup();
        backpack2.sort();
        let mut backpack3: Vec<char> = backpacks.get((index * 3) + 2).unwrap().chars().collect();
        backpack3.dedup();
        backpack3.sort();

        for ele in backpack1 {
            if backpack2.contains(&ele) && backpack3.contains(&ele) {
                ascii_value = ele as u32;
                println!("Badge is {:?} ", ele);
                break;
            }
        }
        let priority: u32 = if ascii_value >= 65 && ascii_value <= 90 {
            ascii_value - 38
        } else {
            ascii_value - 96
        };

        println!("Priority is {:?}", priority);
        backprios += priority;
    }

    println!("Total prio of badges {:?}", backprios);
    // End of part 2
}
