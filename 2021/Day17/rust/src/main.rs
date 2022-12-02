

static INPUT_FILE: &str = include_str!("../../input.txt");

fn main() {
    println!("Part 1: {}", part1(INPUT_FILE));
    println!("Part 2: {}", part2(INPUT_FILE));
}

fn part1(input: &str) -> i32 {
    let mut x_range = (0, 0);
    let mut y_range = (0, 0);
    let mut best_y = i32::MIN;
    for line in input.lines() {
        let data = line.split_once(':').unwrap().1.split_once(',').unwrap();
        let x = data.0.split_once('=').unwrap().1.split_once("..").unwrap();
        x_range = (x.0.parse().unwrap(), x.1.parse().unwrap());
        let y = data.1.split_once('=').unwrap().1.split_once("..").unwrap();
        y_range = (y.0.parse().unwrap(), y.1.parse().unwrap());
    }
    // dbg!(x_range);
    // dbg!(y_range);

    for x_init_val in 1..100 {
        for y_init_val in -100..100 {
            let mut x = 0;
            let mut y = 0;
            let mut x_val = x_init_val;
            let mut y_val = y_init_val;
            let mut highest_y = i32::MIN;

            loop {
                x += x_val;
                if x_val > 0 {
                    x_val -= 1;
                }

                y += y_val;
                y_val -= 1;

                highest_y = highest_y.max(y);

                if x_range.0 <= x && x <= x_range.1 && y_range.0 <= y && y <= y_range.1 {
                    best_y = best_y.max(highest_y);
                    break;
                }

                if x_range.1 < x || y < y_range.0 {
                    break;
                }
            }
        }
    }
    best_y
}

fn part2(input: &str) -> i32 {
    let mut x_range = (0, 0);
    let mut y_range = (0, 0);
    let mut best_y = i32::MIN;
    let mut results = 0;
    for line in input.lines() {
        let data = line.split_once(':').unwrap().1.split_once(',').unwrap();
        let x = data.0.split_once('=').unwrap().1.split_once("..").unwrap();
        x_range = (x.0.parse().unwrap(), x.1.parse().unwrap());
        let y = data.1.split_once('=').unwrap().1.split_once("..").unwrap();
        y_range = (y.0.parse().unwrap(), y.1.parse().unwrap());
    }
    // dbg!(x_range);
    // dbg!(y_range);

    for x_init_val in 1..1000 {
        for y_init_val in -1000..1000 {
            let mut x = 0;
            let mut y = 0;
            let mut x_val = x_init_val;
            let mut y_val = y_init_val;
            let mut highest_y = i32::MIN;

            loop {
                x += x_val;
                if x_val > 0 {
                    x_val -= 1;
                }

                y += y_val;
                y_val -= 1;

                highest_y = highest_y.max(y);

                if x_range.0 <= x && x <= x_range.1 && y_range.0 <= y && y <= y_range.1 {
                    best_y = best_y.max(highest_y);
                    results += 1;
                    break;
                }

                if x_range.1 < x || y < y_range.0 {
                    break;
                }
            }
        }
    }
    results
}

#[cfg(test)]

mod test {
    use super::*;
    const INPUT: &str = "target area: x=20..30, y=-10..-5";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 45);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 112);
    }
}
