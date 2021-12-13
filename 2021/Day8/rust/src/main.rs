static INPUT_FILE: &'static str = include_str!("../../input.txt");
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
        for (_, parts) in line.split_once(" | ") {
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
