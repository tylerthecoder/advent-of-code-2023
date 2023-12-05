use crate::utils::read_full_file;
use std::{collections::HashSet, fmt::Display, sync::atomic::AtomicUsize};

#[derive(Debug, PartialEq, Clone, Copy, Eq, PartialOrd, Ord)]
enum GridElement {
    Empty,
    Num(u32),
    Part(char),
}

impl Display for GridElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GridElement::Empty => write!(f, "."),
            GridElement::Num(n) => write!(f, "{}", n),
            GridElement::Part(c) => write!(f, "{}", c),
        }
    }
}

#[derive(Clone)]
struct Engine {
    grid: Vec<Vec<GridElement>>,
}

#[derive(Clone, PartialEq, PartialOrd, Eq, Hash, Debug)]
struct PartNum {
    id: usize,
    num: u32,
}

struct PartNums {
    grid: Vec<Vec<Option<PartNum>>>,
    pub nums: Vec<u32>,
}

static COUNTER: AtomicUsize = AtomicUsize::new(1);

impl PartNums {
    pub fn new(x: usize, y: usize) -> PartNums {
        PartNums {
            grid: vec![vec![None; x]; y],
            nums: vec![],
        }
    }

    pub fn add_part_num(&mut self, x_start: usize, x_end: usize, y: usize, num: u32) {
        let part_id = COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        for x in x_start..x_end {
            self.grid[y][x] = Some(PartNum { id: part_id, num });
        }

        self.nums.push(num);
    }

    pub fn get_part_num(&self, x: usize, y: usize) -> Option<PartNum> {
        self.grid.get(y)?.get(x)?.clone()
    }
}

struct Position {
    x: usize,
    y: usize,
}

struct GridElementPos {
    ele: GridElement,
    pos: Position,
}

impl Engine {
    fn new(x: usize, y: usize) -> Engine {
        Engine {
            grid: vec![vec![GridElement::Empty; x]; y],
        }
    }

    fn set_grid_element(&mut self, x: usize, y: usize, el: GridElement) {
        self.grid[y][x] = el;
    }

    pub fn get_grid_element(&self, x: usize, y: usize) -> Option<&GridElement> {
        self.grid.get(y)?.get(x)
    }

    pub fn get_grid_elements_around(&self, x: usize, y: usize) -> Vec<&GridElement> {
        let mut eles = vec![];
        fn add(u: usize, i: i32) -> usize {
            if i.is_negative() {
                if u == 0 {
                    return 0;
                }
                u - i.wrapping_abs() as usize
            } else {
                u + i as usize
            }
        }
        for i in -1 as i32..=1 {
            for j in -1 as i32..=1 {
                let x_n = add(x, i);
                let y_n = add(y, j);
                if let Some(ele) = self.get_grid_element(x_n, y_n) {
                    eles.push(ele);
                }
            }
        }
        eles
    }

    fn parse(str: String) -> Engine {
        let lines = str.split('\n');

        let mut eng = Engine::new(1000, 1000);

        for (y, line) in lines.into_iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                match c {
                    '.' => eng.set_grid_element(x, y, GridElement::Empty),
                    _ => match c.to_digit(10) {
                        Some(digit) => eng.set_grid_element(x, y, GridElement::Num(digit)),
                        None => eng.set_grid_element(x, y, GridElement::Part(c)),
                    },
                }
            }
        }

        eng
    }

    fn find_part_nums(self) -> PartNums {
        let mut part_nums = PartNums::new(1000, 1000);

        self.grid
            .clone()
            .into_iter()
            .enumerate()
            .for_each(|(y, row)| {
                // Find all consecutive numbers
                let mut current_num = 0;
                let mut found_part = false;
                let mut reading_num = false;
                let mut start_x = 0;
                for (x, ele) in row.into_iter().enumerate() {
                    match ele {
                        GridElement::Num(n) => {
                            if !reading_num {
                                start_x = x;
                                reading_num = true;
                            }
                            current_num = current_num * 10 + n;
                            let part_nearby = self.get_grid_elements_around(x, y).into_iter().any(
                                |ele| match ele {
                                    GridElement::Part(_) => true,
                                    _ => false,
                                },
                            );
                            if part_nearby {
                                found_part = part_nearby
                            }
                        }
                        _ => {
                            if found_part {
                                println!("Found part num: {current_num}");
                                part_nums.add_part_num(start_x, x, y, current_num);
                            }
                            start_x = 0;
                            found_part = false;
                            reading_num = false;
                            current_num = 0;
                        }
                    }
                }
            });

        part_nums
    }
}

impl IntoIterator for Engine {
    type Item = GridElementPos;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.grid
            .into_iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.into_iter()
                    .enumerate()
                    .map(move |(x, ele)| GridElementPos {
                        ele,
                        pos: Position { x, y },
                    })
            })
            .collect::<Vec<_>>()
            .into_iter()
    }
}

fn part1(eng: Engine) {
    let answer: u32 = eng.find_part_nums().nums.into_iter().sum();

    println!("Part1 answer: {answer}");
}

fn part2(eng: Engine) {
    let parts = eng.clone().find_part_nums();

    let mut total_gear_ratio = 0;

    eng.into_iter().for_each(|ele_pos| {
        if ele_pos.ele == GridElement::Part('*') {
            println!(
                "\nFound potential gear {:?} {:?}",
                ele_pos.pos.x, ele_pos.pos.y
            );
            let mut adjacent_part_nums: HashSet<PartNum> = HashSet::new();

            fn add(u: usize, i: i32) -> usize {
                if i.is_negative() {
                    if u == 0 {
                        return 0;
                    }
                    u - i.wrapping_abs() as usize
                } else {
                    u + i as usize
                }
            }
            for i in -1 as i32..=1 {
                for j in -1 as i32..=1 {
                    let x_n = add(ele_pos.pos.x, i);
                    let y_n = add(ele_pos.pos.y, j);
                    if let Some(part) = parts.get_part_num(x_n, y_n) {
                        adjacent_part_nums.insert(part.clone());
                        println!(
                            "Found adjacent part num ({:?}) at pos {x_n} {y_n}: {:?}",
                            part.num,
                            adjacent_part_nums.len()
                        );
                        // print the elements in the set
                        for p in adjacent_part_nums.iter() {
                            println!("Part num: {:?}", p);
                        }
                    }
                }
            }

            let mut part_itter = adjacent_part_nums.into_iter();

            let part_num1 = part_itter.next();
            let part_num2 = part_itter.next();

            if let Some(num1) = part_num1 {
                if let Some(num2) = part_num2 {
                    if part_itter.count() == 0 {
                        let gear_ratio = num1.num * num2.num;
                        println!("Found gear {gear_ratio}");
                        total_gear_ratio += gear_ratio;
                    }
                }
            }
        }
    });

    println!("Part 2 answer: {total_gear_ratio}");
}

pub fn main() {
    let puzzle_input = read_full_file("src/day3/input.txt");

    if let Ok(puzzle_input) = puzzle_input {
        let engine = Engine::parse(puzzle_input);
        println!("Input parsed");
        // part1(engine);
        part2(engine);
    }
}
