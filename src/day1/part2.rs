const NUMBERS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];
const DIGIT_CHARS: [char; 9] = ['1', '2', '3', '4', '5', '6', '7', '8', '9'];

pub fn solution(input: String) -> u32 {
    input
        .lines()
        .map(|s| {
            let mut matches = Vec::new();
            for (i, number) in NUMBERS.iter().enumerate() {
                for (num_index, _) in s.match_indices(number) {
                    matches.push((num_index, DIGIT_CHARS[i]));
                }
            }

            let mut char_digits: Vec<_> = s
                .chars()
                .enumerate()
                .filter_map(|(i, x)| {
                    if x.is_ascii_digit() {
                        Some((i, x))
                    } else {
                        None
                    }
                })
                .collect();
            char_digits.append(&mut matches);

            char_digits.sort_by(|(x, _), (y, _)| x.cmp(&y));

            let (_, first) = char_digits.first().expect("there must be a first");
            let (_, last) = char_digits.last().expect("there must be a last");

            String::from_iter([first, last])
        })
        .map(|s| s.parse::<u32>().expect("these should all be digits!"))
        .sum()
}
