# Performance Comparison Report: Rust vs. Python ETL-Query Scripts

## Overview

This report compares the performance of ETL (Extract, Transform, Load) processes implemented in Rust and Python. Both implementations perform the same tasks of data extraction from a URL, loading the data into an SQLite database, and providing CRUD (Create, Read, Update, Delete) functionalities. The comparison focuses on execution time and memory usage, two critical factors in evaluating the efficiency of programming languages for data processing tasks.

## Terminal Output

Below is rust's output:

```
URL: https://raw.githubusercontent.com/fivethirtyeight/data/refs/heads/master/alcohol-consumption/drinks.csv
File path: data/drinks.csv
File successfully downloaded to data/drinks.csv
Extract completed in: 197.20ms
Memory used during Extract: 4407 KB
Current working directory: Ok("/Users/zhangsu/Desktop/Su_Zhang_IDS706_Individual_Project_2")
Load completed in: 74.80ms
Memory used during Load: 246 KB
create: ("Country1", 90, 20, 16, 4.5)
Create completed in: 25.00ms
Memory used during Create: 16 KB
Read completed in: 18.88ms
Memory used during Read: 0 KB
rows updated: 1
Update completed in: 19.65ms
Memory used during Update: 115 KB
rows deleted: 1
Delete completed in: 17.84ms
Memory used during Delete: 16 KB
Total Process completed in: 389.87ms
Total Memory used: 8798 KB
```

Below is python's output:

```
Performance of `extract` function:
File successfully downloaded to data/drinks.csv
Execution Time: 124.16ms
Memory Usage Before: 29984.00 KB
Memory Usage After: 34016.00 KB
Memory Consumed: 4032.00 KB

Performance of `load` function:
/Users/zhangsu/Desktop/Su_Zhang_IDS706_Individual_Project_2
Execution Time: 5.17ms
Memory Usage Before: 34016.00 KB
Memory Usage After: 34832.00 KB
Memory Consumed: 816.00 KB

Performance of `create_data` function:
create: [('Country1', 90, 20, 16, 4.5)]
Execution Time: 0.55ms
Memory Usage Before: 34832.00 KB
Memory Usage After: 35040.00 KB
Memory Consumed: 208.00 KB

Performance of `read_data` function:
Execution Time: 0.14ms
Memory Usage Before: 35040.00 KB
Memory Usage After: 35040.00 KB
Memory Consumed: 0.00 KB

Performance of `update_data` function:
rows updated: 1
Execution Time: 0.52ms
Memory Usage Before: 35040.00 KB
Memory Usage After: 35040.00 KB
Memory Consumed: 0.00 KB

Performance of `delete_data` function:
rows deleted: 1
Execution Time: 0.61ms
Memory Usage Before: 35040.00 KB
Memory Usage After: 35136.00 KB
Memory Consumed: 96.00 KB

Total Performance Summary:
Total Execution Time: 131.15ms
Initial Memory Usage: 29968.00 KB
Final Memory Usage: 35136.00 KB
Total Memory Consumed: 5168.00 KB
```

### Execution Time

| Operation          | Python Execution Time | Rust Execution Time |
|---------------------|-----------------------|---------------------|
| **Extract**         | 173.66 ms             | 272.03 ms           |
| **Load**            | 5.32 ms               | 79.56 ms            |
| **Create**          | 0.50 ms               | 20.90 ms            |
| **Read**            | 0.17 ms               | 17.07 ms            |
| **Update**          | 0.51 ms               | 16.31 ms            |
| **Delete**          | 0.54 ms               | 16.91 ms            |
| **Total**           | 180.70 ms             | 465.48 ms           |

### Memory Usage

| Operation          | Python Memory Consumed | Rust Memory Consumed |
|---------------------|-------------------------|----------------------|
| **Extract**         | 4000.00 KB              | 4244 KB              |
| **Load**            | 800.00 KB               | 196 KB               |
| **Create**          | 176.00 KB               | 0 KB                 |
| **Read**            | 0.00 KB                 | 148 KB               |
| **Update**          | 0.00 KB                 | 0 KB                 |
| **Delete**          | 0.00 KB                 | 33 KB                |
| **Total**           | 4992.00 KB              | 8503 KB              |

## Analysis and Conclusion

- **Execution Time**: The total execution time for the Rust implementation (465.48 ms) was longer than the Python version (180.70 ms). This is likely because for rust program, I added command line binary, which takes longer to compile than python script. 
  
- **Memory Usage**: The Rust version consumed more memory overall (8503 KB) compared to the Python implementation (4992 KB). This difference may arise from Rust's management of data ownership, which can sometimes lead to higher memory usage during operations.

Overall, this comparison highlights the strengths and weaknesses of both languages in handling ETL processes. Python's ease of use and mature libraries allowed for faster execution, while Rust's performance characteristics could provide advantages in more complex scenarios or when optimization is more critical.