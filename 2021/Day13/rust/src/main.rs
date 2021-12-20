use std::collections::HashMap;

static INPUT_FILE: &str = include_str!("../../input.txt");

fn part1(input: &str) -> usize {
    let mut coords: Vec<(usize, usize)> = Vec::new();
    let mut folds: HashMap<&str, usize> = HashMap::new();

    for lines in input.lines() {
        if lines.contains(',') {
            let this: (&str, &str) = lines.trim().split_once(',').unwrap();
            coords.push((this.0.parse().unwrap(), this.1.parse().unwrap()));
        } else if lines.contains('=') {
            let data: Vec<&str> = lines.trim().split_whitespace().collect();
            let (var, val) = data[data.len() - 1].split_once('=').unwrap();
            folds.insert(var, val.parse().unwrap());
        }
    }

    dbg!(coords);
    dbg!(folds);
    0
}

fn part2(_input: &str) -> usize {
    todo!();
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
        assert_eq!(part1(INPUT), 0);
    }

    // #[test]
    // fn test_part2() {
    //     assert_eq!(part2(INPUT), 0);
    // }
}
