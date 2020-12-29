use crate::util::{print_part_1, print_part_2};
use std::fs::read_to_string;
use std::time::Instant;
use std::collections::HashSet;

fn santa_travel(input: &str, part: usize) -> usize {
    let mut santa_index = 0;
    let mut coll = HashSet::new();

    let mut x = [0, 0];
    let mut y =  [0, 0];
    coll.insert((x[santa_index],y[santa_index]));


    for dir in input.chars() {
        match dir {
            '<' => {x[santa_index]-= 1;}
            '>' => {x[santa_index] += 1;}
            '^' => {y[santa_index] += 1;}
            'v' => {y[santa_index] -= 1;}
            _ => unreachable!()
        }
        coll.insert((x[santa_index],y[santa_index]));

        if part == 2 {santa_index ^= 1; // flip to next santa
        }
    }
    coll.len()
}

pub fn main() {
    let input = read_to_string("inputs/day03.txt").expect("Input not found..");
    // PART 1
    let start = Instant::now();
    let known_answer = "2572";
    let part_1: usize = santa_travel(&input, 1);
    let duration = start.elapsed();
    print_part_1(&part_1.to_string(), &known_answer, duration);

    // PART 2
    let start = Instant::now();
    let known_answer = "2631";
    let part_2: usize = santa_travel(&input, 2);
    let duration = start.elapsed();
    print_part_2(&part_2.to_string(), &known_answer, duration);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let input = ">";
        let answer = santa_travel(&input, 1);
        assert_eq!(answer, 2);
    }

    #[test]
    fn test_example_2() {
        let input = "^>v<";
        let answer = santa_travel(&input, 1);
        assert_eq!(answer, 4);
    }

    #[test]
    fn test_example_3() {
        let input = "^v^v^v^v^v";
        let answer = santa_travel(&input, 1);
        assert_eq!(answer, 2);
    }
    #[test]
    fn test_example_4() {
        let input = "^v";
        let answer = santa_travel(&input, 2);
        assert_eq!(answer, 3);
    }

    #[test]
    fn test_example_5() {
        let input = "^>v<";
        let answer = santa_travel(&input, 2);
        assert_eq!(answer, 3);
    }

    #[test]
    fn test_example_6() {
        let input = "^v^v^v^v^v";
        let answer = santa_travel(&input, 2);
        assert_eq!(answer, 11);
    }
}
