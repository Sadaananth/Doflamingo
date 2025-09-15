#!/bin/sh

export SLINT_BACKEND="winit-software"

CMAKE_PREFIX_PATH="~/tools/Slint-cpp-1.13.1-Linux-x86_64"

mkdir -p _build
cmake -B _build -S . -DCMAKE_PREFIX_PATH="$CMAKE_PREFIX_PATH"
