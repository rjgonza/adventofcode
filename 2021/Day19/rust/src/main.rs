use std::collections::{HashMap, HashSet};
use std::ops::{Add, Neg, Sub};

static INPUT_FILE: &str = include_str!("../../input.txt");
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Vec3 {
    x: i32,
    y: i32,
    z: i32,
}

impl Add for Vec3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Vec3 {
    fn zero() -> Self {
        Self { x: 0, y: 0, z: 0 }
    }

    fn rot(self, axis_rotations: Self) -> Self {
        let mut res = self;

        // x-axis rotation
        for _ in 0..axis_rotations.x {
            let prev_z = res.z;
            res.z = res.y;
            res.y = -prev_z;
        }

        // y-axis rotation
        for _ in 0..axis_rotations.y {
            let prev_z = res.z;
            res.z = -res.x;
            res.x = prev_z;
        }

        // z-axis rotation
        for _ in 0..axis_rotations.z {
            let prev_y = res.y;
            res.y = res.x;
            res.x = -prev_y;
        }

        res
    }
}

fn translate_points(src: &HashSet<Vec3>, translate: Vec3) -> HashSet<Vec3> {
    src.iter().map(|p| *p + translate).collect()
}

fn find_offset(a: &HashSet<Vec3>, b: &HashSet<Vec3>, min_similar: usize) -> Option<Vec3> {
    for a_origin in a.iter() {
        let a_translated = translate_points(a, -*a_origin);
        for b_origin in b.iter() {
            let b_translated = translate_points(b, -*b_origin);
            if a_translated.intersection(&b_translated).count() >= min_similar {
                return Some(*a_origin - *b_origin);
            }
        }
    }

    None
}

fn all_rotations() -> Vec<Vec3> {
    let mut res = Vec::new();
    let mut known_rots = HashSet::new();

    for xrot in 0..=3 {
        for yrot in 0..=3 {
            for zrot in 0..=3 {
                let rot = Vec3 {
                    x: xrot,
                    y: yrot,
                    z: zrot,
                };
                let axes_rotated = (
                    Vec3 { x: 1, y: 0, z: 0 }.rot(rot),
                    Vec3 { x: 0, y: 1, z: 0 }.rot(rot),
                    Vec3 { x: 0, y: 0, z: 1 }.rot(rot),
                );
                if known_rots.contains(&axes_rotated) {
                    continue;
                }

                known_rots.insert(axes_rotated);
                res.push(rot);
            }
        }
    }

    res
}

// fn parse_input(input: &str) -> Vec<Scanner> {
//     let mut scanners: Vec<Scanner> = Vec::new();
//     let all_rots = all_rotations();
//     let mut scanner_i = 0;
//     for line in input.lines() {
//         if line.trim().is_empty() {
//             continue;
//         }
//         if line.contains("---") {
//             scanner_i = line.split_whitespace().nth(2).unwrap().parse().unwrap();
//             // println!("Scanner! - {}", scanner);
//             scanners.push(Scanner {
//                 location: (Point(0, 0, 0)),
//                 // location: (Point(0, 0)),
//                 beacons: Vec::new(),
//             });
//             continue;
//         }

//         // dbg!(line);

//         let coord: Vec<isize> = line.trim().split(',').map(|c| c.parse().unwrap()).collect();
//         scanners[scanner_i].add_beacon(Beacon(Point(coord[0], coord[1], coord[2])));
//         // scanners[scanner_i].add_beacon(Beacon(Point(coord[0], coord[1])));
//     }
//     scanners
//     // dbg!(scanners);
// }

fn main() {
    println!("Part 1: {}", part1(INPUT_FILE));
    println!("Part 2: {}", part2(INPUT_FILE));
}

