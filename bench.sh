#!/bin/bash

# warm up cache:
cat domains.txt > /dev/null

echo ""
echo "Running Rust (psl) benchmark"
/usr/bin/time ./target/release/psl < domains.txt | wc -l

echo ""
echo "Running Rust (publicsuffix) benchmark"
/usr/bin/time ./target/release/publicsuffix < domains.txt | wc -l

echo ""
echo "Running C benchmark"
/usr/bin/time psl --load-psl-file public_suffix_list.dat --print-reg-domain < domains.txt | wc -l
