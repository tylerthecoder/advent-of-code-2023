use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufRead};
use std::path::Path;

pub fn run() {
    let path = Path::new("input.txt");

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", path.display(), why),
        Ok(file) => file,
    };

    let mut contents = String::new();

    match file.read_to_string(&mut contents) {
        Err(why) => panic!("couldn't read {}: {}", path.display(), why),
        Ok(_) => (),
    }

    println!("Part 1: {}", &contents);

    let letters = [
        ["one", "1"],
        ["two", "2"],
        ["three", "3"],
        ["four", "4"],
        ["five", "5"],
        ["six", "6"],
        ["seven", "7"],
        ["eight", "8"],
        ["nine", "9"],
    ];

    let mut total = 0;
    if let Ok(lines) = read_lines("input.txt") {
        for line in lines {
            if let Ok(line) = line {
                let mut num = 0;
                println!("{line}");

                let mut closest_index = 10000;
                let mut new_line = line.clone();
                for [letter, letter_num] in letters {
                    if let Some(index) = line.find(letter) {
                        if index < closest_index {
                            closest_index = index;
                            new_line = line.replace(letter, letter_num);
                        }
                    }
                }

                println!("{new_line}");

                for c in new_line.chars() {
                    if let Some(n) = c.to_digit(10) {
                        num = n * 10;
                        break;
                    }
                }

                let mut closest_index = 0;
                let mut new_line = line.clone();
                for [letter, letter_num] in letters {
                    if let Some(index) = line.rfind(letter) {
                        if index > closest_index {
                            closest_index = index;
                            new_line = line.replace(letter, letter_num);
                        }
                    }
                }

                println!("{new_line}");

                for c in new_line.chars().rev() {
                    if let Some(n) = c.to_digit(10) {
                        num += n;
                        break;
                    }
                }

                total += num;
                println!("Num: {num}");
            }
        }
    }

    println!("Total: {total}");
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
