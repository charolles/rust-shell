#!/bin/bash

echo "Running rust-shell..."
./target/release/rust-shell

if [ $? -ne 0 ]; then
    echo "Error occurred while running rust-shell."
    exit 1
fi