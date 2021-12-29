use std::{collections::HashMap, iter::Scan};

static INPUT_FILE: &str = include_str!("../../input.txt");

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Point(isize, isize, isize);

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Beacon(Point);

#[derive(Debug)]
struct Scanner {
    location: Point,
    beacons: Vec<Beacon>,
}

impl Scanner {
    fn add_beacon(&mut self, b: Beacon) {
        self.beacons.push(b);
    }
}

fn parse_input(input: &str) -> Vec<Scanner> {
    let mut scanners: Vec<Scanner> = Vec::new();
    let mut scanner_i = 0;
    for line in input.lines() {
        if line.is_empty() {
            continue;
        }
        if line.contains("---") {
            scanner_i = line.split_whitespace().nth(2).unwrap().parse().unwrap();
            // println!("Scanner! - {}", scanner);
            scanners.push(Scanner {
                location: (Point(0, 0, 0)),
                beacons: Vec::new(),
            });
            continue;
        }

        let coord: Vec<isize> = line.split(',').map(|c| c.parse().unwrap()).collect();
        scanners[scanner_i].add_beacon(Beacon(Point(coord[0], coord[1], coord[2])));
    }
    scanners
    // dbg!(scanners);
}

fn main() {
    println!("Part 1: {}", part1(INPUT_FILE));
    println!("Part 2: {}", part2(INPUT_FILE));
}

fn part1(input: &str) -> usize {
    let mut scanners = parse_input(input);
    // dbg!(scanners);

    let max = scanners[0].beacons.iter().max().unwrap();
    dbg!(max);
    let x = max.0 .0.abs();
    let y = max.0 .1.abs();
    let z = max.0 .2.abs();

    dbg!(x, y, z);

    // let map = vec![vec![vec![' '; x as usize]; y as usize]; z as usize];
    let map = vec![vec!['.'; x as usize]; y as usize];
    for row in map {
        for col in row {
            print!("{}", col);
        }
        println!();
    }

    // for beacon in scanners[0].beacons.iter() {
    //     dbg!(beacon);
    // }
    79
}

fn part2(_input: &str) -> usize {
    todo!();
}

#[cfg(test)]

mod test {
    use super::*;
    static TEST_INPUT: &str = include_str!("../../test_input.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 79);
    }

    // #[test]
    // fn test_part2() {
    //     assert_eq!(part2(TEST_INPUT), 0);
    // }
}
