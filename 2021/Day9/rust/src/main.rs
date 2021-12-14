use std::{
    collections::{HashSet, VecDeque},
    hash::Hash,
};

static INPUT_FILE: &str = include_str!("../../input.txt");

fn main() {
    println!("Part 1: {}", part1(INPUT_FILE));
    println!("Part 2: {}", part2(INPUT_FILE));
}

fn part1(input: &str) -> u32 {
    let mut heightmap = vec![vec![0; 0]; 0];

    for line in input.lines() {
        let row: Vec<u32> = line
            .trim()
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect();
        // dbg!(row);
        heightmap.push(row);
    }
    // dbg!(heightmap[0][0]);

    let y_bound = heightmap[0].len() - 1;
    let x_bound = heightmap.len() - 1;

    let mut risk_sum = 0;
    for (y, row) in heightmap.iter().enumerate() {
        for (x, value) in row.iter().enumerate() {
            if y != 0 && value >= &heightmap[y - 1][x] {
                continue;
            }

            if y != x_bound && value >= &heightmap[y + 1][x] {
                continue;
            }

            if x != 0 && value >= &heightmap[y][x - 1] {
                continue;
            }

            if x != y_bound && value >= &heightmap[y][x + 1] {
                continue;
            }

            risk_sum += value + 1;
        }
    }

    // for row in heightmap {
    //     // dbg!(row.min());
    // }

    risk_sum
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point(usize, usize);

fn part2(input: &str) -> u32 {
    let mut heightmap = vec![vec![0; 0]; 0];

    for line in input.lines() {
        let row: Vec<u32> = line
            .trim()
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect();
        // dbg!(row);
        heightmap.push(row);
    }
    // dbg!(heightmap[0][0]);

    let x_bound = heightmap.len() - 1;
    let y_bound = heightmap[0].len() - 1;

    let mut low_points: Vec<Point> = Vec::new();
    for (x, row) in heightmap.iter().enumerate() {
        for (y, value) in row.iter().enumerate().rev() {
            // dbg!(Point(x, y));
            if x > 0 && value >= &heightmap[x - 1][y] {
                continue;
            }

            if x < x_bound && value >= &heightmap[x + 1][y] {
                continue;
            }

            if y > 0 && value >= &heightmap[x][y - 1] {
                continue;
            }

            if y < y_bound && value >= &heightmap[x][y + 1] {
                continue;
            }
            // dbg!(Point(x, y));
            low_points.push(Point(x, y)); // need to flip the board because we start at the top and not the bottom
        }
    }

    let mut basin_sizes: Vec<usize> = Vec::new();

    for point in low_points.iter() {
        // dbg!(point);
        // dbg!(heightmap[point.0][point.1]);
        let mut seen: HashSet<Point> = HashSet::new();
        let mut window: VecDeque<Point> = VecDeque::new();
        window.push_back(*point);
        seen.insert(*point);

        while !window.is_empty() {
            let current = window.pop_front().unwrap();
            let x = current.0;
            let y = current.1;
            let mut neighbors: Vec<Point> = Vec::new();

            if x > 0 {
                neighbors.push(Point(x - 1, y));
            }

            if x < x_bound {
                neighbors.push(Point(x + 1, y));
            }

            if y > 0 {
                neighbors.push(Point(x, y - 1))
            }

            if y < y_bound {
                neighbors.push(Point(x, y + 1));
            }

            for n in neighbors {
                // dbg!(n);
                if seen.contains(&n) || heightmap[n.0][n.1] == 9 {
                    continue;
                }

                window.push_back(n);
                seen.insert(n);
            }
        }
        basin_sizes.push(seen.len());
    }

    // dbg!(basin_sizes);
    basin_sizes.sort_unstable();
    basin_sizes.reverse();

    let basin_product = basin_sizes[0] * basin_sizes[1] * basin_sizes[2];

    basin_product as u32
}

#[cfg(test)]

mod test {
    use super::*;
    const INPUT: &str = "2199943210
    3987894921
    9856789892
    8767896789
    9899965678";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 15);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 1134);
    }
}
