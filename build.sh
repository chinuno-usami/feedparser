#! /bin/bash -x
g++ -std=c++17 ./test.cpp -Ltarget/release/ -lfeedparser
