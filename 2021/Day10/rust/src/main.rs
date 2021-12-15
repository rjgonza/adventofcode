static INPUT_FILE: &str = include_str!("../../input.txt");

fn closing(c: char) -> char {
    match c {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => panic!("unrecognized character {}", c),
    }
}

fn part1(input: &str) -> usize {
    let mut score = 0;

    'outer: for line in input.lines() {
        let mut chunck_chars: Vec<char> = Vec::new();
        for value in line.trim().chars() {
            match value {
                '(' | '[' | '{' | '<' => chunck_chars.push(value),
                ')' | ']' | '}' | '>' => {
                    if chunck_chars.is_empty() || value != closing(*chunck_chars.last().unwrap()) {
                        score += match value {
                            ')' => 3,
                            ']' => 57,
                            '}' => 1197,
                            '>' => 25137,
                            _ => panic!("Unable to parse {}", value),
                        };
                        continue 'outer;
                    }
                    chunck_chars.pop();
                }
                _ => panic!("Unable to parse {}", value),
            }
        }
    }

    score
}

fn part2(input: &str) -> isize {
    let mut score_list: Vec<isize> = Vec::new();

    'outer: for line in input.lines() {
        let mut chunck_chars: Vec<char> = Vec::new();
        for value in line.trim().chars() {
            match value {
                '(' | '[' | '{' | '<' => chunck_chars.push(value),
                ')' | ']' | '}' | '>' => {
                    if chunck_chars.is_empty() || value != closing(*chunck_chars.last().unwrap()) {
                        continue 'outer;
                    }
                    chunck_chars.pop();
                }
                _ => panic!("Unable to parse {}", value),
            }
        }

        chunck_chars.reverse();
        let mut score = 0;
        for value in chunck_chars {
            score *= 5;
            score += match closing(value) {
                ')' => 1,
                ']' => 2,
                '}' => 3,
                '>' => 4,
                _ => panic!("Unable to parse {}", value),
            };
        }
        score_list.push(score);
    }

    score_list.sort_unstable();
    let mid = score_list.len() / 2;
    score_list[mid]
}

fn main() {
    println!("Part 1: {}", part1(INPUT_FILE));
    println!("Part 2: {}", part2(INPUT_FILE));
}

#[cfg(test)]

mod test {
    use super::*;
    const INPUT: &str = "[({(<(())[]>[[{[]{<()<>>
        [(()[<>])]({[<{<<[]>>(
        {([(<{}[<>[]}>{[]{[(<()>
        (((({<>}<{<{<>}{[]{[]{}
        [[<[([]))<([[{}[[()]]]
        [{[{({}]{}}([{[{{{}}([]
        {<[[]]>}<{[{[{[]{()[[[]
        [<(<(<(<{}))><([]([]()
        <{([([[(<>()){}]>(<<{{
        <{([{{}}[<[[[<>{}]]]>[]]";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 26397);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 288957);
    }
}
