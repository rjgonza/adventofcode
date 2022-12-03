static INPUT_FILE: &str = include_str!("../../input.txt");

fn main() {
    println!("Part 1: {}", part1(INPUT_FILE));
    println!("Part 2: {}", part2(INPUT_FILE));
}

fn part1(input: &str) -> usize {
    // let mut commons: Vec<char> = Vec::new();
    let mut sum: usize = 0;
    for line in input.lines() {
        let rucksack: Vec<char> = line.chars().collect();
        let len = rucksack.len();
        let mid = len / 2;

        let compartment1 = &rucksack[0..mid];
        let compartment2 = &rucksack[mid..len];

        for c in compartment1 {
            if compartment2.contains(c) {
                // println!("The common one is: {}", c);
                // commons.push(*c);
                sum += match *c {
                    'a'..='z' => (*c as usize) - b'a' as usize + 1,
                    'A'..='Z' => (*c as usize) - b'A' as usize + 27,
                    _ => panic!("Unknown char"),
                };
                // println!("Sum: {}", sum);
                break;
            }
        }
        // println!("Rucksack: {:?}", rucksack);
        // println!("Compartment1: {:?}", compartment1);
        // println!("Compartment2: {:?}", compartment2);
        // rucksacks.push(rucksack);
    }
    sum as usize
}

fn part2(input: &str) -> usize {
    let mut sum: usize = 0;

    for set in input.lines().collect::<Vec<&str>>().chunks(3) {
        sum += set[0]
            .chars()
            .filter(|c| set[1].contains(*c) && set[2].contains(*c))
            .map(|c| match c {
                'a'..='z' => (c as usize) - b'a' as usize + 1,
                'A'..='Z' => (c as usize) - b'A' as usize + 27,
                _ => panic!("Unknown char"),
            })
            .take(1)
            .sum::<usize>();
    }
    sum
}

#[cfg(test)]

mod test {
    use super::*;
    const INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 157);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 70);
    }
}
