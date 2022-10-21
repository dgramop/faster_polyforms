# first argument is the number elements
# second argument is the number of shuffles
# third argument is the number of polyforms
# fourth argument is the prefix for where to place these files

for (( i=0; i<=$3; i++ ))
do
	echo $i
	./target/release/main --norender --length $1 --shuffles $2 --export analysis > $4/$i.txt
done
