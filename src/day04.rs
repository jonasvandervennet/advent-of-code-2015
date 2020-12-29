use crate::util::{print_part_1, print_part_2};
use md5::{Digest, Md5};
use std::time::Instant;

fn mine_coin(key: &str, part: usize) -> usize {
    let mut suffix = 0;
    loop {
        let mut hasher = Md5::new();
        hasher.update(format!("{}{}", key, suffix));
        let result = &hasher.finalize()[..3];
        // either 5 or 6 leading zeros, depending on the part
        let part_dependent = (part == 1 && result[2] < 16) || (part == 2 && result[2] == 0);
        if result[0] == 0 && result[1] == 0 && part_dependent {
            break;
        }
        suffix += 1;
    }
    suffix
}

pub fn main() {
    let input = "ckczppom";
    // PART 1
    let start = Instant::now();
    let known_answer = "117946";
    let part_1: usize = mine_coin(&input, 1);
    let duration = start.elapsed();
    print_part_1(&part_1.to_string(), &known_answer, duration);

    // PART 2
    let start = Instant::now();
    let known_answer = "3938038";
    let part_2: usize = mine_coin(&input, 2);
    let duration = start.elapsed();
    print_part_2(&part_2.to_string(), &known_answer, duration);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let input = "abcdef";
        let answer = mine_coin(&input, 1);
        assert_eq!(answer, 609043);
    }
}
