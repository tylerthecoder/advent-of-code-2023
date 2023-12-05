use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::FromStr;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Color {
    Red,
    Blue,
    Green,
}

impl FromStr for Color {
    type Err = ();

    fn from_str(input: &str) -> Result<Color, Self::Err> {
        match input {
            "red" => Ok(Color::Red),
            "blue" => Ok(Color::Blue),
            "green" => Ok(Color::Green),
            _ => Err(()),
        }
    }
}

struct ColorCount {
    amount: u32,
    color: Color,
}

struct Round {
    counts: Vec<ColorCount>,
}

impl Round {
    pub fn get_count_for_color(&self, color: Color) -> u32 {
        self.counts
            .iter()
            .find(|count| count.color == color)
            .map(|count| count.amount)
            .unwrap_or(0)
    }
}

struct Game {
    id: String,
    rounds: Vec<Round>,
}

impl Game {
    pub fn get_largest_count_for_color(&self, color: Color) -> u32 {
        self.rounds
            .iter()
            .map(|round| round.get_count_for_color(color))
            .max()
            .unwrap_or(0)
    }
}

impl Game {
    fn parse_line(line: String) -> Option<Game> {
        let re = Regex::new(r"Game (\d+): (.*)").unwrap();
        let color_re = Regex::new(r"(\d+) (\w+)").unwrap();

        let caps = re.captures(&line)?;
        let id = caps.get(1)?.as_str().to_string();
        let rounds_data = caps.get(2)?.as_str();

        let rounds: Vec<Round> = rounds_data
            .split(';')
            .map(|round_data| {
                let counts = color_re
                    .captures_iter(round_data)
                    .filter_map(|cap| {
                        let amount = cap[1].parse().ok()?;
                        let color = Color::from_str(&cap[2]).ok()?;

                        Some(ColorCount { amount, color })
                    })
                    .collect();

                Round { counts }
            })
            .collect();

        Some(Game { id, rounds })
    }
}

pub fn part1() {
    fn is_valid_game(game: &Game) -> bool {
        let red_limit = 12;
        let green_limit = 13;
        let blue_limit = 14;
        for round in game.rounds.iter() {
            let red_count = round.get_count_for_color(Color::Red);
            let green_count = round.get_count_for_color(Color::Green);
            let blue_count = round.get_count_for_color(Color::Blue);

            if red_count > red_limit || green_count > green_limit || blue_count > blue_limit {
                return false;
            }
        }
        return true;
    }

    if let Ok(lines) = read_lines("src/day2/input.txt") {
        let games = lines
            .filter_map(|line| line.ok())
            .filter_map(Game::parse_line)
            .filter(is_valid_game);

        let id_sum = games
            .map(|game| game.id.parse::<u32>().unwrap())
            .sum::<u32>();

        println!("Sum of valid game IDs: {}", id_sum);
    } else {
        println!("Could not read input file");
    }
}

pub fn part2() {
    fn smallest_set(game: Game) -> u32 {
        let smallest_red = game.get_largest_count_for_color(Color::Red);
        let smallest_green = game.get_largest_count_for_color(Color::Green);
        let smallest_blue = game.get_largest_count_for_color(Color::Blue);

        return smallest_red * smallest_blue * smallest_green;
    }

    if let Ok(lines) = read_lines("src/day2/input.txt") {
        let sum = lines
            .filter_map(|line| line.ok())
            .filter_map(Game::parse_line)
            .map(smallest_set)
            .sum::<u32>();

        println!("Answer: {}", sum);
    } else {
        println!("Could not read input file");
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
