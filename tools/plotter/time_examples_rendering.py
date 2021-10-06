import os
import matplotlib.pyplot as plt
import pandas as pd
import numpy as np
from scipy.interpolate import UnivariateSpline


data = pd.read_csv("../../output/data/svg/examples/renders.csv")

# Sort by frames in order
data = data.sort_values(by=["frame"], ascending=True)

chart_labels = data["frame"].unique()
# Plot data
for filename in data["filename"].unique():
    # Make plot
    fig, ax = plt.subplots()

    trials = data[data["filename"] == filename]
    amt_trials = len(trials)
    # Plot frametimes
    ft = round(trials["frame_time"] / 1000000, 3)  # nanosec to ms
    # Append chart data
    chart_frame_times = []
    for trial in range(len(trials)):
        frame_time_nanos = trials['frame_time'].values[trial]
        frame_time_ms = round(frame_time_nanos / 1000000, 3)
        chart_frame_times.append(frame_time_ms)
    line = ax.plot(chart_labels, chart_frame_times,
                   linewidth=3, alpha=0.5, color="grey")

    # Calculate best fit
    spline = UnivariateSpline(
        chart_labels, chart_frame_times, s=int(amt_trials / 5))
    xs = np.linspace(chart_labels.min(), chart_labels.max(),
                     amt_trials)
    ys = spline(xs)
    plt.plot(xs, ys, "--", color="blue")

    # Dress plot
    ax.set_xlabel("Frame")
    ax.set_ylabel("Total time (ms)")
    ax.set_title(
        f"Continuous frame-times of {filename}, flattened")
    ax.yaxis.grid()
    plt.tight_layout()

    # Save plot
    if not os.path.exists('../../output/figs/svg/examples/rendering'):
        os.makedirs('../../output/figs/svg/examples/rendering')
    fig.savefig(
        f"../../output/figs/svg/examples/rendering/frametime_{filename}.png", dpi=500)
