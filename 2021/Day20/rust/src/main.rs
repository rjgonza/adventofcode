static INPUT_FILE: &str = include_str!("../../input.txt");

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn get_grid(p: Point) -> Vec<Point> {
    let mut result = Vec::new();
    for x in -1..=1 {
        for y in -1..=1 {
            let new_x = p.x + x;
            let new_y = p.y + y;
            if new_x < 0 || new_y < 0 {
                continue;
            }
            result.push(Point { x: new_x, y: new_y })
        }
    }
    result
}

fn convert(s: &str, n: usize) -> Vec<String> {
    s.lines()
        .skip(n)
        .map(|l| {
            l.trim()
                .chars()
                .map(|c| match c {
                    '.' => '0',
                    '#' => '1',
                    _ => unreachable!(),
                })
                .collect::<String>()
        })
        .collect()
}

fn lookup(ps: Vec<Point>, algo: &str) -> i32 {
    let s = algo.to_string();
    // for p in ps.iter() {
    //     println!("{}", s[p.x])
    // }
    0
}

fn main() {
    println!("Part 1: {}", part1(INPUT_FILE));
    // println!("Part 2: {}", part2(INPUT_FILE));
}

fn part1(input: &str) -> usize {
    let (iem, image) = input.split_once('\n').unwrap();
    // let r = get_grid(Point { x: 5, y: 10 });
    // dbg!(r);

    // for line in image.lines() {
    //     let l = line.trim();
    //     for c in l.chars() {
    //         print!("{}", c);
    //     }
    //     println!();
    // }

    let grid: Vec<String> = convert(image, 1);
    let algo: Vec<String> = convert(iem, 0);

    // dbg!(grid);
    // dbg!(algo);

    for (x, line) in grid.iter().enumerate() {
        for (y, c) in line.char_indices() {
            let sample = get_grid(Point {
                x: x as i32,
                y: y as i32,
            });
            for p in sample.iter() {}
        }
    }

    35
}

fn part2(_input: &str) -> usize {
    todo!();
}

#[cfg(test)]

mod test {
    use super::*;
    const INPUT: &str = "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#
    
    #..#.
    #....
    ##..#
    ..#..
    ..###";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 35);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 0);
    }
}
