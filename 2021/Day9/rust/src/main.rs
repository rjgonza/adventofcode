use std::io::BufRead;

static INPUT_FILE: &'static str = include_str!("../../input.txt");

fn main() {
    println!("Part 1: {}", part1(INPUT_FILE));
    println!("Part 2: {}", part2(INPUT_FILE));
}

fn part1(input: &str) -> usize {
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

    for row in heightmap {
        // dbg!(row.min());
    }
    15
}

fn part2(_input: &str) -> usize {
    todo!();
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
        assert_eq!(part2(INPUT), 0);
    }
}
