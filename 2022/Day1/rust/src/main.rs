static INPUT_FILE: &str = include_str!("../../input.txt");

fn main() {
    println!("Part 1: {}", part1(INPUT_FILE));
    println!("Part 2: {}", part2(INPUT_FILE));
}

fn part1(input: &str) -> usize {
    let largest_calories = input
        .split("\n\n")
        .map(|elf| {
            elf.lines()
                .map(|calories| calories.parse::<usize>().unwrap())
                .sum()
        })
        .max()
        .unwrap();

    largest_calories
}

fn part2(input: &str) -> usize {
    let mut largest_calories = input
        .split("\n\n")
        .map(|elf| {
            elf.lines()
                .map(|calories| calories.parse::<usize>().unwrap())
                .sum::<usize>()
        })
        .collect::<Vec<_>>();

    largest_calories.sort_by(|a, b| b.cmp(a));
    let sum = largest_calories.iter().take(3).sum();
    sum
}

#[cfg(test)]

mod test {
    use super::*;
    static TEST_INPUT: &str = include_str!("../../test_input.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 24000);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 45000);
    }
}
