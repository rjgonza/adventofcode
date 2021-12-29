static INPUT_FILE: &str = include_str!("../../input.txt");

#[derive(Debug, Clone, PartialEq)]
struct VecTree {
    values: Vec<u32>,
    depths: Vec<u32>,
}

impl VecTree {
    fn parse(s: &str) -> VecTree {
        let mut t = VecTree {
            values: Vec::new(),
            depths: Vec::new(),
        };

        let mut depth = 0;
        for c in s.chars() {
            match c {
                '[' => {
                    depth += 1;
                }
                ',' => (),
                ']' => {
                    depth -= 1;
                }
                d => {
                    t.values.push(d.to_digit(10).unwrap());
                    t.depths.push(depth - 1);
                }
            }
        }

        t
    }

    fn explode(&mut self) -> bool {
        for i in 0..self.depths.len() {
            let depth = self.depths[i];
            if depth != 4 {
                continue;
            }

            // add left value to left neighbor
            if i != 0 {
                self.values[i - 1] += self.values[i];
            }

            // add right value to right neighbor
            if i + 2 < self.values.len() {
                self.values[i + 2] += self.values[i + 1];
            }

            self.values[i] = 0;
            self.depths[i] = 3;
            self.values.remove(i + 1);
            self.depths.remove(i + 1);

            return true;
        }

        false
    }

    fn split(&mut self) -> bool {
        for i in 0..self.values.len() {
            let v = self.values[i];
            if v < 10 {
                continue;
            }

            let (a, b) = if v % 2 == 0 {
                (v / 2, v / 2)
            } else {
                (v / 2, v / 2 + 1)
            };

            self.values[i] = a;
            self.depths[i] += 1;
            self.values.insert(i + 1, b);
            self.depths.insert(i + 1, self.depths[i]);

            return true;
        }

        false
    }

    fn reduce(&mut self) {
        loop {
            if !self.explode() && !self.split() {
                break;
            }
        }
    }

    fn add(&mut self, other: &VecTree) {
        self.values.extend(other.values.iter());
        self.depths.extend(other.depths.iter());
        for i in 0..self.depths.len() {
            self.depths[i] += 1;
        }
    }

    fn magnitude(&self) -> u32 {
        let mut vals = self.values.clone();
        let mut depths = self.depths.clone();

        while vals.len() > 1 {
            for i in 0..depths.len() - 1 {
                if depths[i] == depths[i + 1] {
                    vals[i] = 3 * vals[i] + 2 * vals[i + 1];
                    vals.remove(i + 1);
                    depths.remove(i + 1);

                    if depths[i] > 0 {
                        depths[i] -= 1;
                    }

                    break;
                }
            }
        }

        vals[0]
    }
}

fn part1(input: &str) -> u32 {
    let mut lines = input.lines();
    let mut tree = VecTree::parse(lines.next().unwrap());
    // dbg!(tree.clone());
    for line in lines {
        // dbg!(line.trim());
        tree.add(&VecTree::parse(line.trim()));
        tree.reduce();
    }

    tree.magnitude()
}

fn part2(input: &str) -> u32 {
    let trees: Vec<VecTree> = input.lines().map(|l| VecTree::parse(l.trim())).collect();
    // dbg!(trees.clone());

    let mut best_score = 0;
    // 'outer: for a in trees.iter() {
    for a in trees.iter() {
        for b in trees.iter() {
            if a == b {
                continue;
            }
            let mut a = a.clone();
            // let orig_a = a.clone();
            // let orig_b = b.clone();
            a.add(b);
            a.reduce();
            best_score = best_score.max(a.magnitude());
            // if best_score == 4667 {
            //TODO: Debug why we are getting 4700 vs 4667...
            // if best_score == 4700 {
            //     dbg!(orig_a);
            //     dbg!(orig_b);
            //     // dbg!(b.clone());
            //     break 'outer;
            // }
        }
    }

    best_score
}

fn main() {
    println!("Part 1: {}", part1(INPUT_FILE));
    println!("Part 2: {}", part2(INPUT_FILE));

    // [[[[4,9],0],[[4,4],9]],[[[6,1],[8,9]],[7,[2,3]]]]
    // [[[[4,9],0],[[4,4],9]],[[[6,1],[8,9]],[7,[2,3]]]]

    // let left = "[[[[8,8],8],[[3,9],[9,3]]],[[8,8],[[7,1],[6,5]]]]";
    // let right = "[[[0,0],[[1,9],[0,6]]],[[5,[8,8]],[[6,9],[3,7]]]]";
    // let mut l_tree: VecTree = VecTree::parse(left);
    // let r_tree: VecTree = VecTree::parse(right);
    // // dbg!(l_tree.clone());
    // // dbg!(r_tree.clone());
    // l_tree.add(&r_tree);
    // l_tree.reduce();
    // dbg!(l_tree.magnitude());
}

#[cfg(test)]

mod test {
    use super::*;
    const INPUT: &str = "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
    [[[5,[2,8]],4],[5,[[9,9],0]]]
    [6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
    [[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
    [[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
    [[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
    [[[[5,4],[7,7]],8],[[8,3],8]]
    [[9,3],[[9,9],[6,[4,9]]]]
    [[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
    [[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 4140);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 3993);
    }

    // fn test_wtf() {

    // }
}
