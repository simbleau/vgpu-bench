rm -rf /home/simbleau/git/vgpu-bench/output*
mkdir /home/simbleau/git/vgpu-bench/output_cache

cd /home/simbleau/git/vgpu-bench
cargo build --release
for filename in ./assets/svg/examples/*.svg; do
	echo $filename
	./target/release/first_frame "$filename"
	cat ./output/Render-Kit/measurements.csv >> ./output/measurements_renderkit_total.csv
	cat ./output/Resvg/measurements.csv >> ./output/measurements_resvg_total.csv
	cat ./output/Pathfinder/measurements.csv >> ./output/measurements_pathfinder_total.csv
done
rm -rf ./output/Render-Kit/
rm -rf ./output/Resvg/
rm -rf ./output/Pathfinder/
mv output output_cache
