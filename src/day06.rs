use crate::util::{print_part_1, print_part_2};
use regex::Regex;
use std::fs::read_to_string;
use std::time::Instant;

fn toggle_lights(input: &str, part: usize) -> Vec<Vec<usize>> {
    let mut grid = vec![vec![0; 1000]; 1000];
    let re: Regex = Regex::new(r"(.*) (\d*),(\d*) .* (\d*),(\d*)").unwrap();
    for cap in re.captures_iter(input) {
        let cmd = &cap[1].parse::<String>().unwrap();
        let &x0 = &cap[2].parse::<usize>().unwrap();
        let &y0 = &cap[3].parse::<usize>().unwrap();
        let &x1 = &cap[4].parse::<usize>().unwrap();
        let &y1 = &cap[5].parse::<usize>().unwrap();

        match cmd.chars().count() {
            6 => {
                for x in x0..=x1 {
                    for y in y0..=y1 {
                        if part == 1 {
                            grid[x][y] ^= 1;
                        } else {
                            grid[x][y] += 2;
                        }
                    }
                }
            }
            7 => {
                for x in x0..=x1 {
                    for y in y0..=y1 {
                        if part == 1 {
                            grid[x][y] = 1;
                        } else {
                            grid[x][y] += 1;
                        }
                    }
                }
            }
            8 => {
                for x in x0..=x1 {
                    for y in y0..=y1 {
                        if part == 1 {
                            grid[x][y] = 0;
                        } else {
                            grid[x][y] = (grid[x][y] as usize).checked_sub(1).unwrap_or(0);
                        }
                    }
                }
            }
            _ => unreachable!(),
        };
    }
    grid
}

fn count_lights(input: &str, part: usize) -> usize {
    let grid = toggle_lights(input, part);
    let mut count = 0;
    for row in grid.iter() {
        for &light in row.iter() {
            count += light;
        }
    }
    count
}

pub fn main() {
    let input = read_to_string("inputs/day06.txt").expect("Input not found..");
    // PART 1
    let start = Instant::now();
    let known_answer = "400410";
    let part_1: usize = count_lights(&input, 1);
    let duration = start.elapsed();
    print_part_1(&part_1.to_string(), &known_answer, duration);

    // PART 2
    let start = Instant::now();
    let known_answer = "15343601";
    let part_2: usize = count_lights(&input, 2);
    let duration = start.elapsed();
    print_part_2(&part_2.to_string(), &known_answer, duration);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let input = "turn on 0,0 through 999,999";
        let answer = count_lights(&input, 1);
        assert_eq!(answer, 1_000_000);
    }

    #[test]
    fn test_example_2() {
        let input = "turn on 0,0 through 0,0";
        let answer = count_lights(&input, 2);
        assert_eq!(answer, 1);
    }

    #[test]
    fn test_example_3() {
        let input = "toggle 0,0 through 999,999";
        let answer = count_lights(&input, 2);
        assert_eq!(answer, 2_000_000);
    }
}
