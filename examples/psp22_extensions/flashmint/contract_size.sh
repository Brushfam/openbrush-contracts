#!/bin/bash

cargo contract build --release

NEW_SIZE=$(find target/ink -maxdepth 1 -type f -name "*.wasm" -printf "%s")

echo "NEW_SIZE | DIFFERENCE | MAX_SIZE | MIN_SIZE"



MX_MN=$(cat contract_size.txt)

MAX_SIZE=$(echo $MX_MN | cut -d' ' -f1)
MIN_SIZE=$(echo $MX_MN | cut -d' ' -f2)

if [[ $NEW_SIZE -gt $MAX_SIZE ]]; then
    MAX_SIZE=$NEW_SIZE
fi

if [[ $NEW_SIZE -lt $MIN_SIZE ]]; then
      MIN_SIZE=$NEW_SIZE
    elif [[ $MIN_SIZE -eq '' ]]; then
      MIN_SIZE=$NEW_SIZE
fi

Info=($MAX_SIZE $MIN_SIZE)

# shellcheck disable=SC2068
echo ${Info[@]} > contract_size.txt

DIFF=$(( $NEW_SIZE - $MIN_SIZE ))

echo "$NEW_SIZE | $DIFF | $MAX_SIZE | $MIN_SIZE"
