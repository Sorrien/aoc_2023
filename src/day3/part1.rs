pub fn solution(input: String) -> u32 {
    let lines = input.lines();

    let symbol_indices = lines
        .clone()
        .enumerate()
        .flat_map(|(i, s)| s.chars().enumerate().map(move |(j, c)| (i, j, c)))
        .filter_map(|(i, j, c)| {
            if !c.is_ascii_digit() && c != '.' {
                Some((i, j))
            } else {
                None
            }
        });

    let line_collection: Vec<Vec<char>> = lines.map(|s| s.chars().collect()).collect::<_>();

    let num_lines = line_collection.len();
    let line_length = line_collection[0].len();

    for (i, j) in symbol_indices {

        let search_locations = [(i,j-1), (i, j+1), (i-1,j), (i+1, j), (i-1, j-1), (i+1, j+1), (i-1, j+1), (i+1, j-1)];

        for (x,y) in search_locations {
            if x < num_lines || y < line_length {
                let c = line_collection[x][y];
                if c.is_ascii_digit() {
                    
                }
            }
        }
        println!("{}", line_collection[i][j]);
    }
    1
}
