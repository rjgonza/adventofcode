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

d="$(date -d "${d:-$date}" +'%-d' )"

dir="${SCRIPT_DIR}/2021/Day${d}"
mkdir -p "${dir}/bash"
mkdir -p "${dir}/rust"

touch "${dir}/input.txt"
touch "${dir}/rust/Cargo.toml"

cat > "${dir}/rust/Cargo.toml" <<EOF
[package]
name = "rust"
version = "0.1.0"
edition = "2021"

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
echo "/home/ramon/code/adventofcode/2021/Day${d}/rust/Cargo.toml"