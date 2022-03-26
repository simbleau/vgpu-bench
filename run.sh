rm -rf /home/simbleau/git/vgpu-bench/output*
mkdir /home/simbleau/git/vgpu-bench/output_cache

cd /home/simbleau/git/vgpu-bench
cargo build --release
for filename in ./assets/svg/examples/*.svg; do
	echo $filename
	./target/release/plotting_pathfinder "$filename"
	cat ./output/measurements.csv >> ./output/measurements_total.csv
done
mv -f ./output/measurements_total.csv ./output/measurements.csv
python3 src/models/plotting/py/numeric_line.py "Pathfinder"
mv output output_cache/pathfinder

./target/release/plotting_renderkit
python3 src/models/plotting/py/numeric_line.py "Render-Kit"
mv output output_cache/renderkit

./target/release/plotting_resvg
python3 src/models/plotting/py/numeric_line.py "Resvg"
mv output output_cache/resvg
