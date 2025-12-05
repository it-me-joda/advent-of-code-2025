const NEIGHBORS: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

pub fn solution(file: String) -> (u64, u64) {
    let grid = parse_grid(&file);

    let mut valid_positions = 0;

    let grid_len = grid.len();
    let mut x = 0;
    while x < grid_len {
        let row = &grid[x];
        let row_len = row.len();
        let mut y = 0;
        while y < row_len {
            if grid[x][y] == 1 {
                let neighbor_count = check_neighbors(&grid, x, y);
                if neighbor_count < 4 {
                    valid_positions += 1;
                }
            }

            y += 1;
        }

        x += 1;
    }

    (valid_positions, 0)
}

fn parse_grid(file: &str) -> Vec<Vec<u8>> {
    let mut grid: Vec<Vec<u8>> = Vec::new();
    for line in file.lines() {
        let row = parse_line(line);
        grid.push(row);
    }
    grid
}

fn parse_line(line: &str) -> Vec<u8> {
    line.chars().map(|c| if c == '@' { 1 } else { 0 }).collect()
}

fn check_neighbors(grid: &[Vec<u8>], x: usize, y: usize) -> u8 {
    let mut count = 0;
    let grid_len = grid.len() as isize;
    let row_len = grid[0].len() as isize;

    for neighbor in NEIGHBORS.iter() {
        let nx = x as isize + neighbor.0;
        let ny = y as isize + neighbor.1;
        if nx >= 0 && nx < grid_len && ny >= 0 && ny < row_len {
            count += grid[nx as usize][ny as usize];
        }
    }

    count
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_parse_line() {
        assert_eq!(super::parse_line("@..@."), vec![1, 0, 0, 1, 0]);
    }

    #[test]
    fn test_parse_grid() {
        let file = "@..@.\n.....\n.@...\n";
        let grid = super::parse_grid(file);
        assert_eq!(grid.len(), 3);
        assert_eq!(grid[0], vec![1, 0, 0, 1, 0]);
        assert_eq!(grid[1], vec![0, 0, 0, 0, 0]);
        assert_eq!(grid[2], vec![0, 1, 0, 0, 0]);
    }
}
