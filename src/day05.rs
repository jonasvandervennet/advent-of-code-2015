use crate::util::{print_part_1, print_part_2};
use fancy_regex::Regex;
use std::fs::read_to_string;
use std::time::Instant;

fn rate_nice_part_1(word: &str) -> bool {
    // should not contain these substrings
    let undesirables = ["ab", "cd", "pq", "xy"];
    for test in &undesirables {
        if word.contains(test) {
            return false;
        }
    }

    // should contain a repeated character
    let re: Regex = Regex::new(r"(.)\1{1}").unwrap();
    if !re.is_match(word).unwrap() {
        return false;
    }

    // should contain three or more vowels
    let mut total_vowels = 0;
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    for &test in &vowels {
        total_vowels += word.chars().filter(|&c| c == test).count();
        if total_vowels >= 3 {
            break;
        }
    }

    total_vowels >= 3
}

fn rate_nice_part_2(word: &str) -> bool {
    // should contain a pair of characters at least twice somewhere in the word
    let re: Regex = Regex::new(r"(..).*\1{1}").unwrap();
    if !re.is_match(word).unwrap() {
        return false;
    }

    // should contain (a character, then another, then the first again) at least once
    let re: Regex = Regex::new(r"(.).{1}\1{1}").unwrap();
    if !re.is_match(word).unwrap() {
        return false;
    }

    true
}

fn count_nice(input: &str, part: usize) -> usize {
    input
        .lines()
        .filter(|&line| {
            if part == 1 {
                rate_nice_part_1(line)
            } else {
                rate_nice_part_2(line)
            }
        })
        .count()
}

pub fn main() {
    let input = read_to_string("inputs/day05.txt").expect("Input not found..");
    // PART 1
    let start = Instant::now();
    let known_answer = "236";
    let part_1: usize = count_nice(&input, 1);
    let duration = start.elapsed();
    print_part_1(&part_1.to_string(), &known_answer, duration);

    // PART 2
    let start = Instant::now();
    let known_answer = "51";
    let part_2: usize = count_nice(&input, 2);
    let duration = start.elapsed();
    print_part_2(&part_2.to_string(), &known_answer, duration);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let input = "ugknbfddgicrmopn";
        let answer = rate_nice_part_1(&input);
        assert_eq!(answer, true);
    }

    #[test]
    fn test_example_2() {
        let input = "aaa";
        let answer = rate_nice_part_1(&input);
        assert_eq!(answer, true);
    }

    #[test]
    fn test_example_3() {
        let input = "jchzalrnumimnmhp";
        let answer = rate_nice_part_1(&input);
        assert_eq!(answer, false);
    }

    #[test]
    fn test_example_4() {
        let input = "haegwjzuvuyypxyu";
        let answer = rate_nice_part_1(&input);
        assert_eq!(answer, false);
    }

    #[test]
    fn test_example_5() {
        let input = "dvszwmarrgswjxmb";
        let answer = rate_nice_part_1(&input);
        assert_eq!(answer, false);
    }

    #[test]
    fn test_example_6() {
        let input = "qjhvhtzxzqqjkmpb";
        let answer = rate_nice_part_2(&input);
        assert_eq!(answer, true);
    }

    #[test]
    fn test_example_7() {
        let input = "xxyxx";
        let answer = rate_nice_part_2(&input);
        assert_eq!(answer, true);
    }

    #[test]
    fn test_example_8() {
        let input = "uurcxstgmygtbstg";
        let answer = rate_nice_part_2(&input);
        assert_eq!(answer, false);
    }

    #[test]
    fn test_example_9() {
        let input = "ieodomkazucvgmuy";
        let answer = rate_nice_part_2(&input);
        assert_eq!(answer, false);
    }
}
