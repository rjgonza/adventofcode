#!/usr/bin/bash

SCRIPT_DIR="$( dirname -- "${BASH_SOURCE[0]}" )"
INPUT="${SCRIPT_DIR}/../input.txt"


infile=$INPUT
maxnum=$(head -1 $infile | grep -o [0-9]* | wc -l)
boardnums=($(grep -v , $infile))
for numcalled in $(head -1 $infile | grep -o [0-9]*); do 
    ptr=0; 
    while [[ $ptr -lt ${#boardnums[@]} ]]; do 
        if [[ ${boardnums[$ptr]} -eq $numcalled ]]; then 
            bbase=$((($ptr/25)*25));lbase=$((($ptr/5)*5));roffset=$(($ptr%5)); 
            boardnums[$ptr]=$maxnum; 
            
            if [[ $((${boardnums[$lbase]}+${boardnums[$lbase+1]}+${boardnums[$lbase+2]}+${boardnums[$lbase+3]}+${boardnums[$lbase+4]})) -eq $(($maxnum*5)) ]]; then 
                break 2; 
            elif [[ $((${boardnums[$bbase+$roffset]}+${boardnums[$bbase+$roffset+5]}+${boardnums[$bbase+$roffset+10]}+${boardnums[$bbase+$roffset+15]}+${boardnums[$bbase+$roffset+20]})) -eq $(($maxnum*5)) ]]; then
                break 2; 
            fi 
        
            ptr=$(($bbase+25))
        else 
            ptr=$(($ptr+1))
        fi
    done
done

sum=0
for num in $(seq $bbase $(($bbase+24))); do
    sum=$(($sum+(${boardnums[$num]}%$maxnum)));
done
echo "board start - $bbase, sum - $sum, last called - $numcalled, product $(($sum * $numcalled))";

# line_counter=0
# next_end=6
# rand_numbers=""
# declare -A boards
# while read line; do
#     if [[ "${line}" == "" ]]; then
#         continue
#     elif [[ $line_counter -eq 0 ]]; then
#         rand_numbers="${line}"
#     elif [[ "$line_counter" -le "${next_end}" ]]; then
#         index=$(( line_counter - ( 1 * line_counter ) ))
#         echo "Inserting $line on $index"
#         boards[$index]+="$line"
#     fi

#     if [[ "$line_counter" -eq "${next_end}" ]]; then
#         (( next_end+=5))
#     fi

#     (( line_counter++ ))

# done <<EOF
# 7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

# 22 13 17 11  0
#  8  2 23  4 24
# 21  9 14 16  7
#  6 10  3 18  5
#  1 12 20 15 19

#  3 15  0  2 22
#  9 18 13 17  5
# 19  8  7 25 23
# 20 11 10 24  4
# 14 21 16 12  6

# 14 21 17 24  4
# 10 16 15  9 19
# 18  8 23 26 20
# 22 11 13  6  5
#  2  0 12  3  7
# EOF

# #< "${INPUT}"


# echo $rand_numbers
# echo ${#boards[@]}
# for board in "${!boards[@]}"; do
#     echo ${boards[$board]}
# done
