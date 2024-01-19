use std::str::Lines;

use rayon::prelude::*;

pub fn solution(input: String) -> u64 {
    let mut lines = input.lines();

    let seeds: Vec<_> = lines.next().expect("there should be seeds!")[7..]
        .split_ascii_whitespace()
        .map(|s| s.parse::<u64>().expect("seeds should be ints"))
        .collect();

    let mut maps = Vec::new();
    for _ in 0..7 {
        maps.push(get_map(&mut lines, ":"));
    }

    seeds
        .par_iter()
        .map(|seed| {
            let mut category = *seed;
            for map in &maps {
                category = map.get_dest_cat(category);
            }
            category
        })
        .min()
        .unwrap()
}

fn get_map(lines: &mut Lines<'_>, map_name: &str) -> Map {
    lines.find(|s| s.contains(map_name));

    Map::new(
        lines
            .take_while(|s| !s.is_empty())
            .map(|s| Mapping::new(s))
            .collect(),
    )
}

pub struct Mapping {
    pub dest_range_start: u64,
    pub source_range_start: u64,
    pub range_length: u64,
}

impl Mapping {
    fn new(string: &str) -> Self {
        let mut numbers = string
            .split_ascii_whitespace()
            .map(|s| s.parse::<u64>().expect("mappings should be numbers"));

        Self {
            dest_range_start: numbers.next().expect("failed to get dest range"),
            source_range_start: numbers.next().expect("failed to get source range"),
            range_length: numbers.next().expect("failed to get range len"),
        }
    }
}

pub struct Map {
    pub mappings: Vec<Mapping>,
}

impl Map {
    fn new(mappings: Vec<Mapping>) -> Self {
        Self { mappings: mappings }
    }

    fn get_dest_cat(&self, source_cat: u64) -> u64 {
        let mut dest_cat = source_cat;
        for mapping in &self.mappings {
            if source_cat >= mapping.source_range_start
                && source_cat <= (mapping.source_range_start + mapping.range_length - 1)
            {
                dest_cat = mapping.dest_range_start + source_cat - mapping.source_range_start;
                break;
            }
        }

        dest_cat
    }
}
