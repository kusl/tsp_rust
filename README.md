Traveling Salesman Problem in Rust









Demonstration of the problem: 
```
10 nodes
=== Solution Found ===
Best path: [0, 4, 8, 9, 7, 1, 3, 5, 2, 6]
Route: 0 -> 4 -> 8 -> 9 -> 7 -> 1 -> 3 -> 5 -> 2 -> 6 -> 0
Total distance: 394.17
Time taken: 0.035 seconds
12 nodes
=== Solution Found ===
Best path: [0, 6, 2, 5, 3, 1, 7, 10, 9, 11, 8, 4]
Route: 0 -> 6 -> 2 -> 5 -> 3 -> 1 -> 7 -> 10 -> 9 -> 11 -> 8 -> 4 -> 0
Total distance: 403.69
Time taken: 4.415 seconds
13 nodes 
=== Solution Found ===
Best path: [0, 4, 8, 11, 12, 10, 9, 7, 1, 3, 5, 2, 6]
Route: 0 -> 4 -> 8 -> 11 -> 12 -> 10 -> 9 -> 7 -> 1 -> 3 -> 5 -> 2 -> 6 -> 0
Total distance: 403.93
Time taken: 55.957 seconds
14 nodes 
=== Solution Found ===
Best path: [0, 6, 2, 13, 5, 3, 1, 7, 9, 10, 12, 11, 8, 4]
Route: 0 -> 6 -> 2 -> 13 -> 5 -> 3 -> 1 -> 7 -> 9 -> 10 -> 12 -> 11 -> 8 -> 4 -> 0
Total distance: 424.12
Time taken: 770.787 seconds
```



Remember, a better algorithm / processor / computer can help but only so much. 
Below we have the same code running on a different computer with a more optimized algorithm: 

```bash 
kushal@kusfedora2024:~/src/rustlang/tsp_rust$ time cargo run --release 13 42
    Finished `release` profile [optimized] target(s) in 0.00s
     Running `target/release/tsp_solver 13 42`
=== Traveling Salesman Problem Solver ===
Cities: 13
Seed: 42
Available CPU threads: 16

City Positions:
  City 0: (0.00, 0.01)
  City 1: (78.56, 60.47)
  City 2: (81.05, 1.19)
  City 3: (70.41, 37.51)
  City 4: (28.28, 37.96)
  City 5: (63.30, 31.49)
  City 6: (41.54, 8.58)
  City 7: (99.72, 80.97)
  City 8: (29.60, 45.77)
  City 9: (0.91, 96.23)
  City 10: (24.66, 76.52)
  City 11: (24.11, 45.28)
  City 12: (22.73, 63.63)

=== Multi-threaded Solution (16 threads) ===
Best path: [0, 6, 2, 5, 3, 1, 7, 9, 10, 12, 11, 8, 4]
Route: 0 -> 6 -> 2 -> 5 -> 3 -> 1 -> 7 -> 9 -> 10 -> 12 -> 11 -> 8 -> 4 -> 0
Total distance: 403.93
Time taken: 4.400 seconds

=== Optimized Solution (Distance Matrix + Branch & Bound + Bitmask DP) ===
Best path: [0, 6, 2, 5, 3, 1, 7, 9, 10, 12, 11, 8, 4]
Route: 0 -> 6 -> 2 -> 5 -> 3 -> 1 -> 7 -> 9 -> 10 -> 12 -> 11 -> 8 -> 4 -> 0
Total distance: 403.93
Time taken: 0.004 seconds

=== Single-threaded Solution ===
Best path: [0, 4, 8, 11, 12, 10, 9, 7, 1, 3, 5, 2, 6]
Route: 0 -> 4 -> 8 -> 11 -> 12 -> 10 -> 9 -> 7 -> 1 -> 3 -> 5 -> 2 -> 6 -> 0
Total distance: 403.93
Time taken: 32.522 seconds

=== Performance Summary ===
Implementation order: ["Multi", "Optimized", "Single"]

Performance Ranking:
  1. Optimized: 0.004s (distance: 403.93, baselinex slower)
  2. Multi-threaded: 4.400s (distance: 403.93, 1243.45x slower)
  3. Single-threaded: 32.522s (distance: 403.93, 9190.07x slower)

✓ All implementations found the same optimal solution!

real	0m36.965s
user	1m20.306s
sys	0m0.018s
kushal@kusfedora2024:~/src/rustlang/tsp_rust$ time cargo run --release 14 42
    Finished `release` profile [optimized] target(s) in 0.00s
     Running `target/release/tsp_solver 14 42`
=== Traveling Salesman Problem Solver ===
Cities: 14
Seed: 42
Available CPU threads: 16

City Positions:
  City 0: (0.00, 0.01)
  City 1: (78.56, 60.47)
  City 2: (81.05, 1.19)
  City 3: (70.41, 37.51)
  City 4: (28.28, 37.96)
  City 5: (63.30, 31.49)
  City 6: (41.54, 8.58)
  City 7: (99.72, 80.97)
  City 8: (29.60, 45.77)
  City 9: (0.91, 96.23)
  City 10: (24.66, 76.52)
  City 11: (24.11, 45.28)
  City 12: (22.73, 63.63)
  City 13: (94.25, 18.45)

=== Multi-threaded Solution (16 threads) ===
Best path: [0, 6, 2, 13, 5, 3, 1, 7, 9, 10, 12, 11, 8, 4]
Route: 0 -> 6 -> 2 -> 13 -> 5 -> 3 -> 1 -> 7 -> 9 -> 10 -> 12 -> 11 -> 8 -> 4 -> 0
Total distance: 424.12
Time taken: 59.084 seconds

=== Optimized Solution (Distance Matrix + Branch & Bound + Bitmask DP) ===
Best path: [0, 6, 2, 13, 5, 3, 1, 7, 9, 10, 12, 11, 8, 4]
Route: 0 -> 6 -> 2 -> 13 -> 5 -> 3 -> 1 -> 7 -> 9 -> 10 -> 12 -> 11 -> 8 -> 4 -> 0
Total distance: 424.12
Time taken: 0.008 seconds

=== Single-threaded Solution ===
Best path: [0, 6, 2, 13, 5, 3, 1, 7, 9, 10, 12, 11, 8, 4]
Route: 0 -> 6 -> 2 -> 13 -> 5 -> 3 -> 1 -> 7 -> 9 -> 10 -> 12 -> 11 -> 8 -> 4 -> 0
Total distance: 424.12
Time taken: 432.124 seconds

=== Performance Summary ===
Implementation order: ["Multi", "Optimized", "Single"]

Performance Ranking:
  1. Optimized: 0.008s (distance: 424.12, baselinex slower)
  2. Multi-threaded: 59.084s (distance: 424.12, 7580.89x slower)
  3. Single-threaded: 432.124s (distance: 424.12, 55444.74x slower)

✓ All implementations found the same optimal solution!

real	8m11.254s
user	19m15.337s
sys	0m0.151s
```






**⚠️ AI Disclosure**: This project includes code generated with assistance from Large Language Models (LLMs) including Claude. All generated code has been reviewed, tested, and validated. Use at your own discretion.
