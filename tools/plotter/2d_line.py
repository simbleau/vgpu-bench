import os
import matplotlib.pyplot as plt
import pandas as pd


def plot(csv, x, y, xlabel, ylabel, title):
    data = pd.read_csv(csv)
    dx = data[x]
    dy = data[y]

    fig, ax = plt.subplots()
    ax.plot(dx, dy)

    ax.set(xlabel=xlabel, ylabel=ylabel,
           title=title)
    ax.grid()
    return fig


if __name__ == "__main__":
    plot = plot("output/data/test.csv", "AAPL_y",
                "AAPL_y", "X label", "Y Label", "Title")
    if not os.path.exists('output/figs'):
        os.makedirs('output/figs')
    # fig.savefig("output/figs/test.png")
    plt.show()
