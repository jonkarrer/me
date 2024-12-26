#!/bin/bash

if [ - "$1" ]; then
    echo "Usage: $0 <secrets.json>"
    exit 1
fi

input_file="$1"
output_file="Secrets.toml"

echo "Converting $input_file to $output_file"

echo "Writing TOML formatt..."

jq -r 'to_entries | map("\(.key) = \(.value | @json)") | .[]' "$input_file" > "$output_file"

echo "Conversion complete"