#![allow(unused_variables)]
#![allow(clippy::needless_collect)]
use std::collections::HashSet;

static INPUT_FILE: &str = include_str!("../../input.txt");
// static ONE: &'static str = "abcefg";
// static TWO: &'static str = "cf";
// static THREE: &'static str = "acdfg";
// static FOUR: &'static str = "bcdf";
// static FIVE: &'static str = "abdfg";
// static SIX: &'static str = "abdefg";
// static SEVEN: &'static str = "acf";
// static EIGHT: &'static str = "abcdefg";
// static NINE: &'static str = "abcdfg";

fn part1(input: &str) -> usize {
    let mut digit_counter = 0;
    for line in input.lines() {
        if let Some((_, parts)) = line.split_once(" | ") {
            // for (_, parts) in line.split_once(" | ") {
            for digits in parts.trim().split_whitespace() {
                let digit = digits.len();
                match digit {
                    2 => digit_counter += 1,
                    3 => digit_counter += 1,
                    4 => digit_counter += 1,
                    7 => digit_counter += 1,
                    _ => (),
                }
            }
        }
    }

    digit_counter
}

fn part2(input: &str) -> usize {
    let mut total = 0;
    for line in input.lines() {
        // for (parts, display) in line.split_once(" | ") {
        if let Some((parts, display)) = line.split_once(" | ") {
            let numbers: Vec<HashSet<_>> = parts
                .trim()
                .split_whitespace()
                .map(|s| HashSet::from_iter(s.chars()))
                .collect();

            let mut left_over = vec![];
            let mut map: Vec<Option<HashSet<char>>> = vec![None; 10];

            for digit_set in numbers.into_iter() {
                match digit_set.len() {
                    2 => map[1] = Some(digit_set),
                    3 => map[7] = Some(digit_set),
                    4 => map[4] = Some(digit_set),
                    7 => map[8] = Some(digit_set),
                    _ => left_over.push(digit_set),
                }
            }

            let head = left_over;
            let mut remainder = vec![];
            for set in head.into_iter() {
                if set.len() == 6 && set.is_superset(map[4].as_ref().unwrap()) {
                    map[9] = Some(set);
                } else if set.len() == 5 && set.is_superset(map[1].as_ref().unwrap()) {
                    map[3] = Some(set);
                } else {
                    remainder.push(set);
                }
            }

            let head = remainder;
            let mut remainder = vec![];
            for set in head.into_iter() {
                if set.len() == 6 && set.is_superset(map[1].as_ref().unwrap()) {
                    map[0] = Some(set);
                } else {
                    remainder.push(set);
                }
            }

            let head = remainder;
            let mut remainder = vec![];
            for set in head.into_iter() {
                if set.len() == 6 {
                    map[6] = Some(set);
                } else {
                    remainder.push(set);
                }
            }

            let head = remainder;
            for set in head.into_iter() {
                if set.is_subset(map[6].as_ref().unwrap()) {
                    map[5] = Some(set);
                } else {
                    map[2] = Some(set);
                }
            }
            let map: Vec<_> = map.into_iter().map(|item| item.unwrap()).collect();

            let mut count = 0;
            for s in display.split_whitespace() {
                let set = HashSet::from_iter(s.chars());
                let n = map.iter().position(|cmp| cmp == &set).unwrap();
                count *= 10;
                count += n;
            }
            total += count;
        }
    }

    total
}

fn main() {
    println!("Part 1: {}", part1(INPUT_FILE));
    println!("Part 2: {}", part2(INPUT_FILE));
}

#[cfg(test)]

mod test {
    use super::*;
    const INPUT: &str =
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
    edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
    fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
    fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
    aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
    fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
    dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
    bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
    egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
    gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 26);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 61229);
    }
}
