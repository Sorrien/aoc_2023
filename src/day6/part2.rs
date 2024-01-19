pub fn solution(input: String) -> u64 {
    let mut lines = input.lines();
    let time = lines
        .next()
        .unwrap()
        .chars()
        .filter(|c| c.is_ascii_digit())
        .collect::<String>()
        .parse::<u64>()
        .unwrap();

    let distance = lines
        .next()
        .unwrap()
        .chars()
        .filter(|c| c.is_ascii_digit())
        .collect::<String>()
        .parse::<u64>()
        .unwrap();

    let mut ways_to_win = 0;
    for i in 1..time {
        if (time - i) * i > distance {
            ways_to_win += 1;
        }
    }

    ways_to_win
}
