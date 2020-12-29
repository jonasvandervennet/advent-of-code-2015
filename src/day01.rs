use crate::util::{print_part_1, print_part_2};
use std::fs::read_to_string;
use std::time::Instant;

fn stop_at_basement(input: &str, part: usize) -> isize {
    let mut floor = 0;
    for (i, c) in input.chars().enumerate() {
        floor += if c == '(' { 1 } else { -1 };
        if part == 2 && floor == -1 {
            return (i + 1) as isize;
        }
    }
    floor
}

pub fn main() {
    let input = read_to_string("inputs/day01.txt").expect("Input not found..");
    // PART 1
    let start = Instant::now();
    let known_answer = "74";
    let part_1: isize = stop_at_basement(&input, 1);
    let duration = start.elapsed();
    print_part_1(&part_1.to_string(), &known_answer, duration);

    // PART 2
    let start = Instant::now();
    let known_answer = "1795";
    let part_2: isize = stop_at_basement(&input, 2);
    let duration = start.elapsed();
    print_part_2(&part_2.to_string(), &known_answer, duration);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let input = "(())";
        let answer: isize = stop_at_basement(&input, 1);
        assert_eq!(answer, 0);
    }

    #[test]
    fn test_example_2() {
        let input = "()()";
        let answer: isize = stop_at_basement(&input, 1);
        assert_eq!(answer, 0);
    }

    #[test]
    fn test_example_3() {
        let input = "(((";
        let answer: isize = stop_at_basement(&input, 1);
        assert_eq!(answer, 3);
    }

    #[test]
    fn test_example_4() {
        let input = "(()(()(";
        let answer: isize = stop_at_basement(&input, 1);
        assert_eq!(answer, 3);
    }

    #[test]
    fn test_example_5() {
        let input = "))(((((";
        let answer: isize = stop_at_basement(&input, 1);
        assert_eq!(answer, 3);
    }

    #[test]
    fn test_example_6() {
        let input = "())";
        let answer: isize = stop_at_basement(&input, 1);
        assert_eq!(answer, -1);
    }

    #[test]
    fn test_example_7() {
        let input = "))(";
        let answer: isize = stop_at_basement(&input, 1);
        assert_eq!(answer, -1);
    }

    #[test]
    fn test_example_8() {
        let input = ")))";
        let answer: isize = stop_at_basement(&input, 1);
        assert_eq!(answer, -3);
    }

    #[test]
    fn test_example_9() {
        let input = ")())())";
        let answer: isize = stop_at_basement(&input, 1);
        assert_eq!(answer, -3);
    }

    #[test]
    fn test_example_10() {
        let input = ")";
        let answer: isize = stop_at_basement(&input, 2);
        assert_eq!(answer, 1);
    }

    #[test]
    fn test_example_11() {
        let input = "()())";
        let answer: isize = stop_at_basement(&input, 2);
        assert_eq!(answer, 5);
    }
}
