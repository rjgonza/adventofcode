static INPUT_FILE: &str = include_str!("../../input.txt");

fn main() {
    println!("Part 1: {}", part1(INPUT_FILE));
    println!("Part 2: {}", part2(INPUT_FILE));
}

fn part1(input: &str) -> usize {
    let mut groups: Vec<(&str, &str)> = Vec::new();
    for line in input.lines() {
        groups.push(line.split_once(',').unwrap());
    }

    let mut overlaps = 0;
    for group in groups {
        let first = group.0.split_once('-').unwrap();
        let second = group.1.split_once('-').unwrap();
        let first_left: usize = first.0.parse().unwrap();
        let first_right: usize = first.1.parse().unwrap();
        let second_left: usize = second.0.parse().unwrap();
        let second_right: usize = second.1.parse().unwrap();

        let left: Vec<usize> = (first_left..=first_right).collect();
        let right: Vec<usize> = (second_left..=second_right).collect();

        let mut left_contained = true;
        for i in &left {
            if !right.contains(&i) {
                left_contained = false;
            }
        }

        let mut right_contained = true;
        for i in &right {
            if !left.contains(&i) {
                right_contained = false;
            }
        }

        if left_contained || right_contained {
            overlaps += 1;
        }
    }
    overlaps
}

fn part2(input: &str) -> usize {
    let mut groups: Vec<(&str, &str)> = Vec::new();
    for line in input.lines() {
        groups.push(line.split_once(',').unwrap());
    }

    let mut overlaps = 0;
    for group in groups {
        let first = group.0.split_once('-').unwrap();
        let second = group.1.split_once('-').unwrap();
        let first_left: usize = first.0.parse().unwrap();
        let first_right: usize = first.1.parse().unwrap();
        let second_left: usize = second.0.parse().unwrap();
        let second_right: usize = second.1.parse().unwrap();

        let left: Vec<usize> = (first_left..=first_right).collect();
        let right: Vec<usize> = (second_left..=second_right).collect();

        let mut left_contained = false;
        for i in &left {
            if right.contains(&i) {
                left_contained = true;
                break;
            }
        }

        let mut right_contained = false;
        if !left_contained {
            for i in &right {
                if left.contains(&i) {
                    right_contained = true;
                    break;
                }
            }
        }

        if left_contained || right_contained {
            overlaps += 1;
        }
    }
    overlaps
}

#[cfg(test)]

mod test {
    use super::*;
    const INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 2);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 4);
    }
}