fn part1(input: &str) -> usize {
    let mut lines = input.lines().peekable();
    let mut scanners = Vec::new();
    let all_rots = all_rotations();

    while lines.peek().is_some() {
        // consume hyphen
        lines.next();

        let mut points = HashSet::new();
        loop {
            if lines.peek().is_none() {
                break;
            }

            let line = lines.next().unwrap();
            if line.is_empty() {
                break;
            }

            let coords: Vec<i32> = line.split(',').map(|toks| toks.parse().unwrap()).collect();
            points.insert(Vec3 {
                x: coords[0],
                y: coords[1],
                z: coords[2],
            });
        }

        // Insert the scanner's list of beacon positions, along with a cache of
        // pre-rotated positions. This allows us to avoid re-processing
        // rotations during brute-force search.
        let mut rots: HashMap<Vec3, HashSet<Vec3>> = HashMap::new();
        for r in all_rots.iter() {
            rots.insert(*r, points.iter().map(|p| p.rot(*r)).collect());
        }

        scanners.push(rots);
    }

    let min_overlap = 12;

    // `resolved` is a map between scanner IDs and a tuple of the scanner's
    // beacon positions, transformed to scanner 0's space.
    let mut resolved = HashMap::new();
    resolved.insert(0, scanners[0][&Vec3::zero()].clone());

    let mut unresolved = HashSet::new();
    for i in 1..scanners.len() {
        unresolved.insert(i);
    }

    let mut unrelated_scanners = HashSet::new();

    'outer: while !unresolved.is_empty() {
        for a_id in resolved.keys().cloned() {
            let a_points = &resolved[&a_id];

            for b_id in unresolved.clone() {
                if unrelated_scanners.contains(&(a_id, b_id)) {
                    continue;
                }

                for b_points in scanners[b_id].values() {
                    if let Some(b_to_a) = find_offset(a_points, b_points, min_overlap) {
                        unresolved.remove(&b_id);
                        resolved.insert(b_id, translate_points(b_points, b_to_a));
                        continue 'outer;
                    }
                }

                unrelated_scanners.insert((a_id, b_id));
            }
        }

        unreachable!();
    }

    let mut beacons = HashSet::new();
    for scanner_beacons in resolved.values() {
        beacons = beacons.union(scanner_beacons).copied().collect();
    }

    beacons.len()
}

fn part2(input: &str) -> usize {
    let mut lines = input.lines().peekable();
    let mut scanners = Vec::new();
    let all_rots = all_rotations();

    while lines.peek().is_some() {
        // consume hyphen
        lines.next();

        let mut points = HashSet::new();
        loop {
            if lines.peek().is_none() {
                break;
            }

            let line = lines.next().unwrap();
            if line.is_empty() {
                break;
            }

            let coords: Vec<i32> = line.split(',').map(|toks| toks.parse().unwrap()).collect();
            points.insert(Vec3 {
                x: coords[0],
                y: coords[1],
                z: coords[2],
            });
        }

        // Insert the scanner's list of beacon positions, along with a cache of
        // pre-rotated positions. This allows us to avoid re-processing
        // rotations during brute-force search.
        let mut rots: HashMap<Vec3, HashSet<Vec3>> = HashMap::new();
        for r in all_rots.iter() {
            rots.insert(*r, points.iter().map(|p| p.rot(*r)).collect());
        }

        scanners.push(rots);
    }

    let min_overlap = 12;

    // `resolved` is a map between scanner IDs and a tuple of the scanner's
    // beacon positions, transformed to scanner 0's space.
    let mut resolved = HashMap::new();
    resolved.insert(0, scanners[0][&Vec3::zero()].clone());
    let mut offsets = HashMap::new();
    offsets.insert(0, Vec3::zero());

    let mut unresolved = HashSet::new();
    for i in 1..scanners.len() {
        unresolved.insert(i);
    }

    let mut unrelated_scanners = HashSet::new();

    'outer: while !unresolved.is_empty() {
        for a_id in resolved.keys().cloned() {
            let a_points = &resolved[&a_id];

            for b_id in unresolved.clone() {
                if unrelated_scanners.contains(&(a_id, b_id)) {
                    continue;
                }

                for b_points in scanners[b_id].values() {
                    if let Some(b_to_a) = find_offset(a_points, b_points, min_overlap) {
                        unresolved.remove(&b_id);
                        resolved.insert(b_id, translate_points(b_points, b_to_a));
                        offsets.insert(b_id, b_to_a);
                        continue 'outer;
                    }
                }

                unrelated_scanners.insert((a_id, b_id));
            }
        }

        unreachable!();
    }

    let offsets: Vec<Vec3> = offsets.values().copied().collect();
    let mut best = 0;
    for i in 0..offsets.len() - 1 {
        for j in (i + 1)..offsets.len() {
            let d = offsets[j] - offsets[i];
            best = best.max(d.x.abs() + d.y.abs() + d.z.abs());
        }
    }

    best as usize
}

#[cfg(test)]

mod test {
    use super::*;

    static TEST_INPUT: &str = include_str!("../../test_input.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 79);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 3621);
    }
}
