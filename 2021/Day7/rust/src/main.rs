static INPUT_FILE: &'static str = include_str!("../../input.txt");

fn part1(input: &str) -> usize {
    let mut crab_positions: Vec<usize> = Vec::new();

    crab_positions = input.split(',').map(|c| c.parse().unwrap()).collect();

    dbg!(crab_positions);
    0
}

fn part2(_input: &str) -> usize {
    0
}

fn main() {
    println!("Part 1: {}", part1(INPUT_FILE));
    println!("Part 2: {}", part2(INPUT_FILE));
}

#[cfg(test)]

mod test {
    use super::*;
    const INPUT: &str = "16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 37);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 0);
    }
}
