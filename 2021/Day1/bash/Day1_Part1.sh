#!/usr/bin/bash

SCRIPT_DIR="$( dirname -- "${BASH_SOURCE[0]}" )"
INPUT="${SCRIPT_DIR}/../input.txt"

count=0
increased=0
decreased=0
while read line; do
    if [[ "$count" -eq 0 ]]; then
        last="$line"
        result="N/A - no previous measurement"
    else    
        if [[ "$last" -lt "$line" ]]; then
            result="increased"
            (( increased++ ))
        else
            result="decreased"
            (( decreased++ ))
        fi
    fi

    last="$line"
    (( count++ ))

    echo "$line ($result)"
    
done < "$INPUT"

echo "increased: $increased"
echo "decreaded: $decreased"