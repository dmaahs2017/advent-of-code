#!/usr/bin/env sh

set -e

cargo build --release
targets=$(ls src/bin | awk -F. '{print $1}')
for target in $targets
do
    ./target/release/$target
done
