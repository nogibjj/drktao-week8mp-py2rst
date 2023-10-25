import pandas as pd
import statistics
import time
from memory_profiler import memory_usage


def denirostats(file):
    df = pd.read_csv(file)
    sumstats = pd.DataFrame(
        {
            "Mean Score": round(df.iloc[:, 1].mean(), 2),
            "Median Score": round(df.iloc[:, 1].median(), 2),
            "Standard Deviation of Scores": round(statistics.stdev(df.iloc[:, 1]), 2),
        },
        index=[0],
    )
    return sumstats


if __name__ == "__main__":
    start_time = time.time()

    print(denirostats("deniro.csv"))

    end_time = time.time()
    print(f"\nRuntime: {end_time - start_time:.6f} seconds")

    mem_usage = memory_usage((denirostats, ('deniro.csv',)))
    print(f"Memory Usage: {max(mem_usage):.6f} MiB")
