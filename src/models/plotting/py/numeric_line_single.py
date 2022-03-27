import sys
import util
import matplotlib.pyplot as plt
import numpy as np

title_arg = sys.argv[1]
file_arg = sys.argv[2]

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

    p_ax.set_xlabel(x_label)
    p_ax.set_ylabel(y_label)

    if df.empty:
        return fig

    df[y_col] = df[y_col].apply(util.ns_to_ms)

    # Get plot classifications
    if plot_by is not None:
        plot_items = df[plot_by].unique()
        for plot_item in sorted(plot_items):
            rows = df.loc[df[plot_by] == plot_item]
            x_values = rows[x_col].unique()
            xy_values = []
            decimals = 4
            for x_value in x_values:
                y_rows = rows.loc[rows[x_col] == x_value]
                y_value = round(np.mean(y_rows[y_col]), decimals)
                xy_values.append(y_value)
                # Plot frame times
            print(f"plot item: {plot_item}")#\nx: {x_values}\ny: {xy_values}")
            p_ax.plot(x_values, xy_values, label=plot_item,
                      linewidth=3)
            x1,x2,y1,y2 = plt.axis()
            if np.amin(xy_values) < y1:
                p_ax.axis((x1,x2,np.amin(xy_values),y2))
            if np.amax(xy_values) > y2:
                p_ax.axis((x1,x2,y1,np.amax(xy_values)))
    else:
        rows = df
        x_values = rows[x_col].unique()
        xy_values = []
        decimals = 4
        for x_value in x_values:
            y_rows = rows.loc[rows[x_col] == x_value]
            y_value = round(np.mean(y_rows[y_col]), decimals)
            xy_values.append(y_value)
            # Plot frame times
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

    p_ax.margins(.05, .05, tight=True)
    p_ax.legend(loc='best')
    return fig


if __name__ == "__main__":
    columns = ["filename", "frame", "frametime_ns"]
    txt_file = open("/home/simbleau/git/vgpu-bench/output/measurements.csv", "r")
    rows = txt_file.read()
    txt_file.close()

    df = util.dataframe(columns, rows, sort=True, by=columns[1], ascending=True)
    print(df)
    show_stats = True
    show_stats_table = True
    plot = plot(df, columns[1], columns[2], f"{title_arg} Frametimes, {file_arg}",
                "Frame", "Time (ms)", plot_by=None, show_stats=show_stats, show_stats_table=show_stats_table)
    plt.grid(True, axis='y')
    #plt.autoscale(enable=True, axis='both', tight=True)
    util.save("/home/simbleau/git/vgpu-bench/output", "plot", "svg")
    #util.show(plot)
