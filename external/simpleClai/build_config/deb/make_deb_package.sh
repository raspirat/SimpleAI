#!/bin/bash

# Make sure to qmake and make it first.

buildDir=$1
sourceDir=$2
appName=$3
packagePath=$4

realpath $buildDir 2> /dev/null
if [ $? -eq 127 ]; then
    $buildDir=$(realpath $1)
fi

realpath $sourceDir 2> /dev/null
if [ $? -eq 127 ]; then
    $sourceDir=$(realpath $2)
fi

realpath $packagePath 2> /dev/null
if [ $? -eq 127 ]; then
    $packagePath=$(realpath $4)
fi

if [[ -z $1 ]]; then
    echo "No arguments given: 1:buildDir not given, 2:sourceDir not given, 3:appName not given. 4:packagePath not required. exiting..."
    exit -1
elif [[ -z $2 ]]; then
    echo "To few arguments: 1: $buildDir, 2:sourceDir not given, 3:appName not given 4:packagePath not required. exiting..."
    exit -1
elif [[ -z $3 ]]; then
    echo "To few arguments: 1: $buildDir, 2:$sourceDir, 3:appName not given 4:packagePath not required. exiting..."
    exit -1
elif [[ -z $4 ]]; then
    echo "packagePath not set. Using ./package"
    packagePath="package"
fi

echo "buildDir:$buildDir, sourceDir:$sourceDir, appName:$appName"

builtAppPath="$buildDir/$appName"
destAppPath="$packagePath/$appName/usr/bin"
destScriptPath="$packagePath/$appName/etc/$appName/scripts"
destConfigPath="$packagePath/$appName/etc/$appName/config"
buidConfigPath="$packagePath/$appName/DEBIAN"

if [[ ! -f $builtAppPath ]]; then
    echo "App not yet built. exiting..."
    exit 0
fi

if [[ ! -d  $destAppPath ]]; then
    echo "creating " $destAppPath
    mkdir -p $destAppPath
fi

if [[ ! -d  $destScriptPath ]]; then
    echo "creating " $destScriptPath
    mkdir -p $destScriptPath
fi

if [[ ! -d  $destConfigPath ]]; then
    echo "creating " $destConfigPath
    mkdir -p $destConfigPath
fi

if [[ ! -d  $buidConfigPath ]]; then
    echo "creating " $buidConfigPath
    mkdir -p $buidConfigPath
fi

# rm $buildDir/*.o $buildDir/Makefile

mv $builtAppPath $destAppPath

cp -f $sourceDir/scripts/* $destScriptPath
cp -f $sourceDir/config/* $destConfigPath
cp -r -f $sourceDir/build_config/deb/DEBIAN/* $buidConfigPath

dpkg-deb --build "$packagePath/$appName"
