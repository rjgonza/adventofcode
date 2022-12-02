#!/usr/bin/bash

SCRIPT_DIR="$( dirname -- "${BASH_SOURCE[0]}" )"
INPUT="${SCRIPT_DIR}/../input.txt"

infile="$INPUT"
maxnum=$(head -1 $infile | grep -o [0-9]* | wc -l)
boardnums=($(grep -v , $infile))
numboards=$((${#boardnums[@]}/25))
donecount=0

for numcalled in $(head -1 $infile | grep -o [0-9]*); do
    ptr=0
    while [[ $ptr -lt ${#boardnums[@]} ]]; do
        if [[ ${boardnums[$ptr]} -eq $(($maxnum+1)) ]]; then 
            ptr=$(($ptr+25))
            continue
        fi

        if [[ ${boardnums[$ptr]} -eq $numcalled ]]; then 
            bbase=$((($ptr/25)*25))
            lbase=$((($ptr/5)*5))
            roffset=$(($ptr%5))
            boardnums[$ptr]=$maxnum
            
            if [[ $((${boardnums[$lbase]}+${boardnums[$lbase+1]}+${boardnums[$lbase+2]}+${boardnums[$lbase+3]}+${boardnums[$lbase+4]})) -eq $(($maxnum*5)) ]]; then
                donecount=$(($donecount+1))
                if [[ $donecount -eq $numboards ]]; then
                    break 2
                fi
                
                boardnums[$bbase]=$(($maxnum+1))
            elif [[ $((${boardnums[$bbase+$roffset]}+${boardnums[$bbase+$roffset+5]}+${boardnums[$bbase+$roffset+10]}+${boardnums[$bbase+$roffset+15]}+${boardnums[$bbase+$roffset+20]})) -eq $(($maxnum*5)) ]]; then
                donecount=$(($donecount+1))
                if [[ $donecount -eq $numboards ]]; then
                    break 2
                fi
                
                boardnums[$bbase]=$(($maxnum+1))
            fi
            
            ptr=$(($bbase+25))
            
        else
            ptr=$(($ptr+1))
        fi
    done
done

sum=0
for num in `seq $bbase $(($bbase+24))`; do
    sum=$(($sum+(${boardnums[$num]}%$maxnum)))
done

echo "board start - $bbase, sum - $sum, last called - $numcalled, product $(($sum * $numcalled))";
