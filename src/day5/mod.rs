use regex::Regex;

use crate::utils::read_full_file;

struct NumMap {
    source: u64,
    dest: u64,
    range: u64,
}

impl NumMap {
    pub fn get_mapped_num(&self, num: u64) -> Option<u64> {
        if num >= self.source && num < self.source + self.range {
            let offset = num - self.source;
            Some(self.dest + offset)
        } else {
            None
        }
    }
}

struct GardenMap {
    from_type: String,
    to_type: String,
    maps: Vec<NumMap>,
}

impl GardenMap {
    fn get_mapped_num(&self, num: u64) -> u64 {
        let mut mapped_num = self
            .maps
            .iter()
            .filter_map(|num_map| num_map.get_mapped_num(num));

        match mapped_num.next() {
            Some(val) => val,
            None => num,
        }
    }
}

pub struct Garden {
    seeds: Vec<u64>,
    maps: Vec<GardenMap>,
}

impl Garden {
    pub fn parse(str: String) -> Option<Garden> {
        let mut lines = str.split('\n');

        let seed_line = lines.next()?;

        // parse seed line
        // seeds: 10 20 28 47
        let seeds: Vec<u64> = seed_line
            .split(' ')
            .filter_map(|s| s.parse().ok())
            .collect();

        let mut maps: Vec<GardenMap> = vec![];

        while lines.clone().count() > 0 {
            let mut line = lines.next()?;

            if line == "" {
                continue;
            }

            // each line looks like "seed-to-soil map:" or "soil-to-water map:"
            // need to parse out the words "seed" and "soil"
            let re = Regex::new(r"(\w+)-to-(\w+) map:").ok()?;

            let from = re.captures(&line)?.get(1)?.as_str().to_string();
            let to = re.captures(&line)?.get(2)?.as_str().to_string();

            line = lines.next()?;

            let mut num_maps: Vec<NumMap> = vec![];

            while line.len() > 0 {
                let re = Regex::new(r"(\d+) (\d+) (\d+)").ok()?;
                let cs = re.captures(&line)?;
                let mut caps = cs.iter().filter_map(|x| x?.as_str().parse::<u64>().ok());

                let dest = caps.next()?;
                let source = caps.next()?;
                let range = caps.next()?;
                let map = NumMap {
                    dest,
                    source,
                    range,
                };
                num_maps.push(map);
                line = lines.next()?;
            }

            let garden_map = GardenMap {
                maps: num_maps,
                to_type: to,
                from_type: from,
            };

            maps.push(garden_map);
        }

        Some(Garden { seeds, maps })
    }

    fn get_map_with_from_type(&self, from_type: String) -> Option<&GardenMap> {
        self.maps.iter().find(|map| map.from_type == from_type)
    }

    pub fn find_smallest_location_numer(&self) -> u64 {
        self.seeds
            .clone()
            .into_iter()
            .map(|seed_num| {
                let starting_type: String = "seed".to_string();
                let mut ending_seed_num = seed_num;

                let mut current_garden_map = self.get_map_with_from_type(starting_type);

                loop {
                    match current_garden_map {
                        Some(gm) => {
                            ending_seed_num = gm.get_mapped_num(ending_seed_num);
                            current_garden_map = self.get_map_with_from_type(gm.to_type.to_owned());
                        }
                        None => return ending_seed_num,
                    }
                }
            })
            .min()
            .unwrap_or(0)
    }
}

pub fn part1(garden: Garden) {
    println!("Part 1: {:?}", garden.find_smallest_location_numer());
}

pub fn main() {
    if let Some(file) = read_full_file("./src/day5/input.txt").ok() {
        let garden = Garden::parse(file);

        println!("The garden is parsed");

        if let Some(garden) = garden {
            part1(garden)
        }
    }
}
