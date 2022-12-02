#!/usr/bin/bash

SCRIPT_DIR="$( dirname -- "${BASH_SOURCE[0]}" )"
INPUT="${SCRIPT_DIR}/../input.txt"

declare -A elf_calorie_count

elf_count=0

while read -r line; do
    [[ -z "$line" ]] && { (( elf_count++ )) ; continue ; }
    (( elf_calorie_count["elf_${elf_count}"]+="$line" ))
done < "$INPUT"

large_calories=0
for key in "${!elf_calorie_count[@]}"; do
    cals="${elf_calorie_count[$key]}"
    if [[ "$cals" -gt "${large_calories}" ]]; then
        echo "Largest: ${key} => ${elf_calorie_count[$key]}"
        large_calories_key="${key}"
        large_calories=${elf_calorie_count[$key]}
    fi
done

echo "Elf: $large_calories_key is carrying: ${elf_calorie_count[$large_calories_key]} calories"