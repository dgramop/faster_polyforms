# first argument is the size of the polyform to analyze (determines with directory it looks in)
# second argument is the number of polyforms to analyze

mkdir -p ../analysis/$1;

for (( i=0; i<=$2; i++ ))
do
	echo $i
	./perseusMac ScubTop ../polyforms/$1/$i.txt ../analysis/$1/$i.txt
done
