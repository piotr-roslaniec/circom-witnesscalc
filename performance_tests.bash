#!/usr/bin/env bash

# Exit if any command fails
set -e

# Check if there are uncommitted changes in the repository
if [ -n "$(git status --porcelain)" ]; then
  echo "Error: Your Git repository has uncommitted changes."
  echo "Please commit or stash them before running this script."
  exit 1
fi

# Paths for storing test results
native_result_path="test_results/native.txt"
wasm_result_path="test_results/wasm_nodejs.txt"
output_file="performance_comparison.md"

# Check if test_circuits.sh and build-wasm.bash exist
if [ ! -f "test_circuits.sh" ]; then
  echo "Error: test_circuits.sh not found!"
  exit 1
fi

if [ ! -f "build-wasm.bash" ]; then
  echo "Error: build-wasm.bash not found!"
  exit 1
fi

# Function to run tests and stash results for each branch
run_tests() {
  local branch="$1"
  local result_file="$2"

  echo "Switching to branch: $branch"

  # Stash any uncommitted changes
  git stash --quiet

  # Switch to the specified branch
  git checkout "$branch" --quiet

  # Run the tests and store the output in the result file
  if [ "$branch" == "main" ]; then
    echo "Running tests on branch: $branch"
    bash test_circuits.sh 2>&1 | tee "$result_file"
  elif [ "$branch" == "wasm" ]; then
    echo "Building and running tests on branch: $branch"
    bash build-wasm.bash && bash test_circuits.sh 2>&1 | tee "$result_file"
  fi

  # Apply stashed changes (if any)
  git stash pop --quiet || true
}

# Reproduce original, native results
run_tests "main" "$native_result_path"

# Reproduce WASM, Node.js results
run_tests "wasm" "$wasm_result_path"

# Parse the results using the original script (extract_performance.sh)
echo "Generating performance comparison report..."
./extract_performance.bash "$native_result_path" "$wasm_result_path" "$output_file"

echo "Performance comparison report saved to: $output_file"
