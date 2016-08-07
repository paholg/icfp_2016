#!/bin/sh

for problem in problems/*.txt; do
    echo; echo; echo "Running $problem"; echo; echo;
    timeout 5s cargo run --release $problem > log
done
