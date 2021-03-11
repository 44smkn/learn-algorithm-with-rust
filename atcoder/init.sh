#!/bin/bash

DIRECTORY=$1
if [ -z $DIRECTORY ]; then
    echo "directory name is required."
    exit 1
fi

mkdir $DIRECTORY
cp template/main.rs $DIRECTORY
cat template/Makefile | sed -e "s/DIRECTORY/$DIRECTORY/g" > $DIRECTORY/Makefile
