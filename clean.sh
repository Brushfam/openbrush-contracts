#!/bin/bash

echo "Starting to search for 'target' directories and running 'cargo clean'..."

# Find all 'target' directories in the current directory and its subdirectories
find . -type d -name target -print0 | while IFS= read -r -d '' dir; do
    # Go to the parent directory of each 'target' directory
    cd "$(dirname "$dir")"

    echo "Found 'target' directory in $(pwd). Running 'cargo clean'..."

    # Run 'cargo clean'
    cargo clean

    echo "'cargo clean' completed in $(pwd)."

    # Go back to the original directory (optional)
    cd -
done

echo "Finished running 'cargo clean' in all 'target' directories."
