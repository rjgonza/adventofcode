use std::str::FromStr;

static INPUT_FILE: &str = include_str!("../../input.txt");

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum RPSChoices {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl FromStr for RPSChoices {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(RPSChoices::Rock),
            "B" | "Y" => Ok(RPSChoices::Paper),
            "C" | "Z" => Ok(RPSChoices::Scissors),
            _ => Err("Unknown move".to_string()),
        }
    }
}

trait Beats {
    fn beats(&self) -> Self;
}

impl Beats for RPSChoices {
    fn beats(&self) -> Self {
        match *self {
            RPSChoices::Rock => RPSChoices::Scissors,
            RPSChoices::Paper => RPSChoices::Rock,
            RPSChoices::Scissors => RPSChoices::Paper,
        }
    }
}

fn main() {
    println!("Part 1: {}", part1(INPUT_FILE));
    println!("Part 2: {}", part2(INPUT_FILE));
}

fn part1(input: &str) -> usize {
    let strategy = input
        .lines()
        .map(|line| {
            let choices: Vec<RPSChoices> = line
                .split(' ')
                .map(|choice| choice.parse().unwrap())
                .collect();
            {
                let mut outcome = 0;
                if choices[0].beats() == choices[1] {
                    outcome = 0
                } else if choices[1].beats() == choices[0] {
                    outcome = 6
                } else if choices[0].beats() == choices[1].beats() {
                    outcome = 3
                }
                outcome + choices[1] as usize
            }
        })
        .sum();
    strategy
}

fn part2(_input: &str) -> usize {
    0
}

#[cfg(test)]

mod test {
    use super::*;
    const INPUT: &str = "A Y
    B X
    C Z";

    // A - Opponent Rock
    // B - Opponent Paper
    // C - Opponent Scissors

    // Y - My Paper
    // X - My Rock
    // Z - My Scissors

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 0);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 0);
    }
}
