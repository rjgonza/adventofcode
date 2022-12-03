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
            _ => Err(s.to_string()),
        }
    }
}
enum Outcomes {
    Win = 6,
    Lose = 0,
    Draw = 3,
}

impl FromStr for Outcomes {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Outcomes::Lose),
            "Y" => Ok(Outcomes::Draw),
            "Z" => Ok(Outcomes::Win),
            _ => Err(s.to_string()),
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
            let game: Vec<RPSChoices> = line
                .split(" ")
                .map(|choice| choice.parse::<RPSChoices>().unwrap())
                .collect();

            if game[0].beats() == game[1] {
                game[1] as usize + 0
            } else if game[1].beats() == game[0] {
                game[1] as usize + 6
            } else {
                game[1] as usize + 3
            }
        })
        .sum();
    strategy
}

fn part2(input: &str) -> usize {
    let strategy = input
        .lines()
        .map(|line| -> usize {
            let moves: Vec<&str> = line.split(' ').collect();
            let res = moves[1].parse::<Outcomes>().unwrap();
            let opponent = moves[0].parse::<RPSChoices>().unwrap();

            match res {
                Outcomes::Win => {
                    let choice = match opponent {
                        RPSChoices::Rock => RPSChoices::Paper,
                        RPSChoices::Paper => RPSChoices::Scissors,
                        RPSChoices::Scissors => RPSChoices::Rock,
                    };
                    res as usize + choice as usize
                }
                Outcomes::Lose => {
                    let choice = match opponent {
                        RPSChoices::Rock => RPSChoices::Scissors,
                        RPSChoices::Paper => RPSChoices::Rock,
                        RPSChoices::Scissors => RPSChoices::Paper,
                    };
                    res as usize + choice as usize
                }
                Outcomes::Draw => res as usize + opponent as usize,
            }
        })
        .sum();
    strategy
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

    // Y - My Paper || Must Draw
    // X - My Rock || Must lose
    // Z - My Scissors || Must Win

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 15);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 12);
    }
}
