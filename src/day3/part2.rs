pub fn solution(input: String) -> u32 {
    let lines = input.lines();

    let symbol_indices = lines
        .clone()
        .enumerate()
        .flat_map(|(i, s)| s.chars().enumerate().map(move |(j, c)| (i, j, c)))
        .filter_map(|(i, j, c)| if c == '*' { Some((i, j)) } else { None });

    let line_collection: Vec<Vec<char>> = lines.map(|s| s.chars().collect()).collect::<_>();

    let num_lines = line_collection.len();
    let line_length = line_collection[0].len();

    symbol_indices
        .filter_map(|(i, j)| {
            let search_locations = [
                (i, j - 1),
                (i, j + 1),
                (i - 1, j),
                (i + 1, j),
                (i - 1, j - 1),
                (i + 1, j + 1),
                (i - 1, j + 1),
                (i + 1, j - 1),
            ];

            let mut matches: Vec<_> = search_locations
                .iter()
                .filter(|(x, y)| x < &num_lines || y < &line_length)
                .filter_map(|(x, y)| {
                    let line = &line_collection[*x];

                    let search_c = line[*y];
                    if search_c.is_ascii_digit() {
                        let mut numbers = vec![];
                        for i in (0..*y).rev() {
                            let c = line[i];
                            if c.is_ascii_digit() {
                                numbers.push(c);
                            } else {
                                break;
                            }
                        }
                        numbers.reverse();

                        for i in *y..line_length {
                            let c = line[i];
                            if c.is_ascii_digit() {
                                numbers.push(c);
                            } else {
                                break;
                            }
                        }

                        if numbers.len() > 0 {
                            let test_string = numbers.iter().collect::<String>();
                            Some(test_string)
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                })
                .collect();

            matches.sort();
            matches.dedup();

            if matches.len() == 2 {
                let mut nums = matches
                    .iter()
                    .map(|s| s.parse::<u32>().expect("these should all be numbers!"));
                let part1 = nums.next().unwrap();
                let part2 = nums.next().unwrap();
                let gear_ratio = part1 * part2;
                println!("{}", gear_ratio);
                Some(gear_ratio)
            } else {
                None
            }
        })
        .sum::<u32>()
}
