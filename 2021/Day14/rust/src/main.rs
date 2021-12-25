use std::collections::HashMap;

static INPUT_FILE: &str = include_str!("../../input.txt");

fn main() {
    println!("Part 1: {}", part1(INPUT_FILE));
    println!("Part 2: {}", part2(INPUT_FILE));
}

fn part1(input: &str) -> usize {
    let (template, pairs) = input.split_once("\n\n").unwrap();
    let pair_inserts: HashMap<(char, char), char> = pairs
        .lines()
        .map(|line| {
            let mut iter = line.trim().split(" -> ");
            let pair = iter.next().unwrap();
            let mut pair = pair.chars();
            let lpair = pair.next().unwrap();
            let rpair = pair.next().unwrap();
            let insert = iter.next().unwrap().chars().next().unwrap();
            ((lpair, rpair), insert)
        })
        .collect();
    // dbg!(template);
    // dbg!(pairs);
    // dbg!(pair_inserts);

    let mut template: Vec<_> = template.chars().collect();
    let mut counts: HashMap<char, usize> = HashMap::new();

    for c in template.iter() {
        counts.entry(*c).and_modify(|val| *val += 1).or_insert(1);
    }

    for _ in 0..10 {
        let mut next_state = vec!['0'; 2 * template.len() - 1];
        for i in 0..template.len() - 1 {
            next_state[2 * i] = template[i];
            let new_char = *pair_inserts.get(&(template[i], template[i + 1])).unwrap();
            next_state[2 * i + 1] = new_char;
            counts
                .entry(new_char)
                .and_modify(|val| *val += 1)
                .or_insert(1);
        }

        let prev_last = template.len() - 1;
        next_state[2 * prev_last] = template[prev_last];

        template = next_state;
    }

    let min = counts.values().min().unwrap();
    let max = counts.values().max().unwrap();

    max - min
}

fn part2(input: &str) -> usize {
    let (template, data) = input.split_once("\n\n").unwrap();
    let mut pairs: Vec<(char, char)> = Vec::new();
    let mut pair_counts: HashMap<(char, char), usize> = HashMap::new();
    let pair_inserts: HashMap<(char, char), char> = data
        .lines()
        .map(|line| {
            let mut iter = line.trim().split(" -> ");
            let pair = iter.next().unwrap();
            let mut pair = pair.chars();
            let lpair = pair.next().unwrap();
            let rpair = pair.next().unwrap();
            let insert = iter.next().unwrap().chars().next().unwrap();
            pairs.push((lpair, rpair));
            pair_counts.insert((lpair, rpair), 0);
            ((lpair, rpair), insert)
        })
        .collect();
    // dbg!(template);
    // dbg!(data);
    // dbg!(pair_inserts);

    let template: Vec<_> = template.chars().collect();
    let mut counts: HashMap<char, usize> = HashMap::new();

    for c in template.iter() {
        counts.entry(*c).and_modify(|val| *val += 1).or_insert(1);
    }
    // dbg!(template.clone());
    // dbg!(counts.clone());

    for i in 0..template.len() - 1 {
        pair_counts
            .entry((template[i], template[i + 1]))
            .and_modify(|val| *val += 1);
    }

    for _ in 0..40 {
        let mut next_pair_counts = pair_counts.clone();

        for (in_a, in_b) in pairs.iter() {
            let pair_count = *pair_counts.get(&(*in_a, *in_b)).unwrap();
            if pair_count < 1 {
                continue;
            }

            let new_char = *pair_inserts.get(&(*in_a, *in_b)).unwrap();
            counts
                .entry(new_char)
                .and_modify(|val| *val += pair_count)
                .or_insert(pair_count);

            next_pair_counts
                .entry((*in_a, *in_b))
                .and_modify(|val| *val -= pair_count);
            next_pair_counts
                .entry((*in_a, new_char))
                .and_modify(|val| *val += pair_count);
            next_pair_counts
                .entry((new_char, *in_b))
                .and_modify(|val| *val += pair_count);
        }

        pair_counts = next_pair_counts;
    }

    let min = counts.values().min().unwrap();
    let max = counts.values().max().unwrap();

    max - min
}

#[cfg(test)]

mod test {
    use super::*;
    const INPUT: &str = "NNCB

    CH -> B
    HH -> N
    CB -> H
    NH -> C
    HB -> C
    HC -> B
    HN -> C
    NN -> C
    BH -> H
    NC -> B
    NB -> B
    BN -> B
    BB -> N
    BC -> B
    CC -> N
    CN -> C";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 1588);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 2188189693529);
    }
}
