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

    return result, duration, start_memory, end_memory


def main():
    url = "https://raw.githubusercontent.com/fivethirtyeight/data/refs/heads/master/alcohol-consumption/drinks.csv"
    file_path = "data/drinks.csv"

    # Initialize total time and memory variables
    total_duration = 0
    total_start_memory = (
        psutil.Process().memory_info().rss / 1024
    )  # Initial memory in KB

    print("Performance of `extract` function:")
    _, duration, _, _ = performance(extract, url, file_path)
    total_duration += duration

    print("Performance of `load` function:")
    _, duration, _, _ = performance(load, file_path)
    total_duration += duration

    print("Performance of `create_data` function:")
    _, duration, _, _ = performance(create_data)
    total_duration += duration

    print("Performance of `read_data` function:")
    _, duration, _, _ = performance(read_data)
    total_duration += duration

    print("Performance of `update_data` function:")
    _, duration, _, _ = performance(update_data)
    total_duration += duration

    print("Performance of `delete_data` function:")
    _, duration, _, _ = performance(delete_data)
    total_duration += duration

    # Final memory usage
    total_end_memory = psutil.Process().memory_info().rss / 1024  # Final memory in KB

    # Calculate and print total performance
    print("Total Performance Summary:")
    print(f"Total Execution Time: {total_duration * 1000:.2f}ms")
    print(f"Initial Memory Usage: {total_start_memory:.2f} KB")
    print(f"Final Memory Usage: {total_end_memory:.2f} KB")
    print(f"Total Memory Consumed: {total_end_memory - total_start_memory:.2f} KB")


if __name__ == "__main__":
    main()
