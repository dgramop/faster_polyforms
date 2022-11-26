#!/bin/bash

# first argument is the number elements
# second argument is the number of shuffles
# third argument is the percolation probability
# fourth argument is the number of polyforms
# fifth argument is the start offset

cargo build --bin main --release
mkdir -p ../percolation/$1;

for ((  i=$5; i<=$5+$4; i++ ))
do
	echo $i
	../target/release/main --norender --length $1 --shuffles $2 --bernoulli $3 --export analysis > ../percolation/$1/$i.txt
done
