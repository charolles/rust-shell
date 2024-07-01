#!/bin/bash

echo "Running my_shell..."
./target/release/rust-shell

if [ $? -ne 0 ]; then
    echo "Error occurred while running my_shell."
    exit 1
fi