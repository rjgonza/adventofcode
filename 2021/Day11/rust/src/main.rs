static INPUT_FILE: &str = include_str!("../../input.txt");

fn neighbors(i: usize, j: usize) -> Vec<(usize, usize)> {
    let mut nei: Vec<(usize, usize)> = Vec::new();
    for di in -1..=1 {
        // ignore out-of-bounds rows
        let cur_i = (i as i32) + di;
        if !(0..10).contains(&cur_i) {
            continue;
        }

        for dj in -1..=1 {
            // ignore self
            if di == 0 && dj == 0 {
                continue;
            }

            // ignore out-of-bounds columns
            let cur_j = (j as i32) + dj;
            if !(0..10).contains(&cur_j) {
                continue;
            }

            nei.push((cur_i as usize, cur_j as usize));
        }
    }

    nei
}

fn part1(input: &str) -> usize {
    let mut grid: Vec<Vec<usize>> = vec![vec![0; 10]; 10];

    for (i, line) in input.lines().enumerate() {
        for (j, octopi) in line.trim().chars().enumerate() {
            grid[i][j] = octopi.to_digit(10).unwrap() as usize;
        }
    }

    let mut flash_count = 0;
    for _ in 0..100 {
        let mut flashed: Vec<(usize, usize)> = Vec::new();
        for (i, row) in grid.iter_mut().enumerate() {
            for (j, col) in row.iter_mut().enumerate() {
                *col += 1;
                if *col > 9 {
                    flashed.push((i, j));
                }
            }
        }

        while !flashed.is_empty() {
            let (i, j) = flashed.pop().unwrap();
            flash_count += 1;
            grid[i][j] = 0;
            for (ni, nj) in neighbors(i, j) {
                if grid[ni][nj] == 0 || grid[ni][nj] > 9 {
                    continue;
                }

                grid[ni][nj] += 1;
                if grid[ni][nj] > 9 {
                    flashed.push((ni, nj));
                }
            }
        }
    }

    flash_count
}

fn part2(input: &str) -> usize {
    let mut grid: Vec<Vec<usize>> = vec![vec![0; 10]; 10];

    for (i, line) in input.lines().enumerate() {
        for (j, octopi) in line.trim().chars().enumerate() {
            grid[i][j] = octopi.to_digit(10).unwrap() as usize;
        }
    }

    let mut step = 0;
    loop {
        step += 1;
        let mut flashed: Vec<(usize, usize)> = Vec::new();
        for (i, row) in grid.iter_mut().enumerate() {
            for (j, col) in row.iter_mut().enumerate() {
                *col += 1;
                if *col > 9 {
                    flashed.push((i, j));
                }
            }
        }

        let mut step_flash = 0;
        while !flashed.is_empty() {
            let (i, j) = flashed.pop().unwrap();
            step_flash += 1;
            grid[i][j] = 0;

            for (ni, nj) in neighbors(i, j) {
                if grid[ni][nj] == 0 || grid[ni][nj] > 9 {
                    continue;
                }

                grid[ni][nj] += 1;
                if grid[ni][nj] > 9 {
                    flashed.push((ni, nj));
                }
            }
        }

        if step_flash == 100 {
            return step;
        }
    }
}

fn main() {
    println!("Part 1: {}", part1(INPUT_FILE));
    println!("Part 2: {}", part2(INPUT_FILE));
}

#[cfg(test)]

mod test {
    use super::*;
    const INPUT: &str = "5483143223
    2745854711
    5264556173
    6141336146
    6357385478
    4167524645
    2176841721
    6882881134
    4846848554
    5283751526";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 1656);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 195);
    }
}
