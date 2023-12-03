pub fn solution(input: String) -> u32 {
    input
        .lines()
        .map(|s| s.split(": ").nth(1).expect("failed to get game string").split("; "))
        .map(|rounds| {
            let cube_sets = rounds
                .map(|round| round.split(", ").map(str::to_owned).collect::<Vec<_>>())
                .flatten()
                .map(|cube_set| {
                    let mut cube_set_split = cube_set.split_ascii_whitespace();
                    let num_cubes: u32 = cube_set_split.next().expect("failed to get num string").parse().expect("failed to parse num cubes");
                    let cube_type = cube_set_split.next().expect("failed to get cube type string");

                    let cube_type = match cube_type {
                        "red" => CubeType::Red,
                        "green" => CubeType::Green,
                        "blue" => CubeType::Blue,
                        _ => panic!("What kind of cube even is that?!"),
                    };
                    (num_cubes, cube_type)
                });

            let red_max =
                get_max_by_color(cube_sets.clone(), CubeType::Red).expect("failed to get red max!");
            let green_max = get_max_by_color(cube_sets.clone(), CubeType::Green)
                .expect("failed to get green max!");
            let blue_max = get_max_by_color(cube_sets, CubeType::Blue)
                .expect("failed to get blue max!");

            red_max * green_max * blue_max
        })
        .sum()
}

pub fn get_max_by_color<I>(cube_sets: I, desired_cube_type: CubeType) -> Option<u32>
where
    I: Iterator<Item = (u32, CubeType)>,
{
    cube_sets
        .filter_map(|(num_cubes, cube_type)| {
            if cube_type == desired_cube_type {
                Some(num_cubes)
            } else {
                None
            }
        })
        .max()
}

#[derive(Eq, PartialEq)]
pub enum CubeType {
    Red,
    Green,
    Blue,
}
