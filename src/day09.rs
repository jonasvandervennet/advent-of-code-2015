use crate::util::{print_part_1, print_part_2};
use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::time::Instant;

type Distmap = HashMap<String, HashMap<String, usize>>;

fn parse_distances(input: &str) -> Distmap {
    let mut coll: Distmap = HashMap::new();
    for line in input.lines() {
        let re: Regex = Regex::new(r"(\w*) to (\w*) = (\d*)").unwrap();
        let cap = re.captures_iter(line).next().unwrap();

        let src = &cap[1];
        let dest = &cap[2];
        let distance = cap[3].parse::<usize>().unwrap();

        coll.entry(src.to_string())
            .and_modify(|e| {
                e.insert(dest.to_string(), distance);
            })
            .or_insert({
                let mut map = HashMap::new();
                map.insert(dest.to_string(), distance);
                map
            });

        // reverse reference
        coll.entry(dest.to_string())
            .and_modify(|e| {
                e.insert(src.to_string(), distance);
            })
            .or_insert({
                let mut map = HashMap::new();
                map.insert(src.to_string(), distance);
                map
            });
    }
    coll
}

fn plan_route(input: &str, part: usize) -> usize {
    let distances = parse_distances(input);
    let places: Vec<&String> = distances.keys().collect();
    let mut optimal_dist = if part == 1 { usize::MAX } else { 0 };
    'poss_solutions: for perm in places.iter().permutations(places.len()) {
        let mut dist = 0;
        for window in perm.windows(2) {
            dist += distances
                .get(&window[0].to_string())
                .unwrap()
                .get(&window[1].to_string())
                .unwrap();

            if part == 1 && dist > optimal_dist {
                continue 'poss_solutions;
            }
        }
        if part == 1 && dist < optimal_dist {
            optimal_dist = dist;
        }
        if part == 2 && dist > optimal_dist {
            optimal_dist = dist;
        }
    }

    optimal_dist
}

pub fn main() {
    let input = read_to_string("inputs/day09.txt").expect("Input not found..");
    // PART 1
    let start = Instant::now();
    let known_answer = "251";
    let part_1: usize = plan_route(&input, 1);
    let duration = start.elapsed();
    print_part_1(&part_1.to_string(), &known_answer, duration);

    // PART 2
    let start = Instant::now();
    let known_answer = "898";
    let part_2: usize = plan_route(&input, 2);
    let duration = start.elapsed();
    print_part_2(&part_2.to_string(), &known_answer, duration);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let input = "London to Dublin = 464\nLondon to Belfast = 518\nDublin to Belfast = 141";
        let answer = plan_route(&input, 1);
        assert_eq!(answer, 605);
    }
    #[test]
    fn test_example_2() {
        let input = "London to Dublin = 464\nLondon to Belfast = 518\nDublin to Belfast = 141";
        let answer = plan_route(&input, 2);
        assert_eq!(answer, 982);
    }
}
