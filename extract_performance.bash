#!/usr/bin/env bash

# Exit if any command fails
set -e

# Check if at least two arguments are provided (one or more input files + output file)
if [ "$#" -lt 2 ]; then
  echo "Usage: $0 <input_file1> <input_file2> ... <output_file>"
  exit 1
fi

# Last argument is the output file
output_file="${@: -1}"

# Input files are all the arguments except the last one (output file)
input_files=("${@:1:$#-1}")

# Reset output file
echo "# Circuit Performance Comparison" > "$output_file"
echo "" >> "$output_file"

# Create the header row of the table
header="| Circuit Name         | Native Circuit Time | Native Witness Time "
divider="|----------------------|-----------------------|--------------------------"
for input_file in "${input_files[@]:1}"; do
  file_name=$(basename "$input_file")
  header+="| $file_name Circuit Time | $file_name Witness Time | Performance Loss (%) "
  divider+="|-----------------------|--------------------------|------------------------"
done
header+="|"
divider+="|"

echo "$header" >> "$output_file"
echo "$divider" >> "$output_file"

# Variables to track state
declare -A circuit_data

# Function to convert times to seconds (for comparison)
convert_to_seconds() {
  local time_str="$1"

  if [[ "$time_str" == *"ms" ]]; then
    echo "$(echo "$time_str" | sed 's/ms//')e-3" | bc -l
  elif [[ "$time_str" == *"µs" ]]; then
    echo "$(echo "$time_str" | sed 's/µs//')e-6" | bc -l
  else
    echo "$(echo "$time_str" | sed 's/m//; s/s//' | awk -F' ' '{print $1 * 60 + $2}')"
  fi
}

# Function to process each file and store the performance data
process_file() {
  local input_file="$1"
  local circuit_name=""
  local circuit_time=""
  local witness_time=""

  while IFS= read -r line; do
    # Detect circuit start
    if [[ "$line" == Running* ]]; then
      # If a circuit was already being processed, store the performance
      if [[ -n "$circuit_name" ]]; then
        echo "Processing circuit: $circuit_name with time: $circuit_time and witness: $witness_time"
        circuit_data["$circuit_name,$input_file"]="$circuit_time|$witness_time"
      fi

      # Extract circuit name (only the filename)
      circuit_name=$(basename "$(echo "$line" | awk '{print $2}')")
      circuit_time=""
      witness_time=""
    fi

    # Detect real time (for circuit creation)
    if [[ "$line" == real* ]] && [[ -z "$circuit_time" ]]; then
      circuit_time=$(echo "$line" | awk '{print $2}')
    fi

    # Detect witness generation time
    if [[ "$line" == *"Witness generated in:"* ]]; then
      witness_time=$(echo "$line" | awk '{print $4, $5}')
    fi
  done < "$input_file"

  # Store the last circuit's performance data
  if [[ -n "$circuit_name" ]]; then
    echo "Storing circuit: $circuit_name with time: $circuit_time and witness: $witness_time"
    circuit_data["$circuit_name,$input_file"]="$circuit_time|$witness_time"
  fi
}

# Process all input files
for input_file in "${input_files[@]}"; do
  process_file "$input_file"
done

# Collect all unique circuit names
unique_circuits=$(for key in "${!circuit_data[@]}"; do echo "$key" | cut -d',' -f1; done | sort | uniq)

# Build the rows of the table
for circuit_name in $unique_circuits; do
  row="| \`$circuit_name\` "

  # Add the native column times
  native_result="${circuit_data["$circuit_name,${input_files[0]}"]}"
  native_time=$(echo "$native_result" | cut -d'|' -f1)
  native_witness=$(echo "$native_result" | cut -d'|' -f2)
  row+="| $native_time | $native_witness |"
  echo "$row" >> "$output_file"
done
