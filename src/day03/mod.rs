#[derive(Debug)]
enum Error {
    InvalidInput,
}

fn count_trees(grid: &[Vec<char>], slope: (usize, usize)) -> crate::Result<i32> {
    let h = grid.len();
    if h == 0 {
        return Err(crate::Error::boxed(Error::InvalidInput));
    }
    let w = grid[0].len();
    if w == 0 {
        return Err(crate::Error::boxed(Error::InvalidInput));
    }

    let mut pos = (0, 0);
    let mut cnt = 0;
    while pos.1 < h {
        if grid[pos.1][pos.0] == '#' {
            cnt += 1;
        }

        pos.0 = (pos.0 + slope.0) % w;
        pos.1 += slope.1;
    }

    Ok(cnt)
}

pub fn part1(input: &str) -> crate::Result<i32> {
    let grid: Vec<Vec<char>> = input
        .lines()
        .map(|s| s.chars().collect::<Vec<_>>())
        .collect();

    Ok(count_trees(&grid, (3, 1))?)
}

pub fn part2(input: &str) -> crate::Result<i64> {
    let grid: Vec<Vec<char>> = input
        .lines()
        .map(|s| s.chars().collect::<Vec<_>>())
        .collect();
    let slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    let cnts = slopes
        .iter()
        .map(|&s| count_trees(&grid, s))
        .collect::<Result<Vec<_>, _>>()?;

    Ok(cnts.iter().map(|&i| i as i64).product())
}
