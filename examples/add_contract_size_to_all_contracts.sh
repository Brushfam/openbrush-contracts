#!/bin/bash

ALL_PATHS=$(find . -maxdepth 3 -type f -name "Cargo.toml") 
INIT_PATH=$(pwd)

paths=()
for i in $ALL_PATHS
do
   temp_path=${i:0:(-10)}
   paths+=($temp_path)
done

len=${#paths[@]}

FILE=$(cat contract_size.sh)

echo $INIT_PATH

for (( i=0; i<$len; i++ ))
do
   path=${paths[$i]}
   cp contract_size.sh $path
   echo "$(( $(( $(( $(($i+1))*100 )) / $len )) ))% progress"
   cd $INIT_PATH
done
