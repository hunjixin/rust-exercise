#[inline]
fn in_grid<const M: usize, const N: usize>(_: &[[char; N]; M], x: usize, y: usize) -> bool {
    return x < M && y < N;
}

fn dfs<const M: usize, const N: usize>(grid: &mut [[char; N]; M], x: usize, y: usize) -> bool {
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

fn get_num_islands<const M: usize, const N: usize>(grid: &mut [[char; N]; M]) -> i32 {
    let mut island_number = 0;
    for x in 0..M {
        for y in 0..N {
            if grid[x][y] == '1' {
                if dfs(grid, x, y) {
                    island_number += 1;
                }
            }
        }
    }
    island_number
}

fn main() {
    let mut arr = [
        ['1', '1', '1', '1', '0'],
        ['1', '1', '0', '1', '0'],
        ['1', '1', '0', '0', '0'],
        ['0', '0', '0', '0', '0'],
    ];
    let numbner = get_num_islands(&mut arr);
    println!("{numbner}");
}
