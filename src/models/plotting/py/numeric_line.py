import util
import matplotlib.pyplot as plt
import numpy as np


def plot(df, x_col, y_col, title, x_label, y_label, plot_by=None, show_stats=False, show_stats_table=False):
    if show_stats_table:
        fig = plt.figure()
        p_ax = plt.subplot2grid((3, 1), (0, 0), rowspan=2)
        t_ax = plt.subplot2grid((3, 1), (2, 0))
        t_ax.axis('tight')
        t_ax.axis('off')
    else:
        fig, p_ax = plt.subplots()
    fig.suptitle(title, fontweight="bold")

    p_ax.axis('equal')
    p_ax.set_xlabel(x_label)
    p_ax.set_ylabel(y_label)
    plt.tight_layout()

    if df.empty:
        return fig

    # Get plot classifications
    if plot_by is not None:
        plot_items = df[plot_by].unique()
        for plot_item in plot_items:
            rows = df.loc[df[plot_by] == plot_item]
            x_values = rows[x_col].unique()
            xy_values = []
            decimals = 4
            for x_value in x_values:
                y_rows = rows.loc[rows[x_col] == x_value]
                y_value = round(np.mean(y_rows[y_col]), decimals)
                xy_values.append(y_value)
                # Plot frame times
            p_ax.plot(x_values, xy_values, label=plot_item,
                      linewidth=3)
    else:
        rows = df
        x_values = rows[x_col].unique()
        xy_values = []
        decimals = 4
        for x_value in x_values:
            y_rows = rows.loc[rows[x_col] == x_value]
            y_value = round(np.mean(y_rows[y_col]), decimals)
            xy_values.append(y_value)
        p_ax.plot(x_values, xy_values,
                  linewidth=3)

    # Get table data
    y_values = df[y_col]
    min = round(np.amin(y_values), decimals)
    max = round(np.amax(y_values), decimals)
    med = round(np.median(y_values), decimals)
    mean = round(np.mean(y_values), decimals)
    std_dev = round(np.std(y_values), decimals)

    # Plot lowest-line
    if show_stats:
        p_ax.axhline(y=min, color='blue', linestyle='--', alpha=0.5)
        p_ax.axhline(y=max, color='red', linestyle='--', alpha=0.5)

    # Make table
    if show_stats_table:
        table_vals = [['Domain (y)', f"{{ y | {min} ≤ y ≤ {max} }}"],
                      ['Median (ỹ)', f"{med}"],
                      ['Mean (ȳ)', f"{mean}"],
                      ['Std. Deviation (σ)', f"{std_dev}"]]
        t_ax.table(
            cellText=table_vals,
            colWidths=[0.4, 0.6],
            loc='center',
        )

    p_ax.legend(loc='best')
    return fig
