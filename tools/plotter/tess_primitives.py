import os
import matplotlib.pyplot as plt
import pandas as pd

width = 0.35       # the width of the bars: can also be len(x) sequence

data = pd.read_csv("../../output/data/tess_primitives.csv")
data = data.sort_values(by=["prep_time"], ascending=False)
labels = data["filename"]
prep_times = data["prep_time"]
tess_times = data["tess_time"]

fig, ax = plt.subplots()

# Plot data
ax.bar(labels, prep_times, width,  label='Pre-processing')
ax.bar(labels, tess_times, width, label='Tessellation', bottom=prep_times)
plt.xticks(labels, rotation='vertical')

ax.set_xlabel("Files")
ax.set_ylabel("Time (ms)")
ax.set_title("Tessellation time for primitives")

# ax.grid()
ax.legend()

# Set sizing
plt.gcf().set_size_inches(10, 10)
plt.subplots_adjust(bottom=0.20)

if not os.path.exists('../../output/figs'):
    os.makedirs('../../output/figs')
fig.savefig("../../output/figs/test.png", dpi=200)
plt.show()
