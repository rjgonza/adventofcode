static INPUT_FILE: &'static str = include_str!("../../input.txt");

fn part1(input: &str) -> i32 {
    let mut crab_positions: Vec<i32> = Vec::new();
    let mut crab_to_get_to: i32 = 0;
    let mut fuel_spent: i32 = 0;

    crab_positions = input
        .split(',')
        .map(|c| c.parse::<i32>().unwrap())
        .collect();

    crab_positions.sort();
    let mid = crab_positions.len() / 2;
    if crab_positions.len() % 2 == 0 {
        let a = crab_positions[mid - 1];
        let b = crab_positions[mid];
        crab_to_get_to = (a + b) / 2;
    } else {
        crab_to_get_to = crab_positions[mid];
    }

    for crab in crab_positions {
        fuel_spent += crab_to_get_to.min(crab) - crab.max(crab_to_get_to);
    }

    fuel_spent.abs()
}

fn part2(input: &str) -> i32 {
    let mut crab_positions: Vec<i32> = Vec::new();
    let mut crab_to_get_to: i32 = 0;
    let mut fuel_spent: i32 = 0;

    crab_positions = input
        .split(',')
        .map(|c| c.parse::<i32>().unwrap())
        .collect();

    crab_positions.sort();
    let mid = crab_positions.len() / 2;
    if crab_positions.len() % 2 == 0 {
        let a = crab_positions[mid - 1];
        let b = crab_positions[mid];
        crab_to_get_to = (a + b) / 2;
    } else {
        crab_to_get_to = crab_positions[mid];
    }

    for crab in crab_positions {
        fuel_spent += crab_to_get_to.min(crab) - crab.max(crab_to_get_to);
    }

    fuel_spent.abs()
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
        assert_eq!(part2(INPUT), 206);
    }
}
