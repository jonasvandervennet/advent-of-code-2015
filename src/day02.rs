use crate::util::{print_part_1, print_part_2};
use regex::Regex;
use std::cmp;
use std::fs::read_to_string;
use std::time::Instant;

fn wrap_present(input: &str, part: usize) -> usize {
    let re: Regex = Regex::new(r"(\d*)x(\d*)x(\d*)").unwrap();

    let mut total = 0;
    for cap in re.captures_iter(input) {
        let l = &cap[1].parse::<usize>().unwrap();
        let w = &cap[2].parse::<usize>().unwrap();
        let h = &cap[3].parse::<usize>().unwrap();

        if part == 1 {
            let side1 = l * w;
            let side2 = w * h;
            let side3 = h * l;

            let surface_area = 2 * (side1 + side2 + side3);
            total += surface_area + cmp::min(side1, cmp::min(side2, side3));
        } else {
            let volume = l * w * h;

            let perimeter1 = 2 * (l + w);
            let perimeter2 = 2 * (w + h);
            let perimeter3 = 2 * (h + l);

            total += volume + cmp::min(perimeter1, cmp::min(perimeter2, perimeter3));
        }
    }
    total
}

pub fn main() {
    let input = read_to_string("inputs/day02.txt").expect("Input not found..");
    // PART 1
    let start = Instant::now();
    let known_answer = "1606483";
    let part_1: usize = wrap_present(&input, 1);
    let duration = start.elapsed();
    print_part_1(&part_1.to_string(), &known_answer, duration);

    // PART 2
    let start = Instant::now();
    let known_answer = "3842356";
    let part_2: usize = wrap_present(&input, 2);
    let duration = start.elapsed();
    print_part_2(&part_2.to_string(), &known_answer, duration);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let input = "2x3x4";
        let answer = wrap_present(&input, 1);
        assert_eq!(answer, 58);
    }

    #[test]
    fn test_example_2() {
        let input = "1x1x10";
        let answer = wrap_present(&input, 1);
        assert_eq!(answer, 43);
    }

    #[test]
    fn test_example_3() {
        let input = "2x3x4";
        let answer = wrap_present(&input, 2);
        assert_eq!(answer, 34);
    }

    #[test]
    fn test_example_4() {
        let input = "1x1x10";
        let answer = wrap_present(&input, 2);
        assert_eq!(answer, 14);
    }
}
