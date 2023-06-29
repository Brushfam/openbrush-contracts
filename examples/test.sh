#!/bin/bash

echo "Starting to search for directories containing 'Cargo.toml' and running specified cargo commands..."

# Create an empty array to hold the list of failing directories
declare -a failed_dirs=()

# Find all directories containing 'Cargo.toml' in the current directory and its subdirectories
find . -name "Cargo.toml" -print0 | while IFS= read -r -d '' file; do
    # Go to the directory of each 'Cargo.toml' file
    dir="$(dirname "$file")"
    cd "$dir"

    echo "Found 'Cargo.toml' in $(pwd). Running specified cargo commands..."

    # Run 'cargo contract build'
    cargo contract build
    if [[ $? -ne 0 ]]; then
        for i in {1..10}; do echo -e "\a"; done
        echo "Error occurred in 'cargo contract build'."
        failed_dirs+=("$(pwd)") # Add the directory to the list of failing directories
    fi

    # Run 'cargo test --feature e2e-tests'
    cargo test --features e2e-tests
    if [[ $? -ne 0 ]]; then
        for i in {1..10}; do echo -e "\a"; done
        echo "Error occurred in 'cargo test --features e2e-tests'."
    fi

    # Run 'cargo clean'
    cargo clean
    if [[ $? -ne 0 ]]; then
        for i in {1..10}; do echo -e "\a"; done
        echo "Error occurred in 'cargo clean'."
    fi

    echo "Commands completed in $(pwd)."

    # Go back to the original directory (optional)
    cd -
done

# Print the summary
if [ ${#failed_dirs[@]} -eq 0 ]; then
    echo "No failures occurred during the 'cargo contract build' commands."
else
    echo "The 'cargo contract build' command failed in the following directories:"
    for dir in "${failed_dirs[@]}"; do
        echo "- $dir"
    done
fi

echo "Finished running specified cargo commands in all 'Cargo.toml' directories."
