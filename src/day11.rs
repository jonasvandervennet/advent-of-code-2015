use crate::util::{print_part_1, print_part_2};
use std::time::Instant;

fn increment(sequence: &mut Vec<u8>) {
    for i in (0..sequence.len()).rev() {
        // ascii: a = 97 --> z = 122
        if sequence[i] == 122 {
            sequence[i] = 97;
        } else {
            // could early check here for lettes i,o,l?
            sequence[i] += 1;
            break;
        }
    }
}

fn find_new_password(old_password: &str) -> String {
    let mut sequence: Vec<u8> = old_password.chars().map(|c| c as u8).collect();

    'new_sequence: loop {
        increment(&mut sequence);

        // RULE: 1: Passwords may not contain the letters i, o, or l
        //                                               105,111, 108
        for d in &[105, 111, 108] {
            if sequence.contains(d) {
                continue 'new_sequence;
            }
        }

        // RULE: 2: Passwords must contain at least two pairs of letters
        let mut skip_next = false;
        let mut num_pairs = 0;
        for window in sequence.windows(2) {
            if skip_next {
                skip_next = false;
                continue;
            }
            if window[0] == window[1] {
                num_pairs += 1;
                skip_next = true;
            }
        }
        if num_pairs < 2 {
            continue 'new_sequence;
        }

        // RULE: 3: Passwords must include one increasing straight of at least three
        let mut found_straight = false;
        for window in sequence.windows(3) {
            if window[0] + 1 == window[1] && window[1] + 1 == window[2] {
                found_straight = true;
                break;
            }
        }
        if !found_straight {
            continue 'new_sequence;
        }

        break;
    }
    sequence.iter().map(|&c| c as char).collect::<String>()
}

pub fn main() {
    let input = "hepxcrrq";
    // PART 1
    let start = Instant::now();
    let known_answer = "hepxxyzz";
    let part_1: String = find_new_password(&input);
    let duration = start.elapsed();
    print_part_1(&part_1.to_string(), &known_answer, duration);

    // PART 2
    let start = Instant::now();
    let known_answer = "heqaabcc";
    let part_2: String = find_new_password(&part_1);
    let duration = start.elapsed();
    print_part_2(&part_2.to_string(), &known_answer, duration);
}
