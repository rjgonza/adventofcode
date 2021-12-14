static INPUT_FILE: &str = include_str!("../../input.txt");

fn part1(_input: &str) -> usize {
    todo!();
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
    const INPUT: &str = "";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 0);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 0);
    }
}

