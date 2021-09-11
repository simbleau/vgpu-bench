#!/bin/bash

# Check if the executable has been built.
binary_path="../../../target/debug/svg-generator"
if [ ! -f $binary_path ]; then
    echo "Please run \`cargo build\` first."
    exit 1
fi

# Stage area for output
output_dir="output/"
if [ ! -d $output_dir ]; then
    mkdir $output_dir
fi

# Generate files with specific counts of lines.
#
primitives=("line" "triangle" "polygon" "curve" "cubic-curve" "bezigon" "cubic-bezigon")
counts=(1 10 50 100 500 1000)
for p in "${primitives[@]}"
do
    for c in "${counts[@]}"
    do
        file_path="$output_dir/$p-$c.svg"
        echo -n "Creating $c of primitive: $p..."
        $binary_path -r -c=$c $p "$file_path"
        echo "Done"
    done
done
echo "All files saved to '$output_dir'."