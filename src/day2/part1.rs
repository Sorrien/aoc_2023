const RED_CUBE_TOTAL: u32 = 12;
const GREEN_CUBE_TOTAL: u32 = 13;
const BLUE_CUBE_TOTAL: u32 = 14;

pub fn solution(input: String) -> u32 {

    input.lines()
        .map(|s| {
            let mut split = s.split(": ");
            
            let first = split.next();
            let mut first_split = first.expect("failed to find first half of game string").split_ascii_whitespace();
            let id = first_split.nth(1).expect("failed to find game id string").parse::<u32>().expect("failed to parse game id");

            let second = split.next();
            let rounds = second.expect("failed to find second half of game string").split("; ");

            (id, rounds)
        })
        .filter_map(|(id, rounds)| {
            let is_game_valid = rounds
                .map(|round| round.split(", ").map(str::to_owned).collect::<Vec<_>>())
                .flatten()
                .map(|cube_set| {
                    let mut cube_set_split = cube_set.split_ascii_whitespace();
                    let num_cubes: u32 = cube_set_split.next().expect("failed to get num string").parse().expect("failed to parse num cubes");
                    let cube_type = cube_set_split.next().expect("failed to get cube type string");

                    let is_cubeset_valid = match cube_type {
                        "red" => num_cubes <= RED_CUBE_TOTAL,
                        "green" => num_cubes <= GREEN_CUBE_TOTAL,
                        "blue" => num_cubes <= BLUE_CUBE_TOTAL,
                        _ => panic!("What kind of cube is this anyway?!"),
                    };

                    is_cubeset_valid
                })
                .all(|x| x);

            if is_game_valid {
                Some(id)
            } else {
                None
            }
        })
        .sum()
}