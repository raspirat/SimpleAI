#!/bin/bash

qtFolder=$1
qtVersion=$2

realpath $qtFolder 2> /dev/null
if [ $? -eq 127 ]; then
    echo "Please specify the folder of your Qt-installation"
    exit -1
fi

realpath $qtVersion 2> /dev/null
if [ $? -eq 127 ]; then
    echo "Please specify the version of your Qt-installation"
    exit -1
fi

if [ ! -d "build" ]; then
	mkdir build
fi

cmake -B build/release -D CMAKE_PREFIX_PATH=$qtFolder/$qtVersion/gcc_64/lib/cmake/Qt6
cmake --build 'build' --target all

./build/appSimpleAI

