#!/bin/bash

# warm up cache:
cat domains.txt > /dev/null

echo "Running PyPy benchmark"
/usr/bin/time pypy main.py public_suffix_list.dat < domains.txt | wc -l
echo ""
echo "Running Rust benchmark"
/usr/bin/time ./target/release/pslbench < domains.txt | wc -l
echo ""
echo "Running C benchmark"
/usr/bin/time psl --load-psl-file public_suffix_list.dat --print-reg-domain < domains.txt | wc -l
