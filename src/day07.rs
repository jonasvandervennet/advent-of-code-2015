use crate::util::{print_part_1, print_part_2};
use regex::Regex;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::time::Instant;

#[derive(PartialEq)]
enum OP {
    INV,
    OP,
    ASS,
}

struct Command {
    kind: OP,
    args: Vec<String>,
}

impl Command {
    fn new(line: &str) -> Self {
        let re: Regex = Regex::new(r"^NOT (.{1,}) -> (.{1,})$").unwrap();
        if re.is_match(line) {
            // Inversion
            let cap = re.captures_iter(line).next().unwrap();
            let lhs = cap[1].parse::<String>().unwrap();
            let rhs = cap[2].parse::<String>().unwrap();
            return Self {
                kind: OP::INV,
                args: vec![lhs, rhs],
            };
        }

        let re: Regex = Regex::new(r"^(.{1,}|\d{1,}) (.{1,}) (.{1,}|\d{1,}) -> (.{1,})$").unwrap();
        if re.is_match(line) {
            // Operation
            let cap = re.captures_iter(line).next().unwrap();
            let lhs = cap[1].parse::<String>().unwrap();
            let op = cap[2].parse::<String>().unwrap();
            let rhs = cap[3].parse::<String>().unwrap();
            let ass = cap[4].parse::<String>().unwrap();

            return Self {
                kind: OP::OP,
                args: vec![lhs, op, rhs, ass],
            };
        }

        let re: Regex = Regex::new(r"^(.{1,}|\d{1,}) -> (.{1,})$").unwrap();
        if re.is_match(line) {
            // Assignment
            let cap = re.captures_iter(line).next().unwrap();
            let lhs = cap[1].parse::<String>().unwrap();
            let rhs = cap[2].parse::<String>().unwrap();
            return Self {
                kind: OP::ASS,
                args: vec![lhs, rhs],
            };
        }

        panic!("{}", line);
    }
}

fn execute_circuit(input: &str, part: usize) -> HashMap<String, u16> {
    let mut registry: HashMap<String, u16> = HashMap::new();
    let mut commands: Vec<Command> = input.lines().map(|line| Command::new(line)).collect();
    let mut processed = vec![false; commands.len()];

    let mut part2_intervention = false;
    loop {
        let mut processed_one = false;
        'cmd_loop: for (i, cmd) in commands.iter().enumerate() {
            if processed[i] {
                continue 'cmd_loop;
            }
            match cmd.kind {
                OP::INV => {
                    let lhs = match registry.get(&cmd.args[0]) {
                        Some(n) => *n,
                        None => {
                            continue 'cmd_loop;
                        } // leave processing, this variable was not assigned yet
                    };
                    registry
                        .entry(cmd.args[1].to_string())
                        .and_modify(|e| *e = !lhs)
                        .or_insert(!lhs);
                }
                OP::OP => {
                    let lhs = match cmd.args[0].parse::<u16>() {
                        Ok(x) => x,
                        Err(_) => match registry.get(&cmd.args[0]) {
                            Some(n) => *n,
                            None => {
                                continue 'cmd_loop;
                            } // leave processing, this variable was not assigned yet
                        },
                    };
                    let rhs = match cmd.args[2].parse::<u16>() {
                        Ok(x) => x,
                        Err(_) => match registry.get(&cmd.args[2]) {
                            Some(n) => *n,
                            None => {
                                continue 'cmd_loop;
                            } // leave processing, this variable was not assigned yet
                        },
                    };

                    let result = match &cmd.args[1][..] {
                        "AND" => lhs & rhs,
                        "OR" => lhs | rhs,
                        "LSHIFT" => lhs << rhs,
                        "RSHIFT" => lhs >> rhs,
                        _ => unreachable!(),
                    };
                    registry
                        .entry(cmd.args[3].to_string())
                        .and_modify(|e| *e = result)
                        .or_insert(result);
                }
                OP::ASS => {
                    let lhs = match cmd.args[0].parse::<u16>() {
                        Ok(x) => x,
                        Err(_) => match registry.get(&cmd.args[0]) {
                            Some(n) => *n,
                            None => {
                                continue 'cmd_loop;
                            } // leave processing, this variable was not assigned yet
                        },
                    };
                    registry
                        .entry(cmd.args[1].to_string())
                        .and_modify(|e| *e = lhs)
                        .or_insert(lhs);
                }
            }

            processed_one = true;
            processed[i] = true;
        }

        // stop as soon as no new commands are executed
        if !processed_one {
            if part == 2 && !part2_intervention {
                let b_index = commands
                    .iter()
                    .position(|cmd| cmd.kind == OP::ASS && cmd.args[1] == "b")
                    .unwrap();
                commands[b_index] = Command::new(&format!("{} -> b", registry.get("a").unwrap()));
                processed = vec![false; commands.len()];
                registry.clear();

                part2_intervention = true;
            } else {
                return registry;
            }
        }
    }
}

pub fn main() {
    let input = read_to_string("inputs/day07.txt").expect("Input not found..");
    // PART 1
    let start = Instant::now();
    let known_answer = "46065";
    let part_1: u16 = *execute_circuit(&input, 1).get("a").unwrap();
    let duration = start.elapsed();
    print_part_1(&part_1.to_string(), &known_answer, duration);

    // PART 2
    let start = Instant::now();
    let known_answer = "14134";
    let part_2: u16 = *execute_circuit(&input, 2).get("a").unwrap();
    let duration = start.elapsed();
    print_part_2(&part_2.to_string(), &known_answer, duration);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let input = "123 -> x\n456 -> y\nx AND y -> d\nx OR y -> e\nx LSHIFT 2 -> f\ny RSHIFT 2 -> g\nNOT x -> h\nNOT y -> i";
        let answer = execute_circuit(&input, 1);
        println!("{:?}", answer);
        for (wire, value) in &[
            ("d", 72),
            ("e", 507),
            ("f", 492),
            ("g", 114),
            ("h", 65412),
            ("i", 65079),
            ("x", 123),
            ("y", 456),
        ] {
            assert_eq!(answer.get(&wire.to_string()).unwrap(), value);
        }
    }

    #[test]
    fn test_example_1_out_of_order() {
        let input = "456 -> y\nx AND y -> d\nx OR y -> e\nx LSHIFT 2 -> f\ny RSHIFT 2 -> g\n123 -> x\nNOT x -> h\nNOT y -> i";
        let answer = execute_circuit(&input, 1);
        println!("{:?}", answer);
        for (wire, value) in &[
            ("d", 72),
            ("e", 507),
            ("f", 492),
            ("g", 114),
            ("h", 65412),
            ("i", 65079),
            ("x", 123),
            ("y", 456),
        ] {
            assert_eq!(answer.get(&wire.to_string()).unwrap(), value);
        }
    }
}
