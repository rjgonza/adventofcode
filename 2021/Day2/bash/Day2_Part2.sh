#!/usr/bin/bash

SCRIPT_DIR="$( dirname -- "${BASH_SOURCE[0]}" )"
INPUT="${SCRIPT_DIR}/../input.txt"

x=0
y=0
z=0

while read direction distance; do
    case "$direction" in
        "forward")
            (( x += distance ))
            (( y += ( z * distance ) ))
            ;;
        "down")
            (( z += distance ))
            ;;
        "up") 
            (( z -= distance ))
            ;;
    esac
    
    echo "x: $x , y: $y, z: $z"
    
done < "$INPUT"
# done <<EOF
# forward 5
# down 5
# forward 8
# up 3
# down 8
# forward 2
# EOF

echo "Position: $(( x * y ))"