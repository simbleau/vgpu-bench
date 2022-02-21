import matplotlib.pyplot as plt


def plot(df, title, t_label, f_label):
    labels = t_label, f_label

    t_rows = len(df.loc[df['value'] == True])
    f_rows = len(df.loc[df['value'] == False])

    sizes = [t_rows, f_rows]

    if t_rows > f_rows:
        explode = (0.1, 0)
    elif t_rows < f_rows:
        explode = (0, 0.1)
    else:
        explode = (0, 0)

    fig = plt.figure()
    axs = fig.add_subplot(1, 1, 1)
    axs.pie(sizes, explode=explode, labels=labels, autopct='%1.1f%%',
            startangle=90)
    # Equal aspect ratio ensures that pie is drawn as a circle.
    axs.axis('equal')

    axs.legend()
    fig.suptitle(title, fontweight="bold")

    return fig
