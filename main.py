"""
ETL-Query with time and memory tracking
"""

import time
import psutil
from mylib.extract import extract
from mylib.transform_load import load
from mylib.query import create_data, read_data, delete_data, update_data


def performance(func, *args):
    """
    Measures the performance of a function by tracking time and memory usage.
    """
    process = psutil.Process()
    start_memory = process.memory_info().rss / 1024  # Memory in KB
    start_time = time.time()

    # Execute the argument input function
    result = func(*args)

    end_time = time.time()
    end_memory = process.memory_info().rss / 1024  # Memory in KB
    duration = end_time - start_time

    print(f"Execution Time: {duration * 1000:.2f}ms")
    print(f"Memory Usage Before: {start_memory:.2f} KB")
    print(f"Memory Usage After: {end_memory:.2f} KB")
    print(f"Memory Consumed: {end_memory - start_memory:.2f} KB\n")

    return result


def main():
    url = "https://raw.githubusercontent.com/fivethirtyeight/data/refs/heads/master/alcohol-consumption/drinks.csv"
    file_path = "data/drinks.csv"

    print("Performance of `extract` function:")
    performance(extract, url, file_path)

    print("Performance of `load` function:")
    performance(load, file_path)

    print("Performance of `create_data` function:")
    performance(create_data)

    print("Performance of `read_data` function:")
    performance(read_data)

    print("Performance of `update_data` function:")
    performance(update_data)

    print("Performance of `delete_data` function:")
    performance(delete_data)


if __name__ == "__main__":
    main()
