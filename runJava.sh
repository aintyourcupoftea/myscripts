#!/bin/bash

# Check if a filename is provided
if [ "$#" -ne 1 ]; then
    echo "Usage: $0 <JavaFileName>"
    exit 1
fi

# Get the filename without the extension
filename=$(basename "$1" .java)

# Compile the Java file
javac "$1"

# Check if compilation was successful
if [ $? -eq 0 ]; then
    # Run the compiled class
    java "$filename"
    
    # Delete the .class file
    rm "$filename.class"
else
    echo "Compilation failed."
fi
