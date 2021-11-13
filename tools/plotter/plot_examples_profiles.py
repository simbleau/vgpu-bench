from matplotlib.offsetbox import OffsetImage, AnnotationBbox
import os
import matplotlib.pyplot as plt
import numpy as np
import pandas as pd
import cairosvg
import os

# Get CLI args
import sys

print(sys.argv)
if len(sys.argv) != 4:
    print("Usage: <input_file> <output_dir> <output_name>")
    exit(1)

INPUT_CSV = sys.argv[1]
OUTPUT_DIR = sys.argv[2]
OUTPUT_NAME = sys.argv[3]
OUTPUT_TYPE = "png"

#INPUT_DATA_DIR = "../.."
#INPUT_CSV = "../../output/data/svg/examples/profiles.csv"
#OUTPUT_DIR = "../../output/figs/svg/examples"
#OUTPUT_NAME = "profiles"
#OUTPUT_TYPE = "png"

# Get Data
data = pd.read_csv(INPUT_CSV)

# Sort by total triangles in order
data = data.sort_values(by=["triangles"], ascending=True)
# Filter rows
backends = data['tessellator'].unique()

for backend in backends:
    fig, ax = plt.subplots()

    # Get rows for this primitive
    rows = data[data["tessellator"] == backend]
    num_rows = len(rows)

    # Plot data
    filenames = rows["filename"]
    basenames = [os.path.basename(f) for f in filenames]
    triangles = rows["triangles"]
    points = ax.scatter(basenames, triangles)

    # Draw icons
    for filename, num_triangles, (x, y) in zip(filenames, triangles, points.get_offsets().data):
        cairosvg.svg2png(url=filename, output_width=200,
                         output_height=200, write_to="temp.png")
        image = plt.imread("temp.png")
        os.remove("temp.png")
        # Draw annotation
        im = OffsetImage(image, zoom=0.1)
        x, y = np.atleast_1d(x, y)
        for x0, y0 in zip(x, y):
            ab = AnnotationBbox(im, (x0, y0), xycoords='data', frameon=False)
            ax.add_artist(ab)
            ax.annotate(num_triangles, (x, y), xytext=(0, 10), textcoords='offset points', ha='center', va='bottom',
                        bbox=dict(boxstyle='round,pad=0.2', fc='black', alpha=0.2))
        ax.update_datalim(np.column_stack([x, y]))
        ax.autoscale()

    # Dress plot
    plt.xticks(basenames, rotation='vertical')
    ax.set_xlabel("Files")
    ax.set_ylabel("Triangles")
    ax.set_title(
        f"Tessellation Triangle Output via {backend}")
    ax.yaxis.grid()
    plt.tight_layout()

    # Save plot
    if not os.path.exists(OUTPUT_DIR):
        os.makedirs(OUTPUT_DIR)
    fig.savefig(
        f"{OUTPUT_DIR}/{backend}_{OUTPUT_NAME}.{OUTPUT_TYPE}", dpi=500)
