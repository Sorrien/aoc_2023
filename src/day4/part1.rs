pub fn solution(input: String) -> u32 {
    input
        .lines()
        .map(|s| {
            let mut card_split = s.split(": ");
            let numbers_section = card_split.nth(1).expect("failed to get number section");

            let mut section_split = numbers_section.split(" | ");
            let winning_section = section_split.next().unwrap();
            let num_section = section_split.next().unwrap();

            let winning_nums = winning_section.split_ascii_whitespace();
            let numbers = num_section.split_ascii_whitespace();

            let num_matches: u32 = winning_nums
                .map(move |s| {
                    numbers
                        .clone()
                        .filter(|n| n.eq_ignore_ascii_case(s))
                        .count() as u32
                })
                .sum();

            if num_matches > 1 {
                (2 as u32).pow(num_matches - 1)
            } else {
                num_matches
            }
        })
        .sum::<u32>()
}
