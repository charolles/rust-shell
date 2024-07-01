#!/bin/bash

echo "Building rust-shell..."
cargo build --release

if [ $? -eq 0 ]; then
    echo "Build successful!"
else
    echo "Build failed."
    exit 1
fi