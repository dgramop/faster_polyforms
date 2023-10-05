# first argument is the perc probability
# second argument is the size of the polyform to analyze (determines with directory it looks in)

mkdir -p ../../analysis/$1/$2;

POLYFORMS="../../percolation/$1/$2/*"

for p in $POLYFORMS
do
	echo $p
	./perseusMac ScubTop $p ../../analysis/$1/$2/`basename $p`.txt
	#../target/release/analysis --file $p > ../../analysis/$1/$2/`basename $p`.sa.txt
done
