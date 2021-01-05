use crate::util::{print_part_1, print_part_2};
use itertools::Itertools;
use std::time::Instant;

fn get_divisors(x: usize, limited_to_50: bool) -> Vec<usize> {
    (1..=(x as f64).sqrt() as usize)
        .filter(|&y| x % y == 0)
        .map(|y| vec![y, x / y])
        .flatten()
        .unique()
        .filter(|&y| !limited_to_50 || y * 50 > x)
        .collect()
}

fn find_house(lower_bound: usize, part: usize) -> usize {
    // each deliverer delivers a factor 10 or 11
    let lower_bound = lower_bound / if part == 1 { 10 } else { 11 };

    let mut i = 1;
    loop {
        let div_sum: usize = get_divisors(i, part == 2).iter().sum();
        if div_sum >= lower_bound {
            return i;
        }
        i += 1;
    }
}

pub fn main() {
    let input = 34000000;
    // PART 1
    let start = Instant::now();
    let known_answer = "786240";
    let part_1: usize = find_house(input, 1);
    let duration = start.elapsed();
    print_part_1(&part_1.to_string(), &known_answer, duration);

    // PART 2
    let start = Instant::now();
    let known_answer = "831600";
    let part_2: usize = find_house(input, 2);
    let duration = start.elapsed();
    print_part_2(&part_2.to_string(), &known_answer, duration);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let input = 60;
        let answer = find_house(input, 1);
        assert_eq!(answer, 4);
    }

    #[test]
    fn test_example_2() {
        let input = 130;
        let answer = find_house(input, 1);
        assert_eq!(answer, 8);
    }

    #[test]
    fn test_example_2_1() {
        let input = 130;
        let answer = find_house(input, 2);
        assert_eq!(answer, 6);
    }
}
