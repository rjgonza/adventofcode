use std::collections::HashSet;

static INPUT_FILE: &str = include_str!("../../input.txt");

#[derive(Debug, Clone, Copy)]
enum Fold {
    X(usize),
    Y(usize),
}

fn part1(input: &str) -> usize {
    let mut coords: HashSet<(usize, usize)> = HashSet::new();
    let mut folds: Vec<Fold> = Vec::new();

    for lines in input.lines() {
        if lines.contains(',') {
            let this: (&str, &str) = lines.trim().split_once(',').unwrap();
            coords.insert((this.0.parse().unwrap(), this.1.parse().unwrap()));
        } else if lines.contains('=') {
            let data: Vec<&str> = lines.split_whitespace().collect();
            let (var, val) = data[data.len() - 1].split_once('=').unwrap();
            let fold = match var {
                "x" => Fold::X(val.parse().unwrap()),
                "y" => Fold::Y(val.parse().unwrap()),
                _ => panic!("Unable to parse fold"),
            };
            folds.push(fold);
        }
    }

    // coords.sort();
    // dbg!(coords);
    // // dbg!(folds.clone());

    // let mut x_bound = 0;
    // let mut y_bound = 0;
    // for coord in coords.iter() {
    //     if coord.0 > x_bound {
    //         x_bound = coord.0 + 2;
    //     }

    //     if coord.1 > y_bound {
    //         y_bound = coord.1 + 1;
    //     }
    // }
    // // dbg!(coords.clone());

    // let mut grid: Vec<Vec<char>> = vec![vec!['.'; x_bound]; y_bound];
    // // dbg!(grid.len());
    // // dbg!(grid[0].len());
    // // dbg!(x_bound);
    // // dbg!(y_bound);

    // for (y, row) in grid.iter_mut().enumerate() {
    //     for (x, col) in row.iter_mut().enumerate() {
    //         for coord in coords.iter() {
    //             if x == coord.0 && y == coord.1 {
    //                 // dbg!(coord.1);
    //                 // dbg!(coord.0);
    //                 *col = '#';
    //                 break;
    //             } else {
    //                 *col = '.';
    //             }
    //         }
    //     }
    // }

    // dbg!(grid.clone());
    // for x in grid.clone().iter() {
    //     for y in x.iter() {
    //         print!("{}", y)
    //     }
    //     println!();
    // }
    // println!();

    for fold in folds.iter().take(1) {
        coords = coords
            .iter()
            .map(|(mut x, mut y)| match fold {
                Fold::X(line) => {
                    if x > *line {
                        let delta = x - line;
                        x = line - delta;
                    }
                    (x, y)
                }
                Fold::Y(line) => {
                    if y > *line {
                        let delta = y - line;
                        y = line - delta;
                    }
                    (x, y)
                }
            })
            .collect();
    }
    // for (x, y) in coords.iter() {
    //     match fold {
    //         Fold::X(line) => {
    //             if x >= line {
    //                 let delta = x - line;
    //                 x = &(line - delta);
    //                 // grid[coord_x - x - (coord_x - x)][coord_y] = '#';
    //             }
    //         }
    //         Fold::Y(line) => {
    //             if y >= line {
    //                 let delta = y - line;
    //                 y = &(line - delta);
    //             }
    //         }
    //         _ => unreachable!(),
    //     };
    // }
    // }

    // println!();
    // let mut count = 0;
    // // dbg!(grid.clone());
    // for x in grid.clone().iter() {
    //     for y in x.iter() {
    //         print!("{}", y);
    //         if *y == '#' {
    //             count += 1;
    //         }
    //     }
    //     println!();

    coords.len()
}

fn part2(input: &str) -> usize {
    let mut coords: HashSet<(usize, usize)> = HashSet::new();
    let mut folds: Vec<Fold> = Vec::new();

    for lines in input.lines() {
        if lines.contains(',') {
            let this: (&str, &str) = lines.trim().split_once(',').unwrap();
            coords.insert((this.0.parse().unwrap(), this.1.parse().unwrap()));
        } else if lines.contains('=') {
            let data: Vec<&str> = lines.split_whitespace().collect();
            let (var, val) = data[data.len() - 1].split_once('=').unwrap();
            let fold = match var {
                "x" => Fold::X(val.parse().unwrap()),
                "y" => Fold::Y(val.parse().unwrap()),
                _ => panic!("Unable to parse fold"),
            };
            folds.push(fold);
        }
    }

    for fold in folds.iter() {
        coords = coords
            .iter()
            .map(|(mut x, mut y)| match fold {
                Fold::X(line) => {
                    if x > *line {
                        let delta = x - line;
                        x = line - delta;
                    }
                    (x, y)
                }
                Fold::Y(line) => {
                    if y > *line {
                        let delta = y - line;
                        y = line - delta;
                    }
                    (x, y)
                }
            })
            .collect();
    }

    let mut x_bound = 0;
    let mut y_bound = 0;
    for coord in coords.iter() {
        if coord.0 > x_bound {
            x_bound = coord.0 + 2;
        }

        if coord.1 > y_bound {
            y_bound = coord.1 + 1;
        }
    }

    dbg!(y_bound);
    dbg!(x_bound);
    for y in 0..y_bound {
        for x in 0..x_bound {
            if coords.contains(&(x, y)) {
                print!("█");
            } else {
                print!(" ");
            }
        }
        println!();
    }
    0
}

fn main() {
    println!("Part 1: {}", part1(INPUT_FILE));
    println!("Part 2: {}", part2(INPUT_FILE));
}

#[cfg(test)]

mod test {
    use super::*;
    const INPUT: &str = "6,10
    0,14
    9,10
    0,3
    10,4
    4,11
    6,0
    6,12
    4,1
    0,13
    10,12
    3,4
    3,0
    8,4
    1,10
    2,14
    8,10
    9,0
    
    fold along y=7
    fold along x=5";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 17);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 0);
    }
}
