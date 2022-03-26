rm -rf /home/simbleau/git/vgpu-bench/output*

cd /home/simbleau/git/vgpu-bench
cargo build --release
for filename in ./assets/svg/examples/*.svg; do
	echo $filename
	./target/release/plotting_pathfinder "$filename"
	cat ./output/measurements.csv >> ./output/measurements_total.csv
done
mv -f ./output/measurements_total.csv ./output/measurements.csv
python3 src/models/plotting/py/numeric_line.py
mv output output_pathfinder

cargo run --release --example plotting_naive
python3 src/models/plotting/py/numeric_line.py
mv output output_naive

cargo run --release --example plotting_resvg
python3 src/models/plotting/py/numeric_line.py
mv output output_resvg
