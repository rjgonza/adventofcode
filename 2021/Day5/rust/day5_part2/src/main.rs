use std::collections::HashSet;

static INPUT_FILE: &str = include_str!("../../../input.txt");

fn part2(s: &str) -> isize {
    let mut intersects: HashSet<(isize, isize)> = HashSet::new();
    let mut items: HashSet<(isize, isize)> = HashSet::new();

    for line in s.lines() {
        let (left, right) = line.split_once(" -> ").unwrap();
        let (x1, y1) = parse_point(left.split_once(',').unwrap());
        let (x2, y2) = parse_point(right.split_once(',').unwrap());

        // dbg!((left, right));
        if x1 == x2 {
            for y in y1.min(y2)..=y1.max(y2) {
                let point = (x1, y);
                if !items.insert(point) {
                    intersects.insert(point);
                }
            }
        } else if y1 == y2 {
            for x in x1.min(x2)..=x1.max(x2) {
                let coord = (x, y1);
                if !items.insert(coord) {
                    intersects.insert(coord);
                }
            }
        } else {
            let x_direction = if x1 < x2 { 1 } else { -1 };
            let y_direction = if y1 < y2 { 1 } else { -1 };

            for distance in 0..=(x2 - x1).abs() {
                let point = (x1 + (distance * x_direction), y1 + (distance * y_direction));
                if !items.insert(point) {
                    intersects.insert(point);
                }
            }
        }
    }
    intersects.len() as isize
}

fn parse_point((l, r): (&str, &str)) -> (isize, isize) {
    (l.parse::<isize>().unwrap(), r.parse::<isize>().unwrap())
}
fn main() {
    println!("{}", part2(INPUT_FILE));
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
    fn test_part2() {
        assert_eq!(part2(INPUT), 12);
    }
}
