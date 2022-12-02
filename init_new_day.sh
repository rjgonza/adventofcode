#!/bin/bash

set -e

SCRIPT_DIR="$( dirname -- "${BASH_SOURCE[0]}" )"

function usage() { 
    cat <<EOF
    ${0}

    -d <date> (defaults to today)
EOF
    exit
}

while getopts "d:" opts; do
    case "${opts}" in
        d) date="${OPTARG}" ;;
        \?) usage ;;
    esac
done

d="$(date -d "${d:-$date}" +'%-d' )"
y="$(date -d "${d:-$date}" +'%-Y' )"

dir="${SCRIPT_DIR}/${y}/Day${d}"
mkdir -p "${dir}/bash"
mkdir -p "${dir}/rust/src"

touch "${dir}/input.txt"
touch "${dir}/rust/Cargo.toml"

cat > "${dir}/rust/src/main.rs" <<EOF
static INPUT_FILE: &str = include_str!("../../input.txt");

fn main() {
    println!("Part 1: {}", part1(INPUT_FILE));
    println!("Part 2: {}", part2(INPUT_FILE));
}

fn part1(_input: &str) -> usize {
    todo!();
}

fn part2(_input: &str) -> usize {
    todo!();
}

#[cfg(test)]

mod test {
    use super::*;
    const INPUT: &str = "";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 0);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 0);
    }
}

EOF

cat > "${dir}/rust/Cargo.toml" <<EOF
[package]
name = "rust"
version = "0.1.0"
edition = "2021"
authors = ["Ramon Gonzalez <ramon@gonzalez.house>"]

[dependancies]
EOF

cat >> "${SCRIPT_DIR}/README.md" <<EOF

### Day ${d}

[Puzzle](2021/Day${d}/Day${d}.md)

| Bash                                        | Rust                                        |
| ------------------------------------------- | ------------------------------------------- |
| [Part 1 & 2](2021/Day${d}/bash/day${d}.sh)  | [Part 1 & 2](2021/Day${d}/rust/src/main.rs) |
EOF

cat > "${dir}/Day${d}.md" <<EOF
[Back](../../README.md)
EOF

echo "Add this line to the VS Code config:"
echo "/home/ramon/code/adventofcode/${y}/Day${d}/rust/Cargo.toml"