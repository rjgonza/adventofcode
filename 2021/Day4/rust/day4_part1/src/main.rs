use std::fs;

#[derive(Debug, Clone, Copy)]
struct bingo_board {
    board: [[i32; 5]; 5],
    marked: [[bool; 5]; 5],
    winner: bool,
}

impl bingo_board {
    fn new(input: &[&str]) -> Self {
        let mut values = [[0; 5]; 5];
        for (x, row) in input.iter().enumerate() {
            let cols: Vec<i32> = row
                .split_whitespace()
                .map(|n| n.parse::<i32>().unwrap())
                .collect();
            for (y, v) in cols.iter().enumerate() {
                values[x][y] = *v;
            }
        }
        bingo_board {
            board: values,
            marked: [[false; 5]; 5],
            winner: false,
        }
    }

    fn check_board(&mut self, number: i32) {
        for (x, row) in self.marked.iter_mut().enumerate() {
            for (y, marked) in row.iter_mut().enumerate() {
                if self.board[x][y] == number {
                    *marked = true;
                }
            }
        }

        for i in 0..5 {
            if self.marked[i].iter().filter(|mark| **mark).count() == 5 {
                self.winner = true;
                break;
            }

            if self.marked.iter().filter(|&row| row[i]).count() == 5 {
                self.winner = true;
                break;
            }
        }
    }

    fn score(&self) -> i32 {
        let mut total_score: i32 = 0;
        for (x, row) in self.marked.iter().enumerate() {
            for (y, col) in row.iter().enumerate() {
                if !self.marked[x][y] {
                    total_score += self.board[x][y];
                }
            }
        }
        total_score
    }
}

fn main() {
    let data = fs::read_to_string("../../input.txt").unwrap();
    let input: Vec<&str> = data.lines().filter(|line| line != &"").collect();
    let numbers: Vec<i32> = input[0]
        .split(",")
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    let mut boards: Vec<bingo_board> = input[1..].chunks(5).map(|c| bingo_board::new(c)).collect();
    for number in numbers.iter() {
        for board in boards.iter_mut() {
            // println!("Checking: {}", number);
            board.check_board(*number);
            if board.winner {
                println!("{:?}", board);
                println!("{:?}", board.score() * number);
                return;
            }
        }
    }
}
