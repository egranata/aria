#!/bin/bash
set -e

git submodule update --init --recursive

mkdir -p ../target/nanobench
g++ -O3 -Ideps/nanobench/src/include src/main.cpp -o ../target/nanobench/main