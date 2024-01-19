pub fn solution(input: String) -> i64 {
    let lines = input.lines();

    lines
        .map(|s| s.split_ascii_whitespace())
        .map(|split| split.map(|s| s.parse::<i64>().unwrap()))
        .map(|vals| get_next_value(&(vals.collect::<Vec<_>>())))
        .sum()
}

pub fn get_next_value(vals: &Vec<i64>) -> i64 {
    let diffs = (0..vals.len() - 1)
        .map(|i| {
            let first = vals[i];
            let second = vals[i + 1];
            second - first
        })
        .collect::<Vec<_>>();

    let next = if diffs.iter().all(|x| *x == 0) {
        *vals.first().unwrap()
    } else {
        let next_diff = get_next_value(&diffs);
        vals.first().unwrap() - next_diff
    };
    next
}
