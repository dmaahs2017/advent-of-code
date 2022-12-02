#!/usr/bin/env sh

cat template | sed "s/NN/$1/g" > src/bin/day$1.rs;
mkdir inputs/day$1;
touch inputs/day$1/{input.txt,sample.txt};
