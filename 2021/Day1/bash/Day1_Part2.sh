#!/usr/bin/bash

SCRIPT_DIR="$( dirname -- "${BASH_SOURCE[0]}" )"
INPUT="${SCRIPT_DIR}/../input.txt"

count=0
increased=0
decreased=0
while read line; do
    if [[ "$count" -lt 3 ]]; then
        first="$line"
        result="N/A - no previous measurement"
    else
        sum=$(( first + second + line ))
        if [[ "$last_sum" -lt "$sum" ]]; then
            result="increased"
            (( increased++ ))
        else
            result="decreased"
            (( decreased++ ))
        fi
    fi

    last_sum="$sum"
    second="$first"
    first="$line"

    (( count++ ))

    echo "$sum ($result)"
    
done < "$INPUT"

echo "increased: $increased"
echo "decreaded: $decreased"