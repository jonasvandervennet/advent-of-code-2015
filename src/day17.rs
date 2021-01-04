use crate::util::{print_part_1, print_part_2};
use itertools::Itertools;
use std::fs::read_to_string;
use std::time::Instant;

fn find_aunt(input: &str, goal: usize, part: usize) -> usize {
    let capacities: Vec<usize> = input.lines().map(|v| v.parse::<usize>().unwrap()).collect();
    let mut num_options = 0;
    for group_size in 2..=capacities.len() {
        let mut num_options_group_size = 0;
        for comb in capacities.iter().combinations(group_size) {
            if comb.iter().cloned().sum::<usize>() == goal {
                num_options_group_size += 1;
            }
        }
        if part == 1 {
            num_options += num_options_group_size;
        } else if num_options_group_size > 0 {
            return num_options_group_size;
        }
    }
    num_options
}

pub fn main() {
    let input = read_to_string("inputs/day17.txt").expect("Input not found..");
    // PART 1
    let start = Instant::now();
    let known_answer = "1638";
    let part_1: usize = find_aunt(&input, 150, 1);
    let duration = start.elapsed();
    print_part_1(&part_1.to_string(), &known_answer, duration);

    // PART 2
    let start = Instant::now();
    let known_answer = "17";
    let part_2: usize = find_aunt(&input, 150, 2);
    let duration = start.elapsed();
    print_part_2(&part_2.to_string(), &known_answer, duration);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let input = "20\n15\n10\n5\n5";
        let answer = find_aunt(&input, 25, 1);
        assert_eq!(answer, 4);
    }

    #[test]
    fn test_example_2() {
        let input = "20\n15\n10\n5\n5";
        let answer = find_aunt(&input, 25, 2);
        assert_eq!(answer, 3);
    }
}
