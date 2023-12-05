use std::collections::HashMap;

use crate::utils::read_lines;

pub struct Card {
    pub winning_nums: Vec<u32>,
    pub nums: Vec<u32>,
}

impl Card {
    pub fn parse_line(line: String) -> Option<Card> {
        let mut parts = line.split(":").nth(1)?.split("|");

        let winning_nums: Vec<u32> = parts
            .next()?
            .split(" ")
            .filter_map(|s| s.parse().ok())
            .collect();

        let other_nums: Vec<u32> = parts
            .next()?
            .split(" ")
            .filter_map(|s| s.parse().ok())
            .collect();

        Some(Card {
            winning_nums,
            nums: other_nums,
        })
    }

    pub fn get_winning_matches(&self) -> usize {
        self.nums
            .iter()
            .filter(|num| self.winning_nums.contains(num))
            .count()
    }

    pub fn get_point_value(&self) -> i32 {
        let winning_num_count = self.get_winning_matches();

        if winning_num_count == 0 {
            return 0;
        }

        let points = i32::pow(2, (winning_num_count - 1) as u32);

        points
    }
}

pub fn part1(cards: Vec<Card>) {
    let total_points: i32 = cards.iter().map(|card| card.get_point_value()).sum();
    println!("Part 1: {}", total_points);
}

pub fn part2(cards: Vec<Card>) {
    // maps card number to its count
    let mut card_count: HashMap<usize, usize> = HashMap::new();

    let mut total_card_count = 0;

    for (i, card) in cards.into_iter().enumerate() {
        let winning_num_count = card.get_winning_matches();
        let checking_card_count = match card_count.get(&i) {
            None => 1,
            Some(val) => *val,
        };

        total_card_count += checking_card_count;

        for j in (i + 1)..=i + winning_num_count {
            let new_card_count = match card_count.get(&j) {
                None => checking_card_count + 1,
                Some(val) => val + checking_card_count,
            };

            card_count.insert(j, new_card_count);
        }
    }

    println!("Part 2: {}", total_card_count);
}

pub fn main() {
    if let Ok(lines) = read_lines("src/day4/input.txt") {
        let cards = lines.map(|line| line.unwrap()).filter_map(Card::parse_line);

        // part1(cards.collect());
        part2(cards.collect());
    }
}
