import os
import matplotlib.pyplot as plt
import pandas as pd
import numpy as np
import helper_methods

INPUT_CSV = "../../output/data/svg/examples/naive_frametimes.csv"
OUTPUT_DIR = "../../output/figs/svg/examples/naive_frametimes"
OUTPUT_PREFIX = "frametimes_"
OUTPUT_TYPE = "png"

# Get data
data = pd.read_csv(INPUT_CSV)
# Sort by frames in order
data = data.sort_values(by=["frame"], ascending=True)
# Filter rows
filenames = data["filename"].unique()

for filename in filenames:
    # Make subplots
    fig, ax = plt.subplots()

    # Get rows for this file
    rows = data[data["filename"] == filename]
    num_rows = len(rows)

    # Make plot
    # Get chart data
    frame_times = []
    for frame in range(num_rows):
        frame_time_nanos = rows['frame_time'].values[frame]
        frame_time_ms = helper_methods.ns_to_ms(
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
    basename = os.path.basename(filename)
    ax.set_xlabel("Frame")
    ax.set_ylabel("Total time (ms)")
    ax.set_title(
        f"Continuous frame-times of {basename}, naive")
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
        f"{OUTPUT_DIR}/{OUTPUT_PREFIX}{basename}.{OUTPUT_TYPE}", dpi=500)
