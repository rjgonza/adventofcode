#!/usr/bin/bash

SCRIPT_DIR="$( dirname -- "${BASH_SOURCE[0]}" )"
INPUT="${SCRIPT_DIR}/../input.txt"

o2_numbers=()
co2_numbers=()

while read line; do
    # echo "$line"
    o2_numbers+=( "${line}" )
    co2_numbers+=( "${line}" )
done < "${INPUT}"
# echo "${o2_numbers[@]}"

o2_number_found="false"
co2_number_found="false"
while [[ ! ($o2_number_found == "true" && $co2_number_found == "true") ]]; do
    for i in {0..11}; do
        counter_0=0
        counter_1=0
        for o2 in "${o2_numbers[@]}"; do
            digit=${o2:i:1}
            [[ $digit -eq 1 ]] && (( counter_1++ ))
            [[ $digit -eq 0 ]] && (( counter_0++ ))
        done

        o2_temp=()
        for o2 in "${o2_numbers[@]}"; do
            # echo "Checking O2: $o2"
            digit=${o2:i:1}
            if [[ counter_1 -ge counter_0 ]]; then
                [[ $digit -eq 1 ]] && o2_temp+=( "$o2" )
            else
                [[ $digit -eq 0 ]] && o2_temp+=( "$o2" )
            fi
        done

        [[ ${#o2_numbers[@]} -eq 1 ]] && { o2_number_found="true"; break; }

        o2_numbers=( "${o2_temp[@]}" )
    done

    for i in {0..11}; do
        counter_0=0
        counter_1=0
        for co2 in "${co2_numbers[@]}"; do
            digit=${co2:i:1}
            [[ $digit -eq 1 ]] && (( counter_1++ ))
            [[ $digit -eq 0 ]] && (( counter_0++ ))
        done
        
        co2_temp=()
        for co2 in "${co2_numbers[@]}"; do
            # echo "Checking CO2: $co2"
            digit=${co2:i:1}
            if [[ counter_1 -ge counter_0 ]]; then
                [[ $digit -eq 0 ]] && co2_temp+=( "$co2" )
            else
                [[ $digit -eq 1 ]] && co2_temp+=( "$co2" )
            fi
        done
        
        [[ ${#co2_numbers[@]} -eq 1 ]] && { co2_number_found="true"; break; }
        
        co2_numbers=( "${co2_temp[@]}" )
    done
done

echo $o2_numbers
echo $co2_numbers

oxygen_generator_rating=$(echo "ibase=2; $o2_numbers" | bc )
co2_scrubber_rating=$(echo "ibase=2; $co2_numbers" | bc )

life_support_rating=$(( oxygen_generator_rating * co2_scrubber_rating ))
echo "O2 Rating (binary): $o2_numbers | O2 Rating (decimal): $oxygen_generator_rating"
echo "CO2 Rating (binary): $co2_numbers | CO2 Rating (decimal): $co2_scrubber_rating"
echo 
echo "Power Consumption: $life_support_rating"