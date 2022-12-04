#!/usr/bin/env sh

no_leading_zero=$(echo $1 | sed 's/^0*//')
cat template | sed -e "s/{NN}/$1/g" -e "s/{N}/$no_leading_zero/" > src/bin/day$1.rs;
mkdir inputs/day$1;
touch inputs/day$1/{input.txt,sample.txt};

curl -b "session=$(cat session.txt)" "https://adventofcode.com/2022/day/$no_leading_zero/input" > inputs/day$1/input.txt
