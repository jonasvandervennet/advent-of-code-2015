use crate::util::{print_part_1, print_part_2};
use itertools::Itertools;
use regex::Regex;
use std::cmp::max;
use std::fs::read_to_string;
use std::time::Instant;

struct Ingredient {
    capacity: isize,
    durability: isize,
    flavor: isize,
    texture: isize,
    calories: isize,
}

fn balance_teaspoons(input: &str, part: usize) -> usize {
    let re: Regex = Regex::new(
    r"\w{1,}: .* (-{0,1}\d{1,}), .* (-{0,1}\d{1,}), .* (-{0,1}\d{1,}), .* (-{0,1}\d{1,}), .* (-{0,1}\d{1,})",
).unwrap();
    let ingredients: Vec<Ingredient> = re
        .captures_iter(input)
        .map(|cap| Ingredient {
            capacity: cap[1].parse::<isize>().unwrap(),
            durability: cap[2].parse::<isize>().unwrap(),
            flavor: cap[3].parse::<isize>().unwrap(),
            texture: cap[4].parse::<isize>().unwrap(),
            calories: cap[5].parse::<isize>().unwrap(),
        })
        .collect();

    let mut optimal_score = 0;
    for comb in (0..=100).combinations_with_replacement(ingredients.len()) {
        if comb.iter().sum::<isize>() != 100 {
            continue;
        }
        // permutations of combinations, maybe optimize this?
        for weights in comb.iter().permutations(comb.len()) {
            let mut total_ingredient = Ingredient {
                capacity: 0,
                durability: 0,
                flavor: 0,
                texture: 0,
                calories: 0,
            };
            for (i, ing) in ingredients.iter().enumerate() {
                total_ingredient.capacity += weights[i] * ing.capacity;
                total_ingredient.durability += weights[i] * ing.durability;
                total_ingredient.flavor += weights[i] * ing.flavor;
                total_ingredient.texture += weights[i] * ing.texture;
                total_ingredient.calories += weights[i] * ing.calories;
            }

            let score = (max(0, total_ingredient.capacity)
                * max(0, total_ingredient.durability)
                * max(0, total_ingredient.flavor)
                * max(0, total_ingredient.texture)) as usize;

            if part == 2 {
                if total_ingredient.calories != 500 {
                    continue;
                }
            };
            if score > optimal_score {
                optimal_score = score;
            }
        }
    }
    optimal_score
}

pub fn main() {
    let input = read_to_string("inputs/day15.txt").expect("Input not found..");
    // PART 1
    let start = Instant::now();
    let known_answer = "13882464";
    let part_1: usize = balance_teaspoons(&input, 1);
    let duration = start.elapsed();
    print_part_1(&part_1.to_string(), &known_answer, duration);

    // PART 2
    let start = Instant::now();
    let known_answer = "11171160";
    let part_2: usize = balance_teaspoons(&input, 2);
    let duration = start.elapsed();
    print_part_2(&part_2.to_string(), &known_answer, duration);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let input = "Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8\nCinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3";
        let answer = balance_teaspoons(&input, 1);
        assert_eq!(answer, 62842880);
    }

    #[test]
    fn test_example_2() {
        let input = "Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8\nCinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3";
        let answer = balance_teaspoons(&input, 2);
        assert_eq!(answer, 57600000);
    }
}
