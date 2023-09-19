#!/bin/bash

if [ $# -eq 0 ]; then
  echo "Error: Please provide at least one glob pattern as an argument."
  exit 1
fi

IGNORED_DIRS=("./examples/reentrancy_guard"
  "./examples/test_helpers" 
  "./examples/diamond" 
  "./examples/alternatives" 
  "./examples/proxy" 
  "./examples" 
  "./examples/flash-borrower" 
  "./examples/psp22_extensions/flashmint/" 
  "./examples/psp22_extensions/wrapper/"
  "./examples/psp22_utils/token_timelock/")

ignore_dir() {
  local element
  for element in "${@:2}"; do
    [[ "$element" == "$1" ]] && return 0
  done
  return 1
}

process_directory() {
  local dir=$1

  if ignore_dir "$dir" "${IGNORED_DIRS[@]}"; then
    return
  fi

  if [ -f "${dir}/Cargo.toml" ]; then
    cd "$dir" || exit

    echo "Building contract in $dir"
    cargo contract build  --release || exit

    echo "Running e2e-tests in $dir"
    cargo test --features e2e-tests --release || exit

    cd - || exit
  else
    for inner in "$dir"/*/; do
      if [[ -d $inner ]]; then
        process_directory "$inner"
      fi
  done
  fi
}

for pattern in "$@"; do
  for dir in $pattern; do
    if [[ -d $dir ]]; then
      process_directory "$dir"
    fi
  done
done
