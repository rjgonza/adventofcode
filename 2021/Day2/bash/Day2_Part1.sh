#!/usr/bin/bash

SCRIPT_DIR="$( dirname -- "${BASH_SOURCE[0]}" )"
INPUT="${SCRIPT_DIR}/../input.txt"

x=0
y=0

while read direction distance; do
    case "$direction" in
        "forward") (( x += distance )) ;;
        "down") (( y += distance )) ;;
        "up") (( y -= distance )) ;;
    esac
    
    echo "x: $x , y: $y"
    
done < "$INPUT"

echo "Position: $(( x * y ))"