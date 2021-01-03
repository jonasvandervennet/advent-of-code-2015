use crate::util::{print_part_1, print_part_2};
use regex::Regex;
use std::fs::read_to_string;
use std::time::Instant;

struct Reindeer {
    speed: usize,
    speed_time: usize,
    rest_time: usize,
    dist: usize,
    points: usize,
    _time_spent: usize,
    _speed_phase: bool,
}

impl Reindeer {
    fn new(speed: usize, speed_time: usize, rest_time: usize) -> Self {
        Self {
            speed: speed,
            speed_time: speed_time,
            rest_time: rest_time,
            dist: 0,
            points: 0,
            _time_spent: 0,
            _speed_phase: true,
        }
    }
    fn tick(&mut self) {
        if self._speed_phase {
            self.dist += self.speed;
        }
        self._time_spent += 1;
        if self._time_spent
            == if self._speed_phase {
                self.speed_time
            } else {
                self.rest_time
            }
        {
            self._speed_phase = !self._speed_phase;
            self._time_spent = 0;
        }
    }
}

fn race(input: &str, duration: usize, part: usize) -> usize {
    let re: Regex = Regex::new(
        r"\w{1,} can fly (\d{1,}) km/s for (\d{1,}) seconds, but then must rest for (\d{1,}) seconds.",
    )
    .unwrap();

    let mut reindeer: Vec<Reindeer> = re
        .captures_iter(input)
        .map(|cap| {
            let speed = cap[1].parse::<usize>().unwrap();
            let speed_time = cap[2].parse::<usize>().unwrap();
            let rest_time = cap[3].parse::<usize>().unwrap();

            Reindeer::new(speed, speed_time, rest_time)
        })
        .collect();

    for _ in 0..duration {
        let mut max_dist = 0;
        let mut iteration_leaders = vec![];
        for (i, r) in reindeer.iter_mut().enumerate() {
            r.tick();
            if r.dist > max_dist {
                max_dist = r.dist;
                iteration_leaders = vec![i];
            } else if r.dist == max_dist {
                iteration_leaders.push(i);
            }
        }
        for &i in iteration_leaders.iter() {
            reindeer[i].points += 1;
        }
    }

    reindeer
        .iter()
        .map(|deer| if part == 1 { deer.dist } else { deer.points })
        .max()
        .unwrap()
}

pub fn main() {
    let input = read_to_string("inputs/day14.txt").expect("Input not found..");
    // PART 1
    let start = Instant::now();
    let known_answer = "2696";
    let part_1: usize = race(&input, 2503, 1);
    let duration = start.elapsed();
    print_part_1(&part_1.to_string(), &known_answer, duration);

    // PART 2
    let start = Instant::now();
    let known_answer = "1084";
    let part_2: usize = race(&input, 2503, 2);
    let duration = start.elapsed();
    print_part_2(&part_2.to_string(), &known_answer, duration);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let input = "Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.\nDancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.";
        let answer = race(&input, 11, 1);
        assert_eq!(answer, 176);
    }

    #[test]
    fn test_example_2() {
        let input = "Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.\nDancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.";
        let answer = race(&input, 138, 1);
        assert_eq!(answer, 176);
    }

    #[test]
    fn test_example_3() {
        let input = "Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.\nDancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.";
        let answer = race(&input, 1000, 1);
        assert_eq!(answer, 1120);
    }

    #[test]
    fn test_example_2_1() {
        let input = "Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.\nDancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.";
        let answer = race(&input, 1, 2);
        assert_eq!(answer, 1);
    }

    #[test]
    fn test_example_2_2() {
        let input = "Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.\nDancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.";
        let answer = race(&input, 140, 2);
        assert_eq!(answer, 139);
    }

    #[test]
    fn test_example_2_3() {
        let input = "Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.\nDancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.";
        let answer = race(&input, 1000, 2);
        assert_eq!(answer, 689);
    }
}
