import os
import matplotlib.pyplot as plt
import pandas as pd

data = pd.read_csv("../../output/data/svg/primitives/time_cubic_curves.csv")

# Add total time column
data["total_time"] = data["init_time"] + data["tess_time"]
data = data.sort_values(by=["total_time"], ascending=False)

chart_labels = []
chart_init_means = []
chart_init_stds = []
chart_tess_means = []
chart_tess_stds = []
# Condense trials into std deviation and mean
for amount in data["amount"].unique():
    trials = data[data["amount"] == amount]
    amt_trials = len(trials)
    # Get means
    init_mean = round(trials["init_time"].mean() / 1000000, 3)  # nanosec to ms
    init_std = round(trials["init_time"].std() / 1000000, 3)  # nanosec to ms
    tess_mean = round(trials["tess_time"].mean() / 1000000, 3)  # nanosec to ms
    tess_std = round(trials["tess_time"].std() / 1000000, 3)  # nanosec to ms
    # Append chart data
    chart_labels.append(amount)
    chart_init_means.append(init_mean)
    chart_init_stds.append(init_std)
    chart_tess_means.append(tess_mean)
    chart_tess_stds.append(tess_std)

fig, ax = plt.subplots()

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
    "Tessellation time for cubic bezier curve primitives (" + str(amt_trials) + " trials)")
ax.yaxis.grid()
ax.legend()
plt.tight_layout()

if not os.path.exists('../../output/figs/svg/primitives'):
    os.makedirs('../../output/figs/svg/primitives')
fig.savefig("../../output/figs/svg/primitives/time_cubic_curves.png", dpi=500)
plt.show()
