static INPUT_FILE: &str = include_str!("../../input.txt");

struct FishBuckets {
    buckets: [usize; 9],
}

impl FishBuckets {
    fn new() -> Self {
        Self { buckets: [0; 9] }
    }
    fn insert(&mut self, index: usize) {
        self.buckets[index] += 1;
    }

    fn step(&mut self) {
        self.buckets.rotate_left(1);
        self.buckets[6] += self.buckets[8]
    }

    fn count(&self) -> usize {
        self.buckets.into_iter().sum()
    }
}

fn part1(input: &str) -> usize {
    let mut fish = FishBuckets::new();

    for number in input.split(',') {
        fish.insert(number.trim().parse().unwrap());
    }

    for _ in 0..80 {
        fish.step();
    }

    fish.count()
}

fn part2(input: &str) -> usize {
    let mut fish = FishBuckets::new();

    for number in input.split(',') {
        fish.insert(number.trim().parse().unwrap());
    }

    for _ in 0..256 {
        fish.step();
    }

    fish.count()
}

fn main() {
    println!("{}", part1(INPUT_FILE));
    println!("{}", part2(INPUT_FILE));
}

#[cfg(test)]

mod test {
    use super::*;
    const INPUT: &str = "3,4,3,1,2";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 5934);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 26984457539);
    }
}
