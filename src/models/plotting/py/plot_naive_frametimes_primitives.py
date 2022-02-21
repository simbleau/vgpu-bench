import os
import matplotlib.pyplot as plt
import pandas as pd
import numpy as np
import models.plotting.py.util as util

# Get CLI args
import sys
if len(sys.argv) != 4:
    print("Usage: <input_file> <output_dir> <output_name>")
    exit(1)
INPUT_CSV = sys.argv[1]
OUTPUT_DIR = sys.argv[2]
OUTPUT_NAME = sys.argv[3]
OUTPUT_TYPE = "png"

# Get data
data = pd.read_csv(INPUT_CSV)
# Sort by frames in order
data = data.sort_values(by=["frame"], ascending=True)
# Filter rows
primitives = data['primitive'].unique()

for primitive in primitives:
    # Make subplots
    fig, ax = plt.subplots()

    # Get rows for this primitive
    rows = data[data["primitive"] == primitive]
    num_rows = len(rows)

    # Make plot
    # Get chart data
    frame_times = []
    for frame in range(num_rows):
        frame_time_nanos = rows['frame_time'].values[frame]
        frame_time_ms = util.ns_to_ms(
            frame_time_nanos, rounding=False)
        frame_times.append(frame_time_ms)

    # Get table data
    decimals = 4
    min_frametime = round(np.amin(frame_times), decimals)
    max_frametime = round(np.amax(frame_times), decimals)
    med_frametime = round(np.median(frame_times), decimals)
    mean_frametime = round(np.mean(frame_times), decimals)
    std_dev_frametime = round(np.std(frame_times), decimals)

    # Plot frame times
    line = ax.plot(rows["frame"].unique(), frame_times,
                   linewidth=3, alpha=0.5, color="grey")
    # Plot lowest-line
    ax.axhline(y=min_frametime, color='blue', linestyle='--')

    # Dress plot
    ax.set_xlabel("Frame")
    ax.set_ylabel("Total time (ms)")
    ax.set_title(
        f"Continuous frame-times of {primitive}, naive")
    ax.yaxis.grid()
    plt.tight_layout()

    # Make table
    table_vals = [['Domain (y)', f"{{ y | {min_frametime} ≤ y ≤ {max_frametime} }} ms"],
                  ['Median (ỹ)', f"{med_frametime} ms"],
                  ['Mean (ȳ)', f"{mean_frametime} ms"],
                  ['Std. Deviation (σ)', f"{std_dev_frametime} ms"]]

    # Make room for table
    space = 0.35  # Percent of area used for table
    fig.subplots_adjust(bottom=space)
    # Add table
    the_table = plt.table(cellText=table_vals,
                          colWidths=[0.4, 0.6],
                          bbox=[0.1, -space * 1.6, 0.8, space])

    # Save plot
    if not os.path.exists(OUTPUT_DIR):
        os.makedirs(OUTPUT_DIR)
    fig.savefig(
        f"{OUTPUT_DIR}/{OUTPUT_NAME}_{primitive}.{OUTPUT_TYPE}", dpi=500)
