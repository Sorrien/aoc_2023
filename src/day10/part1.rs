use core::fmt::Debug;

const NORTH: (i8, i8) = (-1, 0);
const SOUTH: (i8, i8) = (1, 0);
const EAST: (i8, i8) = (0, 1);
const WEST: (i8, i8) = (0, -1);

pub fn solution(input: String) -> u64 {
    let lines = input.lines();

    let line_collection: Vec<Vec<char>> = lines.map(|s| s.chars().collect()).collect::<_>();

    let (start_x, start_y) = find_start_index(&line_collection);

    //println!("start xy: {} {}", start_x, start_y);

    let cardinals = [NORTH, SOUTH, EAST, WEST];

    let starting_adj_tiles = cardinals
        .iter()
        .filter_map(|(x, y)| {
            get_adj_tile(&line_collection, &start_x, &start_y, (x, y))
        })
        .collect::<Vec<_>>();

    println!("{:?}", starting_adj_tiles);

    let starting_pipes = starting_adj_tiles
        .iter()
        .filter(|(c, (_, _))| *c != '.')
        .filter_map(|(tile, (tile_x, tile_y))| {
            let pipe_dirs = get_pipe_dirs_enum(&tile);

            if let Some((_, dir)) = pipe_dirs
                .iter()
                .filter_map(|pipe_dir| {
                    let (pipe_dir_x, pipe_dir_y) = pipe_dir.get_dir();
                    if let Some(x) = get_adj_tile(
                        &line_collection,
                        &tile_x,
                        &tile_y,
                        (&pipe_dir_x, &pipe_dir_y),
                    ) {
                        Some((x, pipe_dir))
                    } else {
                        None
                    }
                })
                .find(|((c, _), _)| *c == 'S')
            {
                Some((*tile, (*tile_x, *tile_y), pipe_dirs, dir.get_opposite()))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    println!("starting pipes: {:?}", starting_pipes);

    let first_pipe = starting_pipes.first().unwrap();

    let mut pipe_loop = Vec::new();

    let mut is_complete = false;

    let mut current_pipe = *first_pipe;
    pipe_loop.push(current_pipe);

    while !is_complete {
        println!("current pipe: {:?}", current_pipe);
        let (_, (x, y), pipe_dirs, last_direction) = current_pipe;

        for pipe_dir in pipe_dirs {
            println!("{:?}", pipe_dir);
            if pipe_dir != last_direction.get_opposite() {
                let (pipe_dir_x, pipe_dir_y) = pipe_dir.get_dir();
                let (adj_pipe_char, (adj_pipe_x, adj_pipe_y)) =
                    get_adj_tile(&line_collection, &x, &y, (&pipe_dir_x, &pipe_dir_y)).unwrap();

                if adj_pipe_char == 'S' {
                    is_complete = true;
                    break;
                }
                let adj_pipe_dirs: [Direction; 2] = get_pipe_dirs_enum(&adj_pipe_char);

                let next_pipe = (
                    adj_pipe_char,
                    (adj_pipe_x, adj_pipe_y),
                    adj_pipe_dirs,
                    pipe_dir,
                );

                pipe_loop.push(next_pipe);
                current_pipe = next_pipe;
                //for adj_pipe_dir in adj_pipe_dirs {}
            }
        }
    }

    println!("{:?}", pipe_loop);

    for (pipe_char, (pipe_x, pipe_y), pipe_dirs, last_pipe_dir) in pipe_loop.clone() {
        println!("{}", pipe_char)
    }

    let loop_length = pipe_loop.len();
    let (first_half, second_half) = pipe_loop.split_at((loop_length / 2) + 1 as usize);

    println!("{:?}", first_half);
    println!("{:?}", second_half);

    first_half.iter().enumerate().last().unwrap().0 as u64 + 1
}

fn get_pipe_dirs_enum(pipe: &char) -> [Direction; 2] {
    match pipe {
        '|' => [Direction::North, Direction::South],
        '-' => [Direction::East, Direction::West],
        'L' => [Direction::North, Direction::East],
        'J' => [Direction::North, Direction::West],
        '7' => [Direction::South, Direction::West],
        'F' => [Direction::South, Direction::East],
        _ => panic!("at the disco!"),
    }
}

fn find_start_index(line_collection: &Vec<Vec<char>>) -> (usize, usize) {
    for (x, line) in line_collection.iter().enumerate() {
        for (y, c) in line.iter().enumerate() {
            if *c == 'S' {
                //println!("{}", c);
                return (x, y);
            }
        }
    }
    panic!("at the disco!");
}

fn get_adj_tile(
    collection: &Vec<Vec<char>>,
    start_x: &usize,
    start_y: &usize,
    (x, y): (&i8, &i8),
) -> Option<(char, (usize, usize))> {
    let tile_x = *start_x as i64 + *x as i64;
    let tile_y = *start_y as i64 + *y as i64;

    /*let start_tile = collection[*start_x][*start_y];

    println!(
        "start tile: {} adj tile xy: {} {}",
        start_tile, tile_x, tile_y
    );*/

    if tile_x >= 0
        && tile_y >= 0
        && tile_x < collection.len() as i64
        && tile_y < collection[tile_x as usize].len() as i64
    {
        let tile_x = tile_x as usize;
        let tile_y = tile_y as usize;
        //println!("tile xy: {} {}", tile_x, tile_y);
        let tile = collection[tile_x][tile_y];
        //println!("adj tile: {} adj tile xy: {} {}", tile, tile_x, tile_y);
        Some((tile, (tile_x, tile_y)))
    } else {
        None
    }
}

#[derive(PartialEq, Copy, Clone)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn get_dir(&self) -> (i8, i8) {
        match self {
            Direction::North => NORTH,
            Direction::South => SOUTH,
            Direction::East => EAST,
            Direction::West => WEST,
        }
    }

    fn get_opposite(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        }
    }
}

impl Debug for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::North => write!(f, "North {:?}", self.get_dir()),
            Self::South => write!(f, "South {:?}", self.get_dir()),
            Self::East => write!(f, "East {:?}", self.get_dir()),
            Self::West => write!(f, "West {:?}", self.get_dir()),
        }
    }
}
