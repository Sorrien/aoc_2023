pub fn solution(input: String) -> u64 {
    let mut lines = input.lines();

    let times = lines
        .next()
        .unwrap()
        .split_whitespace()
        .filter_map(|s| s.parse::<u32>().ok());
    let distances = lines
        .next()
        .unwrap()
        .split_whitespace()
        .filter_map(|s| s.parse::<u32>().ok());

    let mut race_results = times.zip(distances).map(|(t, d)| {
        let mut ways_to_win = 0;
        for i in 1..t {
            if (t - i) * i > d {
                ways_to_win += 1;
            }
        }

        ways_to_win
    });

    let mut result = race_results.next().unwrap();

    for race_result in race_results {
        result *= race_result;
    }
    result
}
