#!/usr/bin/env sh

cat template | sed "s/NN/$1/g" > src/bin/day$1.rs
