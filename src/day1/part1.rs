pub fn solution(input: String) -> u32 {
    let digits: Vec<_> = input
        .lines()
        .map(|s| {
            let chars: Vec<_> = s.chars().filter(|x| x.is_ascii_digit()).collect();
            String::from_iter([
                chars.first().expect("there must be a first"),
                chars.last().expect("there must be a last"),
            ])
        })
        .map(|s| s.parse::<u32>().expect("these should all be digits!"))
        .collect();

    let sum: u32 = digits.iter().sum();
    sum
}
