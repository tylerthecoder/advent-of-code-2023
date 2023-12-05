use crate::utils::read_lines;

pub struct Card {
    pub winning_nums: Vec<u32>,
    pub nums: Vec<u32>,
}

impl Card {
    pub fn parse_line(line: String) -> Option<Card> {
        println!("{}", line);
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

    pub fn get_point_value(&self) -> i32 {
        let winning_num_count = self
            .nums
            .iter()
            .filter(|num| self.winning_nums.contains(num))
            .count();

        if winning_num_count == 0 {
            return 0;
        }

        let points = i32::pow(2, (winning_num_count - 1) as u32);

        println!(
            "win_nums: {:?}, nums: {:?}, cnt: {:?}, points: {:}}}",
            self.winning_nums, self.nums, winning_num_count, points
        );

        points
    }
}

pub fn part1(cards: Vec<Card>) {
    let total_points: i32 = cards.iter().map(|card| card.get_point_value()).sum();
    println!("Part 1: {}", total_points);
}

pub fn main() {
    if let Ok(lines) = read_lines("src/day4/input.txt") {
        let cards = lines.map(|line| line.unwrap()).filter_map(Card::parse_line);

        part1(cards.collect());
    }
}
