#!/bin/bash

# first argument is the number elements
# second argument is the number of shuffles
# third argument is the number of polyforms
# fourth argument is the start offset

cargo build --bin main --release
mkdir -p ../polyforms/$1;

for ((  i=$4; i<=$4+$3; i++ ))
do
	echo $i
	../target/release/main --norender --length $1 --shuffles $2 --export analysis > ../polyforms/$1/$i.txt
done
