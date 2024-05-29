#[inline]
fn in_grid(grid: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    return x < grid.len() && y < grid[0].len();
}

fn dfs(grid: &mut Vec<Vec<char>>, x: usize, y: usize) -> bool {
    if !in_grid(grid, x, y) {
        return false;
    }

    if grid[x][y] != '1' {
        return false;
    }

    grid[x][y] = '2';

    if x > 0 {
        dfs(grid, x - 1, y);
    }

    if y > 0 {
        dfs(grid, x, y - 1);
    }

    dfs(grid, x, y + 1);
    dfs(grid, x + 1, y);
    return true;
}

pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
    let mut island_number = 0;
    for x in 0..grid.len() {
        for y in 0..grid[0].len() {
            if grid[x][y] == '1' {
                if dfs(&mut grid, x, y) {
                    island_number += 1;
                }
            }
        }
    }
    island_number
}

fn main() {
    let mut arr = vec![
        vec!['1', '1', '1', '1', '0'],
        vec! ['1', '1', '0', '1', '0'],
        vec! ['1', '1', '0', '0', '0'],
        vec!['0', '0', '0', '0', '0'],
    ];
    let numbner = num_islands(arr);
    println!("{numbner}");
}
