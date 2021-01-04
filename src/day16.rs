use crate::util::{print_part_1, print_part_2};
use regex::Regex;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::time::Instant;

fn find_aunt(input: &str, part: usize) -> usize {
    let re: Regex = Regex::new(r"Sue (\d{1,}): (.*)").unwrap();
    let re_value: Regex = Regex::new(r"(\w*): (\d{1,})").unwrap();
    let goal: HashMap<&str, usize> = [
        ("children", 3),
        ("cats", 7),
        ("samoyeds", 2),
        ("pomeranians", 3),
        ("akitas", 0),
        ("vizslas", 0),
        ("goldfish", 5),
        ("trees", 3),
        ("cars", 2),
        ("perfumes", 1),
    ]
    .iter()
    .cloned()
    .collect();

    'next_sue: for line in input.lines() {
        let cap = re.captures_iter(line).next().unwrap();
        for c in re_value.captures_iter(&cap[2]) {
            let name = &c[1];
            let value = c[2].parse::<usize>().unwrap();
            let goal_value = *goal.get(name).unwrap();
            match name {
                "cats" | "trees" => {
                    if (part == 2 && value <= goal_value) || (part == 1 && value != goal_value) {
                        continue 'next_sue;
                    }
                }
                "pomeranians" | "goldfish" => {
                    if (part == 2 && value >= goal_value) || (part == 1 && value != goal_value) {
                        continue 'next_sue;
                    }
                }
                _ => {
                    if value != goal_value {
                        continue 'next_sue;
                    }
                }
            }
        }
        return cap[1].parse::<usize>().unwrap();
    }

    0
}

pub fn main() {
    let input = read_to_string("inputs/day16.txt").expect("Input not found..");
    // PART 1
    let start = Instant::now();
    let known_answer = "373";
    let part_1: usize = find_aunt(&input, 1);
    let duration = start.elapsed();
    print_part_1(&part_1.to_string(), &known_answer, duration);

    // PART 2
    let start = Instant::now();
    let known_answer = "260";
    let part_2: usize = find_aunt(&input, 2);
    let duration = start.elapsed();
    print_part_2(&part_2.to_string(), &known_answer, duration);
}
