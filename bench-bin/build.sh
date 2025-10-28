#!/bin/bash
set -e

git submodule update --init --recursive

mkdir -p ../target/nanobench
g++ -O3 -Ideps/nanobench/src/include -Ideps/json/include src/main.cpp -o ../target/nanobench/bencher