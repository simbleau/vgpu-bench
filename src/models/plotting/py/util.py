import os
import pandas as pd
import numpy as np
import matplotlib.pyplot as plt
import io
import csv


def save(path, name, type):
    if not os.path.exists(path):
        os.makedirs(path)
    plt.savefig(
        f"{path}/{name}.{type}", dpi=500)
    pass


def show(plot):
    plt.show()


def ns_to_ms(nanos, rounding=True, decimals=3):
    """
    Converts nanoseconds to milliseconds, with optional rounding.

    :param nanos: A numeric value of nano seconds
    :param rounding: Whether to apply rounding (default is 3 decimal places)
    :param decimals: The amount of decimal places to round to
    :return: returns milliseconds
    """
    if rounding:
        return round(nanos / 1000000, decimals)
    else:
        return nanos / 1000000


def dataframe(columns, rows, sort=False, by=None, ascending=True):
    buf = io.StringIO(rows)
    df = pd.read_csv(buf, sep=",", names=columns)
    if sort:
        df = df.sort_values(by=by, ascending=ascending)
    return df


if __name__ == "__main__":
    columns = ['value']
    data = "true\nfalse"
    df = dataframe(columns, data)
