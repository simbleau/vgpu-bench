import os
import matplotlib.pyplot as plt
import pandas as pd
import helper_methods

INPUT_CSV = "../../output/data/svg/primitives/tessellation.csv"
OUTPUT_DIR = "../../output/figs/svg/primitives/"
OUTPUT_PREFIX = "tessellation_"
OUTPUT_TYPE = "png"

# Get data
data = pd.read_csv(INPUT_CSV)
# Add total time column
data["total_time"] = data["init_time"] + data["tess_time"]
# Sort by total time
data = data.sort_values(by=["total_time"], ascending=False)
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
    chart_labels = []
    chart_init_means = []
    chart_init_stds = []
    chart_tess_means = []
    chart_tess_stds = []
    # Condense trials into std deviation and mean
    amounts = rows["amount"].unique()
    for amount in amounts:
        amt_rows = rows[rows["amount"] == amount]
        trials = len(amt_rows)
        # Get data
        init_time_mean = amt_rows["init_time"].mean()
        init_time_std = amt_rows["init_time"].std()
        tess_time_mean = amt_rows["tess_time"].mean()
        tess_time_std = amt_rows["tess_time"].std()
        # Append chart data
        chart_labels.append(amount)
        chart_init_means.append(helper_methods.ns_to_ms(init_time_mean))
        chart_init_stds.append(helper_methods.ns_to_ms(init_time_std))
        chart_tess_means.append(helper_methods.ns_to_ms(tess_time_mean))
        chart_tess_stds.append(helper_methods.ns_to_ms(tess_time_std))

    # Plot data
    width = 700  # the width of the bars
    bar1 = ax.bar(chart_labels, chart_init_means, width, yerr=chart_init_stds,
                  alpha=0.5, ecolor='black', capsize=5, label='Initialization')
    bar2 = ax.bar(chart_labels, chart_tess_means, width, yerr=chart_tess_stds,
                  alpha=0.5, ecolor='black', capsize=5, label='Tessellation', bottom=chart_init_means)
    ax.bar_label(bar1, fmt='%0.0f', label_type='center')
    ax.bar_label(bar2, fmt='%0.0f', label_type='center')

    # Dress plot
    plt.xticks(chart_labels, rotation='vertical')
    ax.set_xlabel("Amount")
    ax.set_ylabel("Total time (ms)")
    ax.set_title(
        f"Tessellation time for {primitive} ({trials} trials)")
    ax.yaxis.grid()
    ax.legend()
    plt.tight_layout()

    # Save plot
    if not os.path.exists(OUTPUT_DIR):
        os.makedirs(OUTPUT_DIR)
    fig.savefig(
        f"{OUTPUT_DIR}/{OUTPUT_PREFIX}{primitive}.{OUTPUT_TYPE}", dpi=500)
