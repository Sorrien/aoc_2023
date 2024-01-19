pub fn solution(input: String) -> u32 {
    let lines = input.lines();
    let mut card_clone_counts = vec![1; lines.clone().count()];

    lines
        .enumerate()
        .map(|(card_index, s)| {
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

            (card_index, num_matches)
        })
        .map(|(card_index, num_matches)| {
            for _ in 0..card_clone_counts[card_index] {
                for i in (card_index + 1)..=(card_index + num_matches as usize) {
                    card_clone_counts[i] += 1;
                }
            }
            card_clone_counts[card_index]
        })
        .sum()
}
