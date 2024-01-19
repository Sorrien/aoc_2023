use aoc_2023::day7::part2::solution;
use std::fs;

fn main() {
    let input_string =
        fs::read_to_string("inputs/day7.txt").expect("failed to load input file");

    let sum = solution(input_string);

    println!("{}", sum);
}
