#!/bin/bash

ALL_PATHS=$(find . -maxdepth 3 -type f -name "contract_size.txt") 
INIT_PATH=$(pwd)

paths=()
for i in $ALL_PATHS
do
   temp_path=${i:0:(-17)}
   paths+=($temp_path)
done

len=${#paths[@]}

echo $INIT_PATH

for (( i=0; i<$len; i++ ))
do
   path=${paths[$i]}
   cd $path
   
   MX_MN=$(cat contract_size.txt)
   MAX_SIZE=$(echo $MX_MN | cut -d' ' -f1)
   MIN_SIZE=$(echo $MX_MN | cut -d' ' -f2)
   
   echo $path "=> SIZE = $MIN_SIZE"
   
   cd $INIT_PATH
done
