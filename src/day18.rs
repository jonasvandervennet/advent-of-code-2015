use crate::util::{print_part_1, print_part_2};
use std::fs::read_to_string;
use std::time::Instant;

fn iterate_lights(input: &str, iterations: usize, sticky_corners: bool) -> usize {
    let mut grid: Vec<Vec<bool>> = input
        .lines()
        .map(|line| line.chars().map(|c| c == '#').collect())
        .collect();
    let height = grid.len();
    let width = grid[0].len();
    if sticky_corners {
        grid[0][0] = true;
        grid[height - 1][0] = true;
        grid[0][width - 1] = true;
        grid[height - 1][width - 1] = true;
    }

    for _ in 0..iterations {
        let mut new_grid = vec![vec![false; width]; height];
        for (i, row) in grid.iter().enumerate() {
            for (j, &state) in row.iter().enumerate() {
                let mut counter = 0;
                for offsets in &[
                    (-1, -1),
                    (-1, 0),
                    (-1, 1),
                    (1, -1),
                    (1, 0),
                    (1, 1),
                    (0, -1),
                    (0, 1),
                ] {
                    if i == 0 && offsets.0 == -1
                        || i == height - 1 && offsets.0 == 1
                        || j == 0 && offsets.1 == -1
                        || j == width - 1 && offsets.1 == 1
                    {
                        continue;
                    }
                    if grid[(i as isize + offsets.0) as usize][(j as isize + offsets.1) as usize] {
                        counter += 1
                    }
                }
                if state {
                    if counter == 2 || counter == 3 {
                        new_grid[i][j] = true;
                    }
                } else {
                    if counter == 3 {
                        new_grid[i][j] = true;
                    }
                }
            }
        }
        if sticky_corners {
            new_grid[0][0] = true;
            new_grid[height - 1][0] = true;
            new_grid[0][width - 1] = true;
            new_grid[height - 1][width - 1] = true;
        }
        grid = new_grid;
    }
    grid.iter()
        .map(|row| row.iter().filter(|&&x| x).count())
        .sum()
}

pub fn main() {
    let input = read_to_string("inputs/day18.txt").expect("Input not found..");
    // PART 1
    let start = Instant::now();
    let known_answer = "814";
    let part_1: usize = iterate_lights(&input, 100, false);
    let duration = start.elapsed();
    print_part_1(&part_1.to_string(), &known_answer, duration);

    // PART 2
    let start = Instant::now();
    let known_answer = "924";
    let part_2: usize = iterate_lights(&input, 100, true);
    let duration = start.elapsed();
    print_part_2(&part_2.to_string(), &known_answer, duration);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let input = ".#.#.#\n...##.\n#....#\n..#...\n#.#..#\n####..";
        let answer = iterate_lights(&input, 4, false);
        assert_eq!(answer, 4);
    }

    #[test]
    fn test_example_sticky_lights() {
        let input = ".#.#.#\n...##.\n#....#\n..#...\n#.#..#\n####..";
        let answer = iterate_lights(&input, 5, true);
        assert_eq!(answer, 17);
    }
}
