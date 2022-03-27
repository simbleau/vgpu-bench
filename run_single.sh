rm -rf /home/simbleau/git/vgpu-bench/output*
mkdir /home/simbleau/git/vgpu-bench/output_cache

cd /home/simbleau/git/vgpu-bench
cargo build --release

filename=$1
name=$2
./target/release/plotting_pathfinder "$filename"
python3 src/models/plotting/py/numeric_line_single.py "Pathfinder" "$2"
mv output output_cache/pathfinder

./target/release/plotting_renderkit
python3 src/models/plotting/py/numeric_line_single.py "Render-Kit" "$2"
mv output output_cache/renderkit

./target/release/plotting_resvg
python3 src/models/plotting/py/numeric_line_single.py "Resvg" "$2"
mv output output_cache/resvg
