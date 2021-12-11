use std::collections::HashSet;

static INPUT_FILE: &'static str = include_str!("../../../input.txt");

fn part1(s: &str) -> usize {
    let mut intersects: HashSet<(usize, usize)> = HashSet::new();
    let mut items: HashSet<(usize, usize)> = HashSet::new();

    for line in s.lines() {
        let (left, right) = line.split_once(" -> ").unwrap();
        let (x1, y1) = parse_point(left.split_once(',').unwrap());
        let (x2, y2) = parse_point(right.split_once(',').unwrap());

        if x1 == x2 {
            for y in y1.min(y2)..=y1.max(y2) {
                let coord = (x1, y);
                if !items.insert(coord) {
                    intersects.insert(coord);
                }
            }
        }
        if y1 == y2 {
            for x in x1.min(x2)..=x1.max(x2) {
                let coord = (x, y1);
                if !items.insert(coord) {
                    intersects.insert(coord);
                }
            }
        }
    }
    intersects.len()
}

fn parse_point((l, r): (&str, &str)) -> (usize, usize) {
    (l.parse::<usize>().unwrap(), r.parse::<usize>().unwrap())
}
fn main() {
    println!("{}", part1(INPUT_FILE));
}

#[cfg(test)]

mod test {
    use super::*;

    const INPUT: &str = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 5);
    }
}
