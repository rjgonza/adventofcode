#!/usr/bin/bash

SCRIPT_DIR="$( dirname -- "${BASH_SOURCE[0]}" )"
INPUT="${SCRIPT_DIR}/../input.txt"
         # 0 1 2 3 4 5 6 7 8 9 10 11
counter_0=(0 0 0 0 0 0 0 0 0 0 0 0)
counter_1=(0 0 0 0 0 0 0 0 0 0 0 0)

while read line; do
    for number in "${line}"; do
        # echo "$number"
        for (( i=0; i<"${#number}"; i++ )); do
    digit=${number:i:1}
    case "$digit" in
        "0") (( counter_0[i]++ )) ;;
        "1") (( counter_1[i]++ )) ;;
    esac
done
    done
done < "${INPUT}"

gamma_rate=""
epsilon_rate=""

for i in {0..11}; do
    if [[ ${counter_1[i]} -gt ${counter_0[i]} ]]; then
        gamma_rate="${gamma_rate}1"
        epsilon_rate="${epsilon_rate}0"
    else
        gamma_rate="${gamma_rate}0"
        epsilon_rate="${epsilon_rate}1"
    fi
done

gamma_rate_demical=$(echo "ibase=2; $gamma_rate" | bc )
epsilon_rate_demical=$(echo "ibase=2; $epsilon_rate" | bc )

power_consumption=$(( gamma_rate_demical * epsilon_rate_demical ))
echo "Gamma Rate (binary): $gamma_rate | Gamma Rate (decimal): $gamma_rate_demical"
echo "Epsilon Rate (binary): $epsilon_rate | Gamma Rate (decimal): $epsilon_rate_demical"
echo "Power Consumption: $power_consumption"