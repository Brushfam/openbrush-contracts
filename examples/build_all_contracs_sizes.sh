#!/bin/bash

ALL_PATHS=$(find . -maxdepth 3 -type f -name "contract_size.sh") 
INIT_PATH=$(pwd)

paths=()
for i in $ALL_PATHS
do
   temp_path=${i:0:(-16)}
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
   BUILDED=$(echo $MX_MN | cut -d' ' -f3)
   
#   if [ $MIN_SIZE -eq "100000" ] || [ $MAX_SIZE -eq "0" ]
#   then
#   	echo "Needs to build"
   	echo "Now bilding in $path"   	
        bash contract_size.sh
        rm -r target
#  fi
   echo "$(( $(( $(( $(($i+1))*100 )) / $len )) ))% progress"
   cd $INIT_PATH
done
