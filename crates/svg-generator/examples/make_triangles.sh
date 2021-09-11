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

# Generate files with specific counts of triangles.
#
counts=(1 10 50 100)
for c in "${counts[@]}"
do
    file_path="$output_dir/triangles-$c.svg"
    echo -n "Creating $c triangles..."
    $binary_path -c $c triangle "$file_path"
    echo "Done"
done
echo "All files saved to '$output_dir'."