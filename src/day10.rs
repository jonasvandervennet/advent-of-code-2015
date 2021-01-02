use crate::util::{print_part_1, print_part_2};
use std::time::Instant;

fn look_and_say(input: &str, iterations: usize) -> usize {
    let mut sequence: Vec<u32> = input.chars().map(|c| c.to_digit(10).unwrap()).collect();

    for _ in 0..iterations {
        let mut new_sequence = vec![];
        let mut count = 0;
        let mut curr = sequence[0];
        for &c in sequence.iter() {
            if c == curr {
                count += 1;
            } else {
                new_sequence.push(count);
                new_sequence.push(curr);
                count = 1;
                curr = c;
            }
        }
        // last element should be counted too
        new_sequence.push(count);
        new_sequence.push(curr);

        sequence = new_sequence;
    }
    sequence.len()
}

pub fn main() {
    let input = "1113222113";
    // PART 1
    let start = Instant::now();
    let known_answer = "252594";
    let part_1: usize = look_and_say(&input, 40);
    let duration = start.elapsed();
    print_part_1(&part_1.to_string(), &known_answer, duration);

    // PART 2
    let start = Instant::now();
    let known_answer = "3579328";
    let part_2: usize = look_and_say(&input, 50);
    let duration = start.elapsed();
    print_part_2(&part_2.to_string(), &known_answer, duration);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let input = "1";
        let answer = look_and_say(&input, 5);
        assert_eq!(answer, 6);
    }
}
