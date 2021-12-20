use std::collections::{HashMap, HashSet, VecDeque};

static INPUT_FILE: &str = include_str!("../../input.txt");

fn part1(input: &str) -> usize {
    let mut paths: HashMap<String, Vec<String>> = HashMap::new();

    for line in input.lines() {
        let (left, right) = line.trim().split_once('-').unwrap();
        paths
            .entry(left.to_string())
            .and_modify(|e| e.push(right.to_string()))
            .or_insert_with(|| vec![right.to_string()]);

        paths
            .entry(right.to_string())
            .and_modify(|e| e.push(left.to_string()))
            .or_insert_with(|| vec![left.to_string()]);
    }

    let mut q: VecDeque<Vec<String>> = VecDeque::new();
    q.push_back(vec![String::from("start")]);
    let mut num_paths = 0;

    while let Some(path) = q.pop_front() {
        let last = path.last().unwrap();
        for n in paths.get(last).unwrap().iter() {
            if n.chars().next().unwrap().is_ascii_lowercase() && path.contains(n) {
                continue;
            }

            if n == "end" {
                num_paths += 1;
                continue;
            }

            let mut new_path = path.clone();
            new_path.push(n.clone());
            q.push_back(new_path);
        }
    }
    num_paths
}

#[derive(Debug, Clone)]
struct Path<'a> {
    current: &'a str,
    prev_caves: HashSet<&'a str>,
    has_double_lowercase: bool,
}

fn part2(input: &str) -> usize {
    let mut neighbors_of: HashMap<&str, Vec<&str>> = HashMap::new();
    for line in input.lines() {
        let mut chunks = line.split('-');
        let a = chunks.next().unwrap();
        let b = chunks.next().unwrap();
        neighbors_of
            .entry(a)
            .and_modify(|neighbors| neighbors.push(b))
            .or_insert_with(|| vec![b]);
        neighbors_of
            .entry(b)
            .and_modify(|neighbors| neighbors.push(a))
            .or_insert_with(|| vec![a]);
    }

    let mut q: VecDeque<Path> = VecDeque::new();
    q.push_back(Path {
        current: "start",
        prev_caves: HashSet::new(),
        has_double_lowercase: false,
    });

    let mut num_paths = 0;
    while let Some(path) = q.pop_front() {
        for n in neighbors_of.get(&path.current).unwrap().iter() {
            if *n == "start" {
                continue;
            }

            if *n == "end" {
                num_paths += 1;
                continue;
            }

            let mut has_double_lowercase = path.has_double_lowercase;
            if n.chars().next().unwrap().is_ascii_lowercase() && path.prev_caves.contains(n) {
                if path.has_double_lowercase {
                    continue;
                }

                has_double_lowercase = true;
            }

            let mut new_path = Path {
                current: n,
                prev_caves: path.prev_caves.clone(),
                has_double_lowercase,
            };
            new_path.prev_caves.insert(n);

            q.push_back(new_path);
        }
    }

    num_paths
}

fn main() {
    println!("Part 1: {}", part1(INPUT_FILE));
    println!("Part 2: {}", part2(INPUT_FILE));
}

#[cfg(test)]

mod test {
    use super::*;
    const INPUT1: &str = "start-A
    start-b
    A-c
    A-b
    b-d
    A-end
    b-end";

    const INPUT2: &str = "dc-end
    HN-start
    start-kj
    dc-start
    dc-HN
    LN-dc
    HN-end
    kj-sa
    kj-HN
    kj-dc";

    const INPUT3: &str = "fs-end
    he-DX
    fs-he
    start-DX
    pj-DX
    end-zg
    zg-sl
    zg-pj
    pj-he
    RW-he
    fs-DX
    pj-RW
    zg-RW
    start-pj
    he-WI
    zg-he
    pj-fs
    start-RW";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT1), 10);
        assert_eq!(part1(INPUT2), 19);
        assert_eq!(part1(INPUT3), 226);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT1), 36);
        assert_eq!(part2(INPUT2), 103);
        assert_eq!(part2(INPUT3), 3509);
    }
}
