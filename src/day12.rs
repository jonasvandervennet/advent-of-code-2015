use crate::util::{print_part_1, print_part_2};
use regex::Regex;
use serde_json::Value;
use std::fs::read_to_string;
use std::time::Instant;

// Part one asked to add all the numbers, so without parsing I did it with a regex
fn sum_numbers_json_cheaty_part1(input: &str) -> i64 {
    let mut acc = 0;

    let re: Regex = Regex::new(r"(\d{1,}|-\d{1,})[,}\]]").unwrap();
    for cap in re.captures_iter(input) {
        acc += &cap[1].parse::<i64>().unwrap();
    }

    acc
}
// ------------------------------------------------------------------------------

fn sum_numbers_json(value: &Value, part: usize) -> i64 {
    let mut acc = 0;
    if value.is_object() {
        let value = value.as_object().unwrap();
        if part == 2 {
            for (_, value) in value.iter() {
                if value == "red" {
                    return 0;
                }
            }
        }
        for (_, value) in value.iter() {
            acc += handle_value(value, part);
        }
    } else if value.is_array() {
        let value = value.as_array().unwrap();
        for value in value.iter() {
            acc += handle_value(value, part);
        }
    }
    acc
}

fn handle_value(value: &Value, part: usize) -> i64 {
    if value.is_number() {
        value.as_i64().unwrap()
    } else {
        sum_numbers_json(value, part)
    }
}

fn parse_inputstring(input: &str) -> Value {
    match serde_json::from_str(&input) {
        Result::Ok(val) => val,
        Result::Err(err) => {
            panic!("Unable to parse json: {}", err)
        }
    }
}

pub fn main() {
    let input = read_to_string("inputs/day12.txt").expect("Input not found..");
    let input = parse_inputstring(&input);

    // PART 1
    let start = Instant::now();
    let known_answer = "119433";
    let part_1: i64 = sum_numbers_json(&input, 1);
    let duration = start.elapsed();
    print_part_1(&part_1.to_string(), &known_answer, duration);

    // PART 2
    let start = Instant::now();
    let known_answer = "68466";
    let part_2: i64 = sum_numbers_json(&input, 2);
    let duration = start.elapsed();
    print_part_2(&part_2.to_string(), &known_answer, duration);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let input = "[1,2,3]";
        let input = parse_inputstring(&input);
        let answer = sum_numbers_json(&input, 1);
        assert_eq!(answer, 6);
    }

    #[test]
    fn test_example_2() {
        let input = r#"{"a":2,"b":4}"#;
        let input = parse_inputstring(&input);
        let answer = sum_numbers_json(&input, 1);
        assert_eq!(answer, 6);
    }

    #[test]
    fn test_example_3() {
        let input = r#"{"a":2,"b":4}"#;
        let input = parse_inputstring(&input);
        let answer = sum_numbers_json(&input, 1);
        assert_eq!(answer, 6);
    }

    #[test]
    fn test_example_4() {
        let input = r#"{"a":{"b":4},"c":-1}"#;
        let input = parse_inputstring(&input);
        let answer = sum_numbers_json(&input, 1);
        assert_eq!(answer, 3);
    }

    #[test]
    fn test_example_5() {
        let input = r#"{"a":[-1,1]}"#;
        let input = parse_inputstring(&input);
        let answer = sum_numbers_json(&input, 1);
        assert_eq!(answer, 0);
    }

    #[test]
    fn test_example_6() {
        let input = r#"[-1,{"a":1}]"#;
        let input = parse_inputstring(&input);
        let answer = sum_numbers_json(&input, 1);
        assert_eq!(answer, 0);
    }

    #[test]
    fn test_example_7() {
        let input = r#"[]"#;
        let input = parse_inputstring(&input);
        let answer = sum_numbers_json(&input, 1);
        assert_eq!(answer, 0);
    }

    #[test]
    fn test_example_8() {
        let input = r#"{}"#;
        let input = parse_inputstring(&input);
        let answer = sum_numbers_json(&input, 1);
        assert_eq!(answer, 0);
    }

    #[test]
    fn test_example_2_1() {
        let input = r#"[1,2,3]"#;
        let input = parse_inputstring(&input);
        let answer = sum_numbers_json(&input, 2);
        assert_eq!(answer, 6);
    }

    #[test]
    fn test_example_2_2() {
        let input = r#"{"a": [1,{"c":"red","b":2},3]}"#;
        let input = parse_inputstring(&input);
        let answer = sum_numbers_json(&input, 2);
        assert_eq!(answer, 4);
    }

    #[test]
    fn test_example_2_3() {
        let input = r#"{"d":"red","e":[1,2,3,4],"f":5}"#;
        let input = parse_inputstring(&input);
        let answer = sum_numbers_json(&input, 2);
        assert_eq!(answer, 0);
    }

    #[test]
    fn test_example_2_4() {
        let input = r#"[1,"red",5]"#;
        let input = parse_inputstring(&input);
        let answer = sum_numbers_json(&input, 2);
        assert_eq!(answer, 6);
    }

    #[test]
    fn test_example_2_5() {
        let input = r#"{"a": [1,{"b":2},3]}"#;
        let input = parse_inputstring(&input);
        let answer = sum_numbers_json(&input, 2);
        assert_eq!(answer, 6);
    }
}
