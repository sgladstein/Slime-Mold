O=/tmp/claude-0/-home-user/436aa0b5-2e2a-5fcc-a830-4e57c114a955/scratchpad; F=$O/film
AT=$(seq -s, 4 4 120); AT1=$(seq -s, 5 5 150)
go() { $O/bin/$1 look=$F/$2 run=1:$3 hand=$4 ${5:+gun=rifle} at=$6 > $F/$2.txt 2>&1; }
go lw2 d1-base 1 pumps "" $AT1 & go lw3 d1-hot 1 pumps "" $AT1 & go lw2 d2-base 2 idle "" $AT & go lw3 d2-hot 2 idle "" $AT & wait
go lw2 d3-base 3 pumps "" $AT & go lw3 d3-hot 3 pumps "" $AT & go lw2 d4-base 4 stand r $AT & go lw3 d4-hot 4 stand r $AT & wait
go lw2 d5-base 5 stand r $AT & go lw3 d5-hot 5 stand r $AT & go lw2 d6-base 6 stand r $AT & go lw3 d6-hot 6 stand r $AT & wait
echo done > $F/done
