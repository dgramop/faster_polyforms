# first argument is the size of the polyform to analyze (determines with directory it looks in)
# second argument is the perc probability

mkdir -p ../../analysis/$1/$2;

POLYFORMS="../../results/$1/$2/*"

for p in $POLYFORMS
do
	echo $p
	#./perseusLin ScubTop $p ../../analysis/$1/$2/`basename $p`.txt
	../target/release/analysis --file $p > ../../analysis/$1/$2/`basename $p`.sa.txt
done
