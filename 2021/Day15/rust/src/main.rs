use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

static INPUT_FILE: &str = include_str!("../../input.txt");

fn main() {
    println!("Part 1: {}", part1(INPUT_FILE));
    println!("Part 2: {}", part2(INPUT_FILE));
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Node {
    position: (usize, usize),
    cost: usize,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn get_edges(node: Node, num_rows: usize, num_cols: usize) -> Vec<(usize, usize)> {
    let mut edges: Vec<(usize, usize)> = Vec::new();
    if node.position.0 > 0 {
        edges.push((node.position.0 - 1, node.position.1));
    }
    if node.position.1 > 0 {
        edges.push((node.position.0, node.position.1 - 1));
    }
    if node.position.0 < num_rows - 1 {
        edges.push((node.position.0 + 1, node.position.1));
    }
    if node.position.1 < num_cols - 1 {
        edges.push((node.position.0, node.position.1 + 1));
    }
    edges
}

fn part1(input: &str) -> usize {
    let map: Vec<Vec<usize>> = input
        .lines()
        .map(|line| {
            line.trim()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect();

    // dbg!(map.clone());
    // for r in map.clone() {
    //     for c in r {
    //         print!("{}", c);
    //     }
    //     println!();
    // }

    let num_rows = map.len();
    let num_cols = map[0].len();

    let mut total_risk_level = 0;
    let mut pq: BinaryHeap<Node> = BinaryHeap::new();
    pq.push(Node {
        position: (0, 1),
        cost: map[0][1],
    });
    pq.push(Node {
        position: (1, 0),
        cost: map[1][0],
    });

    let mut lowest_cost: HashMap<(usize, usize), usize> = HashMap::new();
    lowest_cost.insert((0, 0), 0);

    while let Some(node) = pq.pop() {
        if node.position == (num_rows - 1, num_cols - 1) {
            total_risk_level = node.cost;
            break;
        }

        if let Some(lowest_cost) = lowest_cost.get(&node.position) {
            if *lowest_cost <= node.cost {
                continue;
            }
        }

        for edge in get_edges(node, num_rows, num_cols) {
            let cost = node.cost + map[edge.0][edge.1];
            if let Some(lowest_cost) = lowest_cost.get(&edge) {
                if cost > *lowest_cost {
                    continue;
                }
            }

            pq.push(Node {
                position: edge,
                cost,
            });

            lowest_cost.insert(node.position, node.cost);
        }
    }

    total_risk_level
}

fn part2(input: &str) -> usize {
    let tile: Vec<Vec<usize>> = input
        .lines()
        .map(|line| {
            line.trim()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect();

    let orig_num_rows = tile.len();
    let orig_num_cols = tile[0].len();

    let num_rows = orig_num_rows * 5;
    let num_cols = orig_num_cols * 5;
    // dbg!(tile);

    // for r in tile.clone() {
    //     for c in r {
    //         print!("{}", c);
    //     }
    //     println!();
    // }

    let mut grid = vec![vec![0; 5 * num_cols]; 5 * num_rows];
    for tile_row in 0..5 {
        for tile_col in 0..5 {
            for i in 0..orig_num_rows {
                for j in 0..orig_num_cols {
                    let mut c = tile[i][j] + tile_row + tile_col;
                    if c > 9 {
                        c -= 9;
                    }
                    grid[tile_row as usize * orig_num_rows + i]
                        [tile_col as usize * orig_num_cols + j] = c;
                }
            }
        }
    }

    // for r in map.clone() {
    //     for c in r {
    //         print!("{}", c);
    //     }
    //     println!();
    // }

    // dbg!(map.clone());

    let mut total_risk_level = 0;
    let mut pq: BinaryHeap<Node> = BinaryHeap::new();
    pq.push(Node {
        position: (0, 1),
        cost: grid[0][1],
    });
    pq.push(Node {
        position: (1, 0),
        cost: grid[1][0],
    });

    let mut lowest_cost: HashMap<(usize, usize), usize> = HashMap::new();
    lowest_cost.insert((0, 0), 0);

    while let Some(node) = pq.pop() {
        if node.position == (num_rows - 1, num_cols - 1) {
            total_risk_level = node.cost;
            break;
        }

        if let Some(lowest_cost) = lowest_cost.get(&node.position) {
            if *lowest_cost <= node.cost {
                continue;
            }
        }

        for edge in get_edges(node, num_rows, num_cols) {
            let cost = node.cost + grid[edge.0][edge.1];
            if let Some(lowest_cost) = lowest_cost.get(&edge) {
                if *lowest_cost <= cost {
                    continue;
                }
            }

            pq.push(Node {
                position: edge,
                cost,
            });

            lowest_cost.insert(node.position, node.cost);
        }
    }

    total_risk_level
}

#[cfg(test)]

mod test {
    use super::*;
    const INPUT: &str = "1163751742
    1381373672
    2136511328
    3694931569
    7463417111
    1319128137
    1359912421
    3125421639
    1293138521
    2311944581";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 40);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 315);
    }
}
