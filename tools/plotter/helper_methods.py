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
