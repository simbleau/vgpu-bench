import os
import matplotlib.pyplot as plt
import pandas as pd

data = pd.read_csv("../../output/data/svg/primitives/time_triangles.csv")

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
ax.bar(chart_labels, chart_init_means, yerr=chart_init_stds, align='center',
       alpha=0.5, ecolor='black', capsize=5,  label='Initialization')
ax.bar(chart_labels, chart_tess_means, yerr=chart_tess_stds, align='center',
       alpha=0.5, ecolor='black', capsize=5, label='Tessellation', bottom=chart_init_means)
plt.xticks(chart_labels, rotation='vertical')

ax.set_xlabel("Amount")
ax.set_ylabel("Time (ms)")
ax.set_title(
    "Tessellation time for triangle primitives (" + str(amt_trials) + " trials)")

ax.grid()
ax.legend()

if not os.path.exists('../../output/figs/svg/primitives'):
    os.makedirs('../../output/figs/svg/primitives')
fig.savefig("../../output/figs/svg/primitives/time_triangles.png", dpi=200)
plt.show()
