#!/bin/sh

CMAKE_PREFIX_PATH="~/temp/tools/Slint-cpp-1.13.1-Linux-x86_64"

mkdir _build
cmake -B _build -S . -DCMAKE_PREFIX_PATH="$CMAKE_PREFIX_PATH"
