from matplotlib.offsetbox import OffsetImage, AnnotationBbox
import os
import matplotlib.pyplot as plt
import numpy as np
import pandas as pd
import cairosvg
import os


data = pd.read_csv("../../output/data/svg/examples/profiles.csv")
# Add total triangles column
data["triangles"] = data["indices"] / 3  # 1 triangle = 3 indices
data = data.sort_values(by=["triangles"], ascending=True)

# Get data
labels = data["filename"]
triangles = data["triangles"]

fig, ax = plt.subplots()

# Plot data
points = ax.scatter(labels, triangles)
for path, (x, y) in zip(labels, points.get_offsets().data):
    tris = int(data[data["filename"] == path]['triangles'].values[0])
    asset_path = os.path.join("../../assets/svg/examples/", path)
    cairosvg.svg2png(url=asset_path, output_width=200,
                     output_height=200, write_to="temp.png")
    image = plt.imread("temp.png")
    os.remove("temp.png")
    im = OffsetImage(image, zoom=0.1)
    x, y = np.atleast_1d(x, y)
    for x0, y0 in zip(x, y):
        ab = AnnotationBbox(im, (x0, y0), xycoords='data', frameon=False)
        ax.add_artist(ab)
        ax.annotate(tris, (x, y), xytext=(0, 10), textcoords='offset points', ha='center', va='bottom',
                    bbox=dict(boxstyle='round,pad=0.2', fc='black', alpha=0.2))
    ax.update_datalim(np.column_stack([x, y]))
    ax.autoscale()

# Dress plot
plt.xticks(labels, rotation='vertical')
ax.set_xlabel("Files")
ax.set_ylabel("Triangles")
ax.set_title(
    "Tessellation Triangle Output")
ax.yaxis.grid()
plt.tight_layout()

if not os.path.exists('../../output/figs/svg/examples'):
    os.makedirs('../../output/figs/svg/examples')
fig.savefig("../../output/figs/svg/examples/profiles.png", dpi=500)
plt.show()
