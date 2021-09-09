# Check if the executable has been built.
binary_path="../../../target/debug/svg-generator"
if [ ! -f $binary_path ]; then
    echo "Please run \`cargo build\` first."
    exit 1
fi

# Get the source for an SVG file with 1 line.
#
# Verbose: On
# Rotation: Off (default)
# Count: 1 (default)
# Primitive: Line
exec $binary_path -v line