#!/usr/bin/bash

SCRIPT_DIR="$( dirname -- "${BASH_SOURCE[0]}" )"
INPUT="${SCRIPT_DIR}/../input.txt"

declare -A elf_calorie_count

elf_count=0

while read -r line; do
    [[ -z "$line" ]] && { (( elf_count++ )) ; continue ; }
    (( elf_calorie_count["elf_${elf_count}"]+="$line" ))
done < "$INPUT"

large_calories1=0
large_calories2=0
large_calories3=0

for key in "${!elf_calorie_count[@]}"; do
    cals="${elf_calorie_count[$key]}"
    if [[ "$cals" -gt "${large_calories1}" ]]; then
        # echo "Largest: ${key} => ${elf_calorie_count[$key]}"

        large_calories3="$large_calories2"
        large_calories2="$large_calories1"
        large_calories1=${elf_calorie_count[$key]}

        large_calories3_key="${large_calories2_key}"
        large_calories2_key="${large_calories1_key}"
        large_calories1_key="${key}"

    elif [[ "$cals" -gt "${large_calories2}" ]]; then
        # echo "Second Largest: ${key} => ${elf_calorie_count[$key]}"

        large_calories3="$large_calories2"
        large_calories2="${elf_calorie_count[$key]}"
        
        large_calories3_key="${large_calories2_key}"
        large_calories2_key="${key}"

    elif [[ "$cals" -gt "${large_calories3}" ]]; then
        # echo "Third Largest: ${key} => ${elf_calorie_count[$key]}"

        large_calories3="${elf_calorie_count[$key]}"
        
        large_calories3_key="${key}"
    fi
done

echo "Elf: $large_calories1_key is carrying: ${elf_calorie_count[$large_calories1_key]} calories"
echo "Elf: $large_calories2_key is carrying: ${elf_calorie_count[$large_calories2_key]} calories"
echo "Elf: $large_calories3_key is carrying: ${elf_calorie_count[$large_calories3_key]} calories"
echo "Total: $(( large_calories1 + large_calories2 + large_calories3 ))"