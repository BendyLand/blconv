#!/bin/bash

echo "// utils.rs" > summary.txt
cat src/utils.rs >> summary.txt
echo "" >> summary.txt
echo "// image.rs" >> summary.txt
cat src/image.rs >> summary.txt
echo "" >> summary.txt
echo "// main.rs" >> summary.txt
cat src/main.rs >> summary.txt
