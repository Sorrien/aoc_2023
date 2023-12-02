pub mod day1;
pub mod day2;

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::day1;
    use crate::day2;

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
}