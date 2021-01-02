use crate::util::{print_part_1, print_part_2};
use std::fs::read_to_string;
use std::time::Instant;

fn parse_file(input: &str, part: usize) -> usize {
    let mut diff = 0;
    let mut extra = 0;

    for line in input.lines() {
        diff += 2; // for quotes
        extra += 4; // for quotes

        let mut skip_next = false; // prevent overlapping sequences
        for window in line[1..line.chars().count() - 1]
            .chars()
            .collect::<Vec<char>>()
            .windows(2)
        {
            if skip_next {
                skip_next = false;
                continue;
            }
            match (window[0], window[1]) {
                ('\\', '\\') | ('\\', '\"') => {
                    skip_next = true;
                    diff += 1;
                    extra += 2;
                }
                ('\\', 'x') => {
                    skip_next = true;
                    diff += 3;
                    extra += 1;
                }
                _ => {}
            }
        }
    }

    if part == 1 {
        diff
    } else {
        extra
    }
}

pub fn main() {
    let input = read_to_string("inputs/day08.txt").expect("Input not found..");
    // PART 1
    let start = Instant::now();
    let known_answer = "1333";
    let part_1: usize = parse_file(&input, 1);
    let duration = start.elapsed();
    print_part_1(&part_1.to_string(), &known_answer, duration);

    // PART 2
    let start = Instant::now();
    let known_answer = "2046";
    let part_2: usize = parse_file(&input, 2);
    let duration = start.elapsed();
    print_part_2(&part_2.to_string(), &known_answer, duration);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let input = read_to_string("inputs/day08_test.txt").expect("Input not found..");
        let answer = parse_file(&input, 1);
        assert_eq!(answer, 12);
    }
    #[test]
    fn test_example_2() {
        let input = read_to_string("inputs/day08_test.txt").expect("Input not found..");
        let answer = parse_file(&input, 2);
        assert_eq!(answer, 19);
    }
}
