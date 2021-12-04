#!/bin/bash

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

d="$(date -d "${d:-$(date)}" +'%-d' )"

dir="${SCRIPT_DIR}/2021/Day${d}"
mkdir -p "${dir}/bash"
mkdir -p "${dir}/rust"

touch "${dir}/input.txt"
touch "${dir}/rust/Cargo.toml"

cat > "${dir}/rust/Cargo.toml" <<EOF
[workspace]

members = [
    "day${d}_part1",
    "day${d}_part2",
]
EOF

cat >> "${SCRIPT_DIR}/README.md" <<EOF

### Day ${d}

[Puzzle](2021/Day${d}/Day${d}.md)

| Bash                                        | Rust                                            |
| ------------------------------------------- | ----------------------------------------------- |
| [Part 1](2021/Day${d}/bash/Day${d}_Part1.sh)      | [Part 1](2021/Day${d}/rust/day${d}_part1/src/main.rs) |
| [Part 1](2021/Day${d}/bash/Day${d}_Part1.sh)      | [Part 2](2021/Day${d}/rust/day${d}_part1/src/main.rs) |
EOF

cat > "${dir}/Day${d}.md" <<EOF
[Back](../../README.md)
EOF

echo "Add this line to the VS Code config:"
echo "/home/ramon/code/adventofcode/2021/Day${d}/rust/Cargo.toml"