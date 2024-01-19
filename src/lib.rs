#![feature(iter_array_chunks)]

pub mod day1;
pub mod day10;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day9;

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::day1;
    use crate::day10;
    use crate::day2;
    use crate::day3;
    use crate::day4;
    use crate::day5;
    use crate::day6;
    use crate::day7;
    use crate::day9;

    #[test]
    fn day1_part1() {
        let input_string =
            fs::read_to_string("inputs/day1.txt").expect("failed to load input file");

        let result = day1::part1::solution(input_string);
        assert_eq!(result, 54159);
    }

    #[test]
    fn day1_part2() {
        let input_string =
            fs::read_to_string("inputs/day1.txt").expect("failed to load input file");

        let result = day1::part2::solution(input_string);
        assert_eq!(result, 53866);
    }

    #[test]
    fn day2_part1() {
        let input_string =
            fs::read_to_string("inputs/day2.txt").expect("failed to load input file");

        let result = day2::part1::solution(input_string);
        assert_eq!(result, 2716);
    }

    #[test]
    fn day2_part2() {
        let input_string =
            fs::read_to_string("inputs/day2.txt").expect("failed to load input file");

        let result = day2::part2::solution(input_string);
        assert_eq!(result, 72227);
    }

    #[test]
    fn day3_part1() {
        let input_string =
            fs::read_to_string("inputs/day3.txt").expect("failed to load input file");

        let result = day3::part1::solution(input_string);
        assert_eq!(result, 526404);
    }

    #[test]
    fn day3_part2() {
        let input_string =
            fs::read_to_string("inputs/day3.txt").expect("failed to load input file");

        let result = day3::part2::solution(input_string);
        assert_eq!(result, 84399773);
    }

    #[test]
    fn day4_part1() {
        let input_string =
            fs::read_to_string("inputs/day4.txt").expect("failed to load input file");

        let result = day4::part1::solution(input_string);
        assert_eq!(result, 19135);
    }

    #[test]
    fn day4_part2() {
        let input_string =
            fs::read_to_string("inputs/day4.txt").expect("failed to load input file");

        let result = day4::part2::solution(input_string);
        assert_eq!(result, 5704953);
    }

    #[test]
    fn day5_part1() {
        let input_string =
            fs::read_to_string("inputs/day5.txt").expect("failed to load input file");

        let result = day5::part1::solution(input_string);
        assert_eq!(result, 403695602);
    }

    /*
    for the moment this solution takes too long to run as a unit test regularly
    #[test]
        fn day5_part2() {
            let input_string =
                fs::read_to_string("inputs/day5.txt").expect("failed to load input file");

            let result = day5::part2::solution(input_string);
            assert_eq!(result, 219529182);
        } */

    #[test]
    fn day6_part1() {
        let input_string =
            fs::read_to_string("inputs/day6.txt").expect("failed to load input file");

        let result = day6::part1::solution(input_string);
        assert_eq!(result, 1083852);
    }

    #[test]
    fn day6_part2() {
        let input_string =
            fs::read_to_string("inputs/day6.txt").expect("failed to load input file");

        let result = day6::part2::solution(input_string);
        assert_eq!(result, 23501589);
    }

    #[test]
    fn day7_part1() {
        let input_string =
            fs::read_to_string("inputs/day7.txt").expect("failed to load input file");

        let result = day7::part1::solution(input_string);
        assert_eq!(result, 248836197);
    }

    #[test]
    fn day7_part2() {
        let input_string =
            fs::read_to_string("inputs/day7.txt").expect("failed to load input file");

        let result = day7::part2::solution(input_string);
        assert_eq!(result, 251195607);
    }

    #[test]
    fn day9_part1() {
        let input_string =
            fs::read_to_string("inputs/day9.txt").expect("failed to load input file");

        let result = day9::part1::solution(input_string);
        assert_eq!(result, 2174807968);
    }

    #[test]
    fn day9_part2() {
        let input_string =
            fs::read_to_string("inputs/day9.txt").expect("failed to load input file");

        let result = day9::part2::solution(input_string);
        assert_eq!(result, 1208);
    }

    #[test]
    fn day10_part1() {
        let input_string =
            fs::read_to_string("inputs/day10.txt").expect("failed to load input file");

        let result = day10::part1::solution(input_string);
        assert_eq!(result, 6733);
    }

    #[test]
    fn day10_part2() {
        let input_string =
            fs::read_to_string("inputs/day10_ex.txt").expect("failed to load input file");

        let result = day10::part2::solution(input_string);
        assert_eq!(result, 10);
    }
}
