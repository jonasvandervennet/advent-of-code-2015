use crate::util::{print_part_1, print_part_2};
use itertools::Itertools;
use regex::Regex;
use std::cmp::max;
use std::time::Instant;

fn fight(player_stats: &(usize, usize, usize), boss_stats: &(usize, usize, usize)) -> bool {
    let player_hp = player_stats.0 as f64;
    let boss_hp = boss_stats.0 as f64;
    let player_damage = max(1, player_stats.1 as i64 - boss_stats.2 as i64) as f64;
    let boss_damage = max(1, boss_stats.1 as i64 - player_stats.2 as i64) as f64;
    return (player_hp / boss_damage).ceil() >= (boss_hp / player_damage).ceil();
}

fn get_optimal_gear_price(
    boss_stats: &str,
    shop_content: &[Vec<(usize, usize, usize)>; 3],
    part: usize,
) -> usize {
    let re: Regex = Regex::new(r".*: (\d+) .*: (\d+) .*: (\d+)").unwrap();
    let cap = re.captures_iter(boss_stats).next().unwrap();
    let boss_stats: (usize, usize, usize) = (
        cap[1].parse().unwrap(),
        cap[2].parse().unwrap(),
        cap[3].parse().unwrap(),
    );

    let mut current_best_price = usize::MAX;
    let mut current_worst_price = 0;
    for &n_rings in &[0, 1, 2] {
        for ring_window in shop_content[2].iter().combinations(n_rings) {
            let mut total_ring = (0, 0, 0);
            for ring in ring_window.iter() {
                total_ring.0 += ring.0;
                total_ring.1 += ring.1;
                total_ring.2 += ring.2;
            }
            for weapon in shop_content[0].iter() {
                // exactly  one weapon
                for armor in shop_content[1].iter() {
                    // 0 or 1 armor piece
                    for &armor_mult in &[0, 1] {
                        let player_price = weapon.0 + armor_mult * armor.0 + total_ring.0;
                        let player_stats = (
                            100,
                            weapon.1 + total_ring.1,
                            armor_mult * armor.2 + total_ring.2,
                        );
                        let win = fight(&player_stats, &boss_stats);
                        if win && player_price < current_best_price {
                            current_best_price = player_price;
                        } else if !win && player_price > current_worst_price {
                            current_worst_price = player_price;
                        }
                    }
                }
            }
        }
    }
    if part == 1 {
        current_best_price
    } else {
        current_worst_price
    }
}

pub fn main() {
    let input = "Hit Points: 104 Damage: 8 Armor: 1";
    let shop_content = [
        vec![(8, 4, 0), (10, 5, 0), (25, 6, 0), (40, 7, 0), (74, 8, 0)],
        vec![(13, 0, 1), (31, 0, 2), (53, 0, 3), (75, 0, 4), (102, 0, 5)],
        vec![
            (25, 1, 0),
            (50, 2, 0),
            (100, 3, 0),
            (20, 0, 1),
            (40, 0, 2),
            (80, 0, 3),
        ],
    ];
    // PART 1
    let start = Instant::now();
    let known_answer = "78";
    let part_1: usize = get_optimal_gear_price(&input, &shop_content, 1);
    let duration = start.elapsed();
    print_part_1(&part_1.to_string(), &known_answer, duration);

    // PART 2
    let start = Instant::now();
    let known_answer = "148";
    let part_2: usize = get_optimal_gear_price(&input, &shop_content, 2);
    let duration = start.elapsed();
    print_part_2(&part_2.to_string(), &known_answer, duration);
}
