use crate::util::{print_part_1, print_part_2};
use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::time::Instant;

type Happinessmap = HashMap<String, HashMap<String, isize>>;

fn parse_gains(input: &str) -> Happinessmap {
    let mut coll: Happinessmap = HashMap::new();
    let re: Regex = Regex::new(
        r"(\w{1,}) would (gain|lose) (\d{1,}) happiness units by sitting next to (\w{1,}).",
    )
    .unwrap();
    for cap in re.captures_iter(input) {
        let src = &cap[1];
        let gain_lose = &cap[2];
        let mut value = cap[3].parse::<isize>().unwrap();
        let dest = &cap[4];

        if gain_lose == "lose" {
            value = -value;
        }

        coll.entry(src.to_string())
            .and_modify(|e| {
                e.insert(dest.to_string(), value);
            })
            .or_insert({
                let mut map = HashMap::new();
                map.insert(dest.to_string(), value);
                map
            });
    }
    coll
}

fn plan_seating(input: &str, part: usize) -> isize {
    let gains = parse_gains(input);
    let mut places: Vec<&String> = gains.keys().collect();
    let yourself = "yourself".to_string();
    if part == 2 {
        places.push(&yourself);
    }
    let mut optimal_happiness = 0;

    for perm in places.iter().permutations(places.len()) {
        let mut perm = perm;
        perm.extend(vec![perm[0]]); // wrap around
        let mut happiness = 0;
        for window in perm.windows(2) {
            // indifferent case
            if window.contains(&&&yourself) {
                continue;
            }
            happiness += gains
                .get(&window[0].to_string())
                .unwrap()
                .get(&window[1].to_string())
                .unwrap();
            // also reversed happiness change
            happiness += gains
                .get(&window[1].to_string())
                .unwrap()
                .get(&window[0].to_string())
                .unwrap();
        }
        if happiness > optimal_happiness {
            optimal_happiness = happiness;
        }
    }

    optimal_happiness
}

pub fn main() {
    let input = read_to_string("inputs/day13.txt").expect("Input not found..");

    // PART 1
    let start = Instant::now();
    let known_answer = "709";
    let part_1: isize = plan_seating(&input, 1);
    let duration = start.elapsed();
    print_part_1(&part_1.to_string(), &known_answer, duration);

    // PART 2
    let start = Instant::now();
    let known_answer = "668";
    let part_2: isize = plan_seating(&input, 2);
    let duration = start.elapsed();
    print_part_2(&part_2.to_string(), &known_answer, duration);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let input = read_to_string("inputs/day13_test.txt").expect("Input not found..");
        let answer = plan_seating(&input, 1);
        assert_eq!(answer, 330);
    }
}
