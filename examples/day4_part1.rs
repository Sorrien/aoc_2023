use aoc_2023::day4::part1::solution;
use std::fs;

fn main() {
    let input_string =
        fs::read_to_string("inputs/day4.txt").expect("failed to load input file");

    let sum = solution(input_string);

    println!("{}", sum);
}
