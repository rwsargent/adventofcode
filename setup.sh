#!/bin/sh
cp template.py $1.py
mkdir tests/$1
touch tests/$1/test.txt
touch input/$1_input.txt
